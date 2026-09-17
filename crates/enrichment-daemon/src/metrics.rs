//! Capture service observations; DataFusion over native_events owns every counter.
pub use enrichment_core::telemetry::{CacheOutcome, ProbeOutcome};
use enrichment_core::{
    telemetry::{ServiceCounters, ServiceObservation},
    wire::Status,
};
use std::sync::Arc;

pub struct Metrics {
    runtime: enrichment_store::runtime::QueryRuntime,
}
impl Metrics {
    pub fn new(runtime: &enrichment_store::runtime::QueryRuntime) -> Arc<Self> {
        Arc::new(Self {
            runtime: runtime.clone(),
        })
    }
    pub fn record_fetch(&self, outcome: CacheOutcome, transferred: u64) {
        self.runtime.record_service(ServiceObservation::Fetch {
            outcome,
            transferred,
        });
    }
    pub fn record_fetch_failure(&self) {
        self.runtime
            .record_service(ServiceObservation::FetchFailure);
    }
    pub fn record_probe(&self, outcome: ProbeOutcome) {
        self.runtime
            .record_service(ServiceObservation::Probe { outcome });
    }
    pub fn record_response(
        &self,
        method: &str,
        status: Option<Status>,
        has_gap: bool,
        elapsed_micros: u64,
        bytes: u64,
    ) {
        self.runtime.record_service(ServiceObservation::Response {
            method: method.into(),
            status,
            has_gap,
            elapsed_micros,
            bytes,
        });
    }
    pub async fn counters(&self) -> std::io::Result<ServiceCounters> {
        self.runtime
            .service_counters()
            .await
            .map_err(std::io::Error::other)
    }
}
