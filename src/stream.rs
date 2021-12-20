use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::rpc::MissionRpc;
use futures_util::stream::StreamExt;
use futures_util::TryFutureExt;
use stubs::coalition::v0::coalition_service_server::CoalitionService;
use stubs::coalition::v0::GetGroupsRequest;
use stubs::common;
use stubs::common::v0::{Coalition, Position, Unit};
use stubs::group::v0::group_service_server::GroupService;
use stubs::group::v0::GetUnitsRequest;
use stubs::mission::v0::stream_events_response::Event;
use stubs::mission::v0::stream_events_response::{BirthEvent, DeadEvent};
use stubs::mission::v0::stream_units_response::UnitGone;
use stubs::mission::v0::stream_units_response::Update;
use stubs::mission::v0::StreamUnitsRequest;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use tokio::time::MissedTickBehavior;
use tonic::{Code, Request, Status};

/// Stream unit updates.
pub async fn stream_units(
    opts: StreamUnitsRequest,
    rpc: MissionRpc,
    tx: Sender<Result<Update, Status>>,
) -> Result<(), Error> {
    // initialize the state for the current units stream instance
    let poll_rate = opts.poll_rate.unwrap_or(5);
    let max_backoff = Duration::from_secs(opts.max_backoff.unwrap_or(30).max(poll_rate) as u64);
    let poll_rate = Duration::from_secs(poll_rate as u64);
    let mut state = State {
        units: HashMap::new(),
        ctx: Context {
            rpc,
            tx,
            poll_rate,
            max_backoff,
        },
    };

    // initial full-sync of all current units inside of the mission
    let groups = futures_util::future::try_join_all(
        [Coalition::Blue, Coalition::Red, Coalition::Neutral].map(|coalition| {
            state
                .ctx
                .rpc
                .get_groups(Request::new(GetGroupsRequest {
                    coalition: coalition.into(),
                    category: None,
                }))
                .map_ok(|res| res.into_inner().groups)
        }),
    )
    .await?
    .into_iter()
    .flatten();

    let group_units = futures_util::future::try_join_all(groups.into_iter().map(|group| {
        state
            .ctx
            .rpc
            .get_units(Request::new(GetUnitsRequest {
                group_name: group.name,
                active: Some(true),
            }))
            .map_ok(|res| res.into_inner().units)
    }))
    .await?;

    for units in group_units {
        state.units.extend(
            units
                .into_iter()
                .map(|unit| (unit.name.clone(), UnitState::new(unit))),
        )
    }

    // send out all initial units
    for unit_state in state.units.values() {
        state
            .ctx
            .tx
            .send(Ok(Update::Unit(unit_state.unit.clone())))
            .await?;
    }

    // initiate an event stream used to update the state
    let mut events = state.ctx.rpc.events().await;

    // create an interval used to poll the mission for updates
    let mut interval = tokio::time::interval(poll_rate);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        // wait for either the next event or the next tick, whatever happens first
        tokio::select! {
            // listen to events that update the current state
            Some(stubs::mission::v0::StreamEventsResponse { event: Some(event), .. }) = events.next() => {
                handle_event(&mut state, event).await?;
            }

            // poll units for updates
            _ = interval.tick() => {
                update_units(&mut state).await?;
            }
        }
    }
}

/// The state of an active units stream.
// TODO: re-use one state for all concurrent unit streams?
struct State {
    units: HashMap<String, UnitState>,
    ctx: Context,
}

/// Various structs and options used to handle unit updates.
struct Context {
    rpc: MissionRpc,
    tx: Sender<Result<Update, Status>>,
    poll_rate: Duration,
    max_backoff: Duration,
}

/// Update the given [State] based on the given [Event].
async fn handle_event(state: &mut State, event: Event) -> Result<(), Error> {
    match event {
        Event::Birth(BirthEvent {
            initiator:
                Some(common::v0::Initiator {
                    initiator: Some(common::v0::initiator::Initiator::Unit(unit)),
                }),
            ..
        }) => {
            state.ctx.tx.send(Ok(Update::Unit(unit.clone()))).await?;
            state.units.insert(unit.name.clone(), UnitState::new(unit));
        }

        // The dead event is known to not fire reliably in certain cases. This is fine here, because
        // those cases are covered by removing those units when they are not found anymore during
        // attempted updates.
        Event::Dead(DeadEvent {
            initiator:
                Some(common::v0::Initiator {
                    initiator: Some(common::v0::initiator::Initiator::Unit(Unit { name, .. })),
                }),
        }) => {
            if let Some(unit_state) = state.units.remove(&name) {
                state
                    .ctx
                    .tx
                    .send(Ok(Update::Gone(UnitGone {
                        id: unit_state.unit.id,
                        name: unit_state.unit.name.clone(),
                    })))
                    .await?;
            }
        }

        _ => {}
    }

    Ok(())
}

/// Updates all units inside of the provided [State].
async fn update_units(state: &mut State) -> Result<(), Error> {
    let mut units = std::mem::take(&mut state.units);
    // Update all units in parallel (will queue a request for each unit, but the execution will
    // still be thottled by the throughputLimit setting).
    futures_util::future::try_join_all(
        units
            .values_mut()
            .map(|unit_state| update_unit(&state.ctx, unit_state)),
    )
    .await?;

    // remove state for all units that are gone
    units.retain(|_, v| !v.is_gone);
    state.units = units;

    Ok(())
}

async fn update_unit(ctx: &Context, unit_state: &mut UnitState) -> Result<(), Error> {
    if !unit_state.should_update() {
        return Ok(());
    }

    match unit_state.update(ctx).await {
        Ok(changed) => {
            if changed {
                ctx.tx
                    .send(Ok(Update::Unit(unit_state.unit.clone())))
                    .await?;
                unit_state.backoff = Duration::ZERO;
                unit_state.last_changed = Instant::now();
            }

            unit_state.last_checked = Instant::now();

            Ok(())
        }
        // if the unit was not found, flag it as gone, and continue with the next unit for now
        Err(err) if err.code() == Code::NotFound => {
            ctx.tx
                .send(Ok(Update::Gone(UnitGone {
                    id: unit_state.unit.id,
                    name: unit_state.unit.name.clone(),
                })))
                .await?;

            unit_state.is_gone = true;

            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

/// The last know information about a unit and various other information to track whether it is
/// worth checking the unit for updates or not.
struct UnitState {
    unit: Unit,
    backoff: Duration,
    last_checked: Instant,
    last_changed: Instant,
    is_gone: bool,
}

impl UnitState {
    fn new(unit: Unit) -> Self {
        Self {
            unit,
            backoff: Duration::ZERO,
            last_checked: Instant::now(),
            last_changed: Instant::now(),
            is_gone: false,
        }
    }

    /// Whether the unit should be checked for updates or not. This can be used to check stationary
    /// units less often.
    fn should_update(&self) -> bool {
        self.last_checked.elapsed() >= self.backoff
    }

    /// Check the unit for updates and return whether the unit got changed or not.
    async fn update(&mut self, ctx: &Context) -> Result<bool, Status> {
        let mut changed = false;

        #[derive(serde::Serialize)]
        struct GetUpdateRequest {
            name: String,
        }

        #[derive(Debug, serde::Deserialize)]
        struct Velocity {
            x: f64,
            y: f64,
            z: f64,
        }

        #[derive(Debug, serde::Deserialize)]
        struct Update {
            position: Position,
            velocity: Velocity,
        }

        let res: Update = ctx
            .rpc
            .request(
                "getUnitUpdate",
                Request::new(GetUpdateRequest {
                    name: self.unit.name.clone(),
                }),
            )
            .await?;

        // update position
        if let Some(position) = &self.unit.position {
            if !eq(&res.position, position) {
                changed = true;
                self.unit.position = Some(res.position);
            }
        } else {
            changed = true;
            self.unit.position = Some(res.position);
        }

        // update heading
        let mut heading = res.velocity.x.atan2(res.velocity.z).to_degrees().round();
        if heading < 0.0 {
            heading += 360.0;
        }
        if self.unit.heading != heading {
            changed = true;
            self.unit.heading = heading;
        }

        // update speed
        let speed = (res.velocity.z.powi(2) + res.velocity.x.powi(2)).sqrt();
        if (self.unit.speed - speed).abs() >= 0.01 {
            changed = true;
            self.unit.speed = speed;
        }

        // keep track of when it was last checked and changed and determine a corresponding backoff
        self.last_checked = Instant::now();
        if changed {
            self.last_changed = Instant::now();
            self.backoff = Duration::ZERO;
        } else {
            self.backoff = if self.backoff.is_zero() {
                ctx.poll_rate
            } else {
                (self.backoff * 2).min(ctx.max_backoff)
            }
        }

        Ok(changed)
    }
}

/// Check whether two positions are equal, taking an epsilon into account.
fn eq(a: &Position, b: &Position) -> bool {
    const LL_EPSILON: f64 = 0.000001;
    const ALT_EPSILON: f64 = 0.001;

    (a.lat - b.lat).abs() < LL_EPSILON
        && (a.lon - b.lon).abs() < LL_EPSILON
        && (a.alt - b.alt).abs() < ALT_EPSILON
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Status(#[from] Status),
    #[error("the cannel got closed")]
    Send(#[from] SendError<Result<Update, Status>>),
}
