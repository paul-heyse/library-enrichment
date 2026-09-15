//! Process-local acquisition counters. Durable sharing and caller interests live in `jobs`.
pub use enrichment_core::wire::status::SingleFlightCounts as Counts;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct SingleFlight {
    counters: Mutex<Counts>,
}
pub struct Running<'a> {
    metrics: &'a SingleFlight,
    began: std::time::Instant,
}
impl Drop for Running<'_> {
    fn drop(&mut self) {
        if let Ok(mut c) = self.metrics.counters.lock() {
            c.inflight = c.inflight.saturating_sub(1);
            c.total_millis = c.total_millis.saturating_add(
                u64::try_from(self.began.elapsed().as_millis()).unwrap_or(u64::MAX),
            );
        }
    }
}
impl SingleFlight {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
    #[must_use]
    pub fn counts(&self) -> Counts {
        self.counters.lock().map(|c| *c).unwrap_or_default()
    }
    pub fn submitted(&self, new: bool) {
        if !new && let Ok(mut c) = self.counters.lock() {
            c.shared = c.shared.saturating_add(1);
        }
    }
    pub fn running(&self) -> Running<'_> {
        if let Ok(mut c) = self.counters.lock() {
            c.started = c.started.saturating_add(1);
            c.inflight = c.inflight.saturating_add(1);
        }
        Running {
            metrics: self,
            began: std::time::Instant::now(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts_only_started_work_and_shared_interests() {
        let metrics = SingleFlight::new();
        metrics.submitted(true);
        assert_eq!(metrics.counts().started, 0);
        let running = metrics.running();
        metrics.submitted(false);
        assert_eq!(metrics.counts().started, 1);
        assert_eq!(metrics.counts().shared, 1);
        assert_eq!(metrics.counts().inflight, 1);
        drop(running);
        assert_eq!(metrics.counts().inflight, 0);
    }
}
