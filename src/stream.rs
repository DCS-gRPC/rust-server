use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::rpc::dcs::coalitions_server::Coalitions;
use crate::rpc::dcs::event::Event;
use crate::rpc::dcs::group::GetUnitsRequest;
use crate::rpc::dcs::groups_server::Groups;
use crate::rpc::dcs::unit_update::{UnitGone, Update};
use crate::rpc::dcs::units_server::Units;
use crate::rpc::dcs::{Coalition, GetGroupsRequest, Position, StreamUnitsRequest, Unit, UnitName};
use crate::rpc::RPC;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use tokio::time::MissedTickBehavior;
use tonic::{Code, Request, Status};

/// Stream unit updates.
pub async fn stream_units(
    opts: StreamUnitsRequest,
    rpc: RPC,
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
    let groups = {
        let mut groups = Vec::new();

        for coalition in &[Coalition::Blue, Coalition::Red, Coalition::Neutral] {
            groups.extend(
                state
                    .ctx
                    .rpc
                    .get_groups(Request::new(GetGroupsRequest {
                        coalition: (*coalition).into(),
                        category: None,
                    }))
                    .await?
                    .into_inner()
                    .groups,
            );
        }

        groups
    };

    for group in groups {
        let res = state
            .ctx
            .rpc
            .get_units(Request::new(GetUnitsRequest {
                group_name: group.name.clone(),
                active: Some(true),
            }))
            .await?
            .into_inner();

        state.units.extend(
            res.units
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
            Some(crate::rpc::dcs::Event { event: Some(event), .. }) = events.next() => {
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
    rpc: RPC,
    tx: Sender<Result<Update, Status>>,
    poll_rate: Duration,
    max_backoff: Duration,
}

/// Update the given [State] based on the given [Event].
async fn handle_event(state: &mut State, event: Event) -> Result<(), Error> {
    use crate::rpc::dcs::event::{BirthEvent, DeadEvent};
    use crate::rpc::dcs::{initiator, Initiator};

    match event {
        Event::Birth(BirthEvent {
            initiator:
                Some(Initiator {
                    initiator: Some(initiator::Initiator::Unit(unit)),
                }),
        }) => {
            state.ctx.tx.send(Ok(Update::Unit(unit.clone()))).await?;
            state.units.insert(unit.name.clone(), UnitState::new(unit));
        }

        // The dead event is known to not fire reliably in certain cases. This is fine here, because
        // those cases are covered by removing those units when they are not found anymore during
        // attempted updates.
        Event::Dead(DeadEvent {
            initiator:
                Some(Initiator {
                    initiator: Some(initiator::Initiator::Unit(Unit { name, .. })),
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
    for unit_state in state.units.values_mut() {
        if !unit_state.should_update() {
            continue;
        }

        match unit_state.update(&state.ctx).await {
            Ok(changed) => {
                if changed {
                    state
                        .ctx
                        .tx
                        .send(Ok(Update::Unit(unit_state.unit.clone())))
                        .await?;
                    unit_state.backoff = Duration::ZERO;
                    unit_state.last_changed = Instant::now();
                }

                unit_state.last_checked = Instant::now();
            }
            // if the unit was not found, flag it as gone, and continue with the next unit for now
            Err(err) if err.code() == Code::NotFound => {
                state
                    .ctx
                    .tx
                    .send(Ok(Update::Gone(UnitGone {
                        id: unit_state.unit.id,
                        name: unit_state.unit.name.clone(),
                    })))
                    .await?;

                unit_state.is_gone = true;
                continue;
            }
            Err(err) => return Err(err.into()),
        }
    }

    // remove state for all units that are gone
    state.units.retain(|_, v| !v.is_gone);

    Ok(())
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

        // update position
        let position = Units::get_position(
            &ctx.rpc,
            Request::new(UnitName {
                name: self.unit.name.clone(),
            }),
        )
        .await?
        .into_inner()
        .position;
        if let (Some(a), Some(b)) = (position.as_ref(), self.unit.position.as_ref()) {
            if !eq(a, b) {
                changed = true;
                self.unit.position = position;
            }
        }

        // update player name
        let player_name = Units::get_player_name(
            &ctx.rpc,
            Request::new(UnitName {
                name: self.unit.name.clone(),
            }),
        )
        .await?
        .into_inner()
        .player_name;
        if player_name != self.unit.player_name {
            changed = true;
            self.unit.player_name = player_name;
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
