use std::future::Future;
use std::time::{Duration, Instant};

use dcs_module_ipc::IPC;
use once_cell::sync::Lazy;
use stubs::mission::v0::stream_events_response::{Event, SimulationFpsEvent};
use stubs::mission::v0::StreamEventsResponse;
use tokio::sync::Mutex;
use tokio::time::{interval, MissedTickBehavior};

static FPS: Lazy<Mutex<Option<Fps>>> = Lazy::new(Default::default);

struct Fps {
    /// The DCS simulation time of the last update.
    simulation_time: f64,
    /// The DCS real time of the last update.
    real_time: f64,
    /// The instant the previous frame got measured.
    previous: Instant,
    /// Collection of frame times in milliseconds.
    frames_in_dcs_time: Vec<f32>,
    /// Collection of frame times in milliseconds.
    frames_in_rust_time: Vec<f32>,
}

pub fn frame(simulation_time: f64, real_time: f64) {
    let mut fps = FPS.blocking_lock();
    let mut fps = match fps.as_mut() {
        Some(fps) => fps,
        None => {
            *fps = Some(Fps {
                simulation_time,
                real_time,
                previous: Instant::now(),
                frames_in_dcs_time: Vec::with_capacity(300),
                frames_in_rust_time: Vec::with_capacity(300),
            });
            return;
        }
    };

    fps.frames_in_dcs_time
        .push(((real_time - fps.real_time) * 1000.0) as f32);
    fps.frames_in_rust_time
        .push(fps.previous.elapsed().as_secs_f32() * 1000.0);

    fps.simulation_time = simulation_time;
    fps.real_time = real_time;
    fps.previous = Instant::now();
}

pub async fn run_in_background(
    ipc: IPC<StreamEventsResponse>,
    mut shutdown_signal: impl Future<Output = ()> + Unpin,
) {
    let mut interval = interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    // clear FPS stats when first being started
    reset().await;

    // keep another instance around to switch instances instead of creating new ones each time the
    // interval ticks
    let mut fps = Fps {
        simulation_time: 0.0,
        real_time: 0.0,
        previous: Instant::now(),
        frames_in_dcs_time: Vec::with_capacity(300),
        frames_in_rust_time: Vec::with_capacity(300),
    };

    loop {
        // wait for either the shutdown signal or the next interval tick, whatever happens first
        tokio::select! {
            _ = &mut shutdown_signal => {
                break
            }
            _ = interval.tick() => {}
        };

        // scope the mutex access to release the lock as soon as possible
        {
            let mut current = FPS.lock().await;
            match current.as_mut() {
                Some(current) => {
                    // swap out the current instance to be able to release the lock right away
                    fps.simulation_time = current.simulation_time;
                    fps.real_time = current.real_time;
                    fps.previous = current.previous;
                    std::mem::swap(current, &mut fps);
                }
                None => continue,
            }
        }

        let dcs_time = fps.frames_in_dcs_time.iter().copied().sum::<f32>();
        let average_in_dcs_time = fps.frames_in_dcs_time.len() as f32 / (dcs_time / 1000.0);
        let lowest_in_dcs_time = dcs_time
            / fps
                .frames_in_dcs_time
                .iter()
                .copied()
                .fold(f32::NEG_INFINITY, f32::max);
        let highest_in_dcs_time = dcs_time
            / fps
                .frames_in_dcs_time
                .iter()
                .copied()
                .fold(f32::INFINITY, f32::min);

        let rust_time: f32 = fps.frames_in_rust_time.iter().sum::<f32>();
        let average_in_rust_time = fps.frames_in_rust_time.len() as f32 / (rust_time / 1000.0);
        let lowest_in_rust_time = rust_time
            / fps
                .frames_in_rust_time
                .iter()
                .copied()
                .fold(f32::NEG_INFINITY, f32::max);
        let highest_in_rust_time = rust_time
            / fps
                .frames_in_rust_time
                .iter()
                .copied()
                .fold(f32::INFINITY, f32::min);

        ipc.event(StreamEventsResponse {
            time: fps.simulation_time,
            event: Some(Event::SimulationFps(SimulationFpsEvent {
                frames_in_dcs_time: fps.frames_in_dcs_time.clone(),
                average_in_dcs_time,
                lowest_in_dcs_time,
                highest_in_dcs_time,
                frames_in_rust_time: fps.frames_in_rust_time.clone(),
                average_in_rust_time,
                lowest_in_rust_time,
                highest_in_rust_time,
            })),
        })
        .await;

        // prepare fps variable to be reused in next iteration
        fps.frames_in_dcs_time.clear();
        fps.frames_in_dcs_time.shrink_to(512);
        fps.frames_in_rust_time.clear();
        fps.frames_in_rust_time.shrink_to(512);
    }
}

async fn reset() {
    let mut fps = FPS.lock().await;
    *fps = None;
}
