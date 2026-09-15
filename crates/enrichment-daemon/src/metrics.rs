//! Operational counters for one daemon process (blueprint §14.3).
//!
//! Plain atomics behind an `Arc`, read into the wire types in
//! [`enrichment_core::wire::status`]. Deliberately small: §14.3 asks for engineering
//! diagnostics, and anything with a retention policy, a time base or a rollup would be a
//! different project.
//!
//! # Where the numbers come from
//!
//! Each counter is incremented at the one place that knows the fact, never inferred later:
//!
//! | Counter | Incremented in |
//! |---|---|
//! | `fetch.*` | [`crate::fetch::Fetcher::get_with_revalidation`], which is the only code that opens a socket |
//! | `evidence.*` | [`crate::server::dispatch`], after budget enforcement, so `response_bytes` is what was actually written |
//! | `verification.*` | [`crate::ops::verify`], when a probe finishes |
//! | `single_flight.*`, `lsp.*` | those subsystems, which already counted for gates C04 and C17; [`crate::single_flight::SingleFlight::running`] also times the producer run it wraps |
//!
//! # Structured logging
//!
//! [`Metrics::log_request`] writes one line per dispatched request. It goes to **stderr**, never
//! stdout: stdout is the MCP protocol channel for the adapter, and while this binary is not the
//! adapter, §7.4's habit is one the whole service keeps. The line is JSON so a log can be read
//! by `jq` without a parser, and it carries no request payload -- a library name is a caller's
//! business, and a log that records what someone asked about is a different artifact than a log
//! that records that the service answered.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use enrichment_core::wire::Status;
use enrichment_core::wire::status::{EvidenceCounters, FetchCounters, VerificationCounters};

/// Monotonic counters for one process. Cheap to clone: every handle shares one set.
#[derive(Debug, Default)]
pub struct Metrics {
    hits: AtomicU64,
    revalidated: AtomicU64,
    misses: AtomicU64,
    fetch_failures: AtomicU64,
    fetched_bytes: AtomicU64,

    requests: AtomicU64,
    ok: AtomicU64,
    partial: AtomicU64,
    pending: AtomicU64,
    errors: AtomicU64,
    gaps: AtomicU64,
    response_bytes: AtomicU64,

    verified: AtomicU64,
    verify_failed: AtomicU64,
    verify_unresolved: AtomicU64,
}

/// How one HTTP request was answered, from the cache's point of view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheOutcome {
    /// Answered from the service cache with no request to the origin.
    Hit,
    /// The origin was asked and answered `304`; the cached bytes were reused.
    Revalidated,
    /// A body was transferred.
    Miss,
}

/// How one verification probe finished.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeOutcome {
    /// It ran and succeeded.
    Succeeded,
    /// It ran and failed -- an observation about the code under test.
    Failed,
    /// It could not be run -- an observation about the service, not the code.
    Unresolved,
}

impl Metrics {
    /// A fresh set of counters.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Record one HTTP request's cache outcome and the bytes it moved.
    pub fn record_fetch(&self, outcome: CacheOutcome, transferred: u64) {
        let counter = match outcome {
            CacheOutcome::Hit => &self.hits,
            CacheOutcome::Revalidated => &self.revalidated,
            CacheOutcome::Miss => &self.misses,
        };
        counter.fetch_add(1, Ordering::Relaxed);
        // Only a miss actually moved bytes over the wire. Counting reused cache bytes here
        // would make the cache look like it costs what it saves.
        if outcome == CacheOutcome::Miss {
            self.fetched_bytes.fetch_add(transferred, Ordering::Relaxed);
        }
    }

    /// Record an HTTP request that never produced a response.
    pub fn record_fetch_failure(&self) {
        self.fetch_failures.fetch_add(1, Ordering::Relaxed);
    }

    /// Record one dispatched request: its outcome, whether it declared a gap, and its size.
    ///
    /// `status` is `None` for a frame that never reached the envelope layer at all -- a
    /// malformed request, an unknown method, a transport-level JSON-RPC error. Those count as
    /// errors, because excluding them would hide exactly the failures an operator looks for.
    pub fn record_response(&self, status: Option<Status>, has_gap: bool, bytes: u64) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        self.response_bytes.fetch_add(bytes, Ordering::Relaxed);
        match status {
            Some(Status::Ok) => &self.ok,
            Some(Status::Partial) => &self.partial,
            Some(Status::Pending) => &self.pending,
            Some(Status::Error) | None => &self.errors,
        }
        .fetch_add(1, Ordering::Relaxed);
        if has_gap {
            self.gaps.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Record how one verification probe finished.
    pub fn record_probe(&self, outcome: ProbeOutcome) {
        match outcome {
            ProbeOutcome::Succeeded => &self.verified,
            ProbeOutcome::Failed => &self.verify_failed,
            ProbeOutcome::Unresolved => &self.verify_unresolved,
        }
        .fetch_add(1, Ordering::Relaxed);
    }

    /// The HTTP counters as published.
    #[must_use]
    pub fn fetch(&self) -> FetchCounters {
        FetchCounters {
            hits: self.hits.load(Ordering::Relaxed),
            revalidated: self.revalidated.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            failures: self.fetch_failures.load(Ordering::Relaxed),
            fetched_bytes: self.fetched_bytes.load(Ordering::Relaxed),
        }
    }

    /// The answer counters as published.
    #[must_use]
    pub fn evidence(&self) -> EvidenceCounters {
        EvidenceCounters {
            requests: self.requests.load(Ordering::Relaxed),
            ok: self.ok.load(Ordering::Relaxed),
            partial: self.partial.load(Ordering::Relaxed),
            pending: self.pending.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
            gaps: self.gaps.load(Ordering::Relaxed),
            response_bytes: self.response_bytes.load(Ordering::Relaxed),
        }
    }

    /// The verification counters as published.
    #[must_use]
    pub fn verification(&self) -> VerificationCounters {
        VerificationCounters {
            succeeded: self.verified.load(Ordering::Relaxed),
            failed: self.verify_failed.load(Ordering::Relaxed),
            unresolved: self.verify_unresolved.load(Ordering::Relaxed),
        }
    }

    /// Write one structured line describing a dispatched request.
    ///
    /// Method, status, duration and size only. No parameters, no identities, no summary text:
    /// what a caller asked about is theirs, and a diagnostic log does not need it to be useful.
    pub fn log_request(method: &str, status: Option<Status>, millis: u64, bytes: usize) {
        let line = serde_json::json!({
            "at": enrichment_core::clock::now_rfc3339(),
            "event": "rpc",
            "method": method,
            "status": status.map_or("none", Self::status_name),
            "duration_ms": millis,
            "response_bytes": bytes,
        });
        eprintln!("{line}");
    }

    /// The wire spelling of a status, for logs. Matches `Status`'s serde representation.
    fn status_name(status: Status) -> &'static str {
        match status {
            Status::Ok => "ok",
            Status::Partial => "partial",
            Status::Pending => "pending",
            Status::Error => "error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_revalidated_request_is_neither_a_hit_nor_a_miss() {
        // The distinction this protects: `304` means the origin WAS contacted, so freshness was
        // re-established, but no body moved. Folding it into `hits` would overstate how offline
        // the service was; folding it into `misses` would overstate what it downloaded.
        let metrics = Metrics::new();
        metrics.record_fetch(CacheOutcome::Hit, 0);
        metrics.record_fetch(CacheOutcome::Revalidated, 4096);
        metrics.record_fetch(CacheOutcome::Miss, 1024);
        let fetch = metrics.fetch();
        assert_eq!(fetch.hits, 1);
        assert_eq!(fetch.revalidated, 1);
        assert_eq!(fetch.misses, 1);
        assert_eq!(
            fetch.fetched_bytes, 1024,
            "only the miss transferred a body; reused cache bytes are not downloads"
        );
    }

    #[test]
    fn a_gap_is_counted_separately_from_a_failure() {
        // An `ok` answer that names a gap is a correct answer to a bounded question. Counting
        // it as an error would make honest partial evidence look like a malfunction.
        let metrics = Metrics::new();
        metrics.record_response(Some(Status::Ok), true, 100);
        metrics.record_response(Some(Status::Ok), false, 100);
        metrics.record_response(Some(Status::Error), false, 50);
        let evidence = metrics.evidence();
        assert_eq!(evidence.requests, 3);
        assert_eq!(evidence.ok, 2);
        assert_eq!(evidence.errors, 1);
        assert_eq!(evidence.gaps, 1);
        assert_eq!(evidence.response_bytes, 250);
    }

    #[test]
    fn an_unrunnable_probe_is_not_a_failed_probe() {
        // §9.3: a probe that could not run says nothing about the code under test. Reporting it
        // as a failure would attribute a service limitation to a caller's library.
        let metrics = Metrics::new();
        metrics.record_probe(ProbeOutcome::Succeeded);
        metrics.record_probe(ProbeOutcome::Failed);
        metrics.record_probe(ProbeOutcome::Unresolved);
        metrics.record_probe(ProbeOutcome::Unresolved);
        let verification = metrics.verification();
        assert_eq!(verification.succeeded, 1);
        assert_eq!(verification.failed, 1);
        assert_eq!(verification.unresolved, 2);
    }

    #[test]
    fn a_frame_with_no_envelope_still_counts_as_an_answered_request() {
        // A JSON-RPC transport error never reaches the envelope layer. If it were not counted,
        // `requests` would silently exclude exactly the failures an operator is looking for.
        let metrics = Metrics::new();
        metrics.record_response(None, false, 80);
        let evidence = metrics.evidence();
        assert_eq!(evidence.requests, 1);
        assert_eq!(evidence.errors, 1);
    }
}
