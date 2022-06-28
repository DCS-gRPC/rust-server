use std::future::Future;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use dcs_module_ipc::IPC;
use stubs::mission::v0::stream_events_response::{Event, SimulationFpsEvent};
use stubs::mission::v0::StreamEventsResponse;
use tokio::time::{interval, MissedTickBehavior};

static FPS: AtomicU32 = AtomicU32::new(0);
static TIME: AtomicU32 = AtomicU32::new(0);

pub fn frame(time: f64) {
    // Increase the frame count by one
    FPS.fetch_add(1, Ordering::Relaxed);

    // Update the DCS simulation time (convert it to an int as there are no atomic floats)
    TIME.store((time * 1000.0) as u32, Ordering::Relaxed);
}

pub async fn run_in_background(
    ipc: IPC<StreamEventsResponse>,
    mut shutdown_signal: impl Future<Output = ()> + Unpin,
) {
    let mut interval = interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    // clear FPS counter when first being started
    let mut previous = interval.tick().await;
    FPS.store(0, Ordering::Relaxed);

    loop {
        // wait for either the shutdown signal or the next interval tick, whatever happens first
        let instant = tokio::select! {
            _ = &mut shutdown_signal => {
                break
            }
            instant = interval.tick() => instant
        };

        // Technically, there could be a simulation frame between the read of `frame_count` and
        // `time` so that `time` already receives a newer value. However, this should be rare and
        // shouldn't really matter for the resolution measured here.
        let frame_count = FPS.swap(0, Ordering::Relaxed);
        let time = TIME.load(Ordering::Relaxed);

        let elapsed = instant - previous;
        previous = instant;
        let average = (frame_count as f64) / elapsed.as_secs_f64();

        ipc.event(StreamEventsResponse {
            time: f64::from(time) / 1000.0,
            event: Some(Event::SimulationFps(SimulationFpsEvent { average })),
        })
        .await;
    }
}
