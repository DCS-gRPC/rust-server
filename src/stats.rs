use std::convert::TryFrom;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;
use tokio::time::MissedTickBehavior;

use crate::shutdown::ShutdownHandle;

#[derive(Clone)]
pub struct Stats(Arc<Inner>);

struct Inner {
    shutdown_signal: ShutdownHandle,
    /// Total numer of calls into the MSE.
    calls_count: AtomicU32,
    /// Total numer of events received from the MSE.
    events_count: AtomicU32,
    /// Total numer of calls in the queue.
    queue_size: AtomicU32,
    /// Time spent waiting for MSE calls to complete (since last report).
    nanoseconds_waited: AtomicUsize,
    /// Stats collected during an interval necessary to create a report at the end of the interval.
    interval_stats: Arc<Mutex<IntervalStats>>,
}

#[derive(Default)]
struct IntervalStats {
    /// Highest TPS count of calls into the MSE.
    tps_highest: f64,
    /// Highest events per second.
    eps_highest: f64,
    /// Sum of the queue sizes at each tick (neccessary to calculate the average).
    queue_size_total: u32,
    /// Highest queue size at a tick of the interval.
    queue_size_highest: u32,
}

/// This guard is used to keep track of the time the gRPC server blocked DCS.
pub struct TrackBlockTimeGuard {
    start: Instant,
    stats: Arc<Inner>,
}

/// This guard is used to keep track of calls in the queue.
pub struct TrackQueueSizeGuard {
    stats: Arc<Inner>,
}

impl Stats {
    pub fn new(shutdown_signal: ShutdownHandle) -> Self {
        Stats(Arc::new(Inner {
            shutdown_signal,
            calls_count: AtomicU32::new(0),
            events_count: AtomicU32::new(0),
            queue_size: AtomicU32::new(0),
            nanoseconds_waited: AtomicUsize::new(0),
            interval_stats: Arc::new(Mutex::new(IntervalStats::default())),
        }))
    }

    pub fn track_call(&self) {
        self.0.calls_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn track_event(&self) {
        self.0.events_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn track_block_time(&self, start: Instant) -> TrackBlockTimeGuard {
        self.0.calls_count.fetch_add(1, Ordering::Relaxed);
        TrackBlockTimeGuard {
            start,
            stats: self.0.clone(),
        }
    }

    pub fn track_queue_size(&self) -> TrackQueueSizeGuard {
        self.0.queue_size.fetch_add(1, Ordering::Relaxed);
        TrackQueueSizeGuard {
            stats: self.0.clone(),
        }
    }

    pub async fn run_in_background(self) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let mut last_logged = Instant::now();
        let log_interval = Duration::from_secs(60);
        let mut shutdown_signal = self.0.shutdown_signal.signal();

        loop {
            let calls_count_before = self.0.calls_count.load(Ordering::Relaxed);
            let events_count_before = self.0.events_count.load(Ordering::Relaxed);
            let start = Instant::now();

            // wait for either the shutdown signal or the next interval tick, whatever happens first
            tokio::select! {
                _ = &mut shutdown_signal => {
                    break
                }
                _ = interval.tick() => {}
            };

            let mut interval_stats = self.0.interval_stats.lock().await;
            let calls_count = self.0.calls_count.load(Ordering::Relaxed);
            let events_count = self.0.events_count.load(Ordering::Relaxed);

            // update report for elapsed second
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                // update highest TPS
                let tps = f64::from(calls_count - calls_count_before) / elapsed;
                if tps > interval_stats.tps_highest {
                    interval_stats.tps_highest = tps;
                }

                // update highest events per second
                let eps = f64::from(events_count - events_count_before) / elapsed;
                if eps > interval_stats.eps_highest {
                    interval_stats.eps_highest = eps;
                }

                // update queue size
                let queue_size = self.0.queue_size.load(Ordering::Relaxed);
                interval_stats.queue_size_total += queue_size;
                if queue_size > interval_stats.queue_size_highest {
                    interval_stats.queue_size_highest = queue_size;
                }
            }

            // log summary every minute
            let elapsed = last_logged.elapsed();
            if elapsed > log_interval {
                // average TPS
                let tps_average =
                    f64::try_from(calls_count).unwrap_or(f64::MAX) / elapsed.as_secs_f64();

                // average events per second
                let eps_average =
                    f64::try_from(events_count).unwrap_or(f64::MAX) / elapsed.as_secs_f64();

                // total block time
                let block_time_total = Duration::from_nanos(
                    u64::try_from(self.0.nanoseconds_waited.swap(0, Ordering::Relaxed))
                        .unwrap_or(u64::MAX),
                );
                let block_time_total_percentage =
                    (block_time_total.as_secs_f64() / elapsed.as_secs_f64()) * 100.0;

                // average queue size
                let queue_size_average = f64::try_from(interval_stats.queue_size_total)
                    .unwrap_or(f64::MAX)
                    / elapsed.as_secs_f64();

                // format and log stats
                log::info!(
                    "Calls per second: average={:.2}, highest={:.2}",
                    tps_average,
                    interval_stats.tps_highest
                );
                log::info!(
                    "Events per second: average={:.2}, highest={:.2}",
                    eps_average,
                    interval_stats.eps_highest
                );
                log::info!(
                    "Blocking time: total={:?} (≙ {:.2}%)",
                    block_time_total,
                    block_time_total_percentage
                );
                log::info!(
                    "Queue size: average={:.2}, biggest={:.2}",
                    queue_size_average,
                    interval_stats.queue_size_highest
                );

                // reset data for next interval
                last_logged = Instant::now();
                *interval_stats = IntervalStats::default();
                self.0.calls_count.store(0, Ordering::Relaxed);
                self.0.nanoseconds_waited.store(0, Ordering::Relaxed);
            }
        }
    }
}

impl Drop for TrackBlockTimeGuard {
    fn drop(&mut self) {
        self.stats.nanoseconds_waited.fetch_add(
            usize::try_from(self.start.elapsed().as_nanos()).unwrap_or(usize::MAX),
            Ordering::Relaxed,
        );
    }
}

impl Drop for TrackQueueSizeGuard {
    fn drop(&mut self) {
        self.stats.queue_size.fetch_sub(1, Ordering::Relaxed);
    }
}
