//! The `service_status` payload, and the operational counters it publishes (§7.1, §14.3).
//!
//! These live in the core rather than in the daemon for the same reason every other tool
//! payload does: `data` is wire surface, and §6.3 gives the core one authoritative definition
//! from which `schemas/generated/tool-data.schema.json` and the Python DTOs are emitted. A
//! status payload defined only in the daemon would be the one tool result the adapter cannot
//! check against its own contract.
//!
//! # Why the counters are here at all
//!
//! Blueprint §14.3 asks for cache hits and misses, producer duration, queue depth, fetched and
//! response bytes, LSP startups and reuse, verification outcomes and evidence gaps. It also says
//! what they are for: **engineering diagnostics, not a benchmarking project**. So they are
//! plain monotonic counters on one process, published where an operator and a test can both read
//! them, and deliberately not:
//!
//! - a time series (nothing here is retained across a restart, and `service.status` says so),
//! - a quality score (§13 forbids converting results into a percentage, and the same reasoning
//!   applies to rolling these up),
//! - a claim about anything but this daemon process.
//!
//! Two of them double as acceptance oracles, which is why they are on the wire rather than only
//! in a log: gate C04 needs `single_flight.started` to check that two callers caused one
//! producer run, and gate C17 needs `lsp.started` to check that a signature-only inspection
//! started no language server.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// One producer or component and whether it is actually usable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ComponentStatus {
    /// The component's stable name.
    pub name: String,
    /// Whether it can be used right now.
    pub available: bool,
    /// The exact version when known, `None` when the component is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Why it is unavailable, or what it covers when it is. Never left blank on an absent
    /// component -- "blocked" must always name its missing prerequisite.
    pub detail: String,
}

impl ComponentStatus {
    /// A component that is not implemented yet, naming the phase that will implement it.
    #[must_use]
    pub fn not_implemented(name: &str, phase: u8) -> Self {
        Self {
            name: name.to_owned(),
            available: false,
            version: None,
            detail: format!("not implemented; scheduled for phase {phase}"),
        }
    }

    /// A component that is installed and usable.
    #[must_use]
    pub fn available(name: &str, version: &str, detail: &str) -> Self {
        Self {
            name: name.to_owned(),
            available: true,
            version: Some(version.to_owned()),
            detail: detail.to_owned(),
        }
    }
}

/// Versions of the daemon and its toolchain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Versions {
    /// The daemon crate version.
    pub daemon: String,
    /// The wire schema version this build emits.
    pub schema: String,
}

/// Wire schema compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SchemaCompatibility {
    /// The version emitted on every response.
    pub emits: String,
    /// Every version this daemon can accept.
    pub accepts: Vec<String>,
}

/// Execution-profile availability (blueprint §10).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Sandbox {
    /// Profiles configuration has enabled.
    ///
    /// Read from `LIBENR_CONFIG`; `enabled_profiles_source` names the file, or says these are
    /// built-in defaults. A caller selects from this list and never grants itself permission.
    pub enabled_profiles: Vec<String>,
    /// Where `enabled_profiles` came from, so a caller is never misled about policy.
    pub enabled_profiles_source: String,
    /// Container/isolation runtimes detected on this host.
    ///
    /// Detection only. A runtime being present is not the same as a profile being enabled, and
    /// this never enables one.
    pub available_runtimes: Vec<String>,
    /// Whether an actual containment run has qualified the configured execution images.
    ///
    /// Three different facts live next to each other here on purpose: a profile can be
    /// *enabled*, a runtime can be *present*, and images can be *configured*, and none of the
    /// three means the service can contain anything. Only `execution_qualified` says that, and
    /// it is set only by a receipt from `just execution-qualify`.
    pub execution_qualified: bool,
    /// When qualification happened and against which images, or the missing prerequisite.
    pub execution_readiness: String,
    /// The image IDs a qualification receipt covers. Empty when there is none.
    pub admitted_images: std::collections::BTreeMap<String, String>,
}

/// Producer runs started and callers served by a shared run (§8.2, gate C04).
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub struct SingleFlightCounts {
    /// Producer runs this process actually started.
    pub started: u64,
    /// Callers that attached to a run someone else started.
    pub shared: u64,
    /// Distinct pieces of work running right now.
    pub inflight: u64,
    /// Total wall time spent inside producer work, across every started run.
    ///
    /// Summed rather than averaged: a mean loses the one run that took a minute, and §14.3 wants
    /// a diagnostic, not a statistic. Divide by `started` if a mean is what you want.
    pub total_millis: u64,
}

/// Language-server starts, reuses and evictions in this daemon process (§9.2, gate C17).
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub struct LspMetrics {
    /// Sessions actually started, each one a language-server process.
    pub started: u64,
    /// Queries answered by a session that was already warm.
    pub reused: u64,
    /// Sessions shut down to stay within the configured bound, or after idling out.
    pub evicted: u64,
    /// Sessions warm right now.
    pub warm: u64,
}

/// HTTP cache behaviour and transferred bytes (§14.3, §8.4).
///
/// The three cache outcomes are kept apart because they answer different questions. A `hit`
/// opened no socket at all. A `revalidated` opened one and the server said `304`, so the bytes
/// were reused but freshness was actually re-established. A `miss` transferred a body. Folding
/// the middle case into either of the others would misreport both cost and freshness.
///
/// **Read `hits` narrowly.** The service revalidates every stored `200` rather than serving it
/// blind, so a reused *representation* shows up as `revalidated`, not as a hit. The only thing
/// that increments `hits` is the bounded negative cache — a recent `404`/`410` reused within
/// `freshness.negative_cache_ttl_seconds`. `hits + revalidated` is the number an operator
/// probably means by "the cache worked"; `hits` alone is "we did not even ask".
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub struct FetchCounters {
    /// Requests answered from the service cache without contacting the origin.
    pub hits: u64,
    /// Requests where validators matched and the origin returned `304`.
    pub revalidated: u64,
    /// Requests that transferred a body from the origin.
    pub misses: u64,
    /// Requests that failed before producing a response.
    pub failures: u64,
    /// Body bytes actually transferred, excluding reused cache bytes.
    pub fetched_bytes: u64,
}

/// What this process has answered, and how truthfully it could (§14.3, §6.2).
///
/// `gaps` counts answers that declared at least one missing piece of coverage -- not answers
/// that failed. An `ok` result with a named gap is a correct answer to a bounded question, and
/// the count exists so an operator can see how often the service is working with partial
/// evidence, which is a different question from how often it errors.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub struct EvidenceCounters {
    /// Requests dispatched, of every method.
    pub requests: u64,
    /// Envelopes answered `ok`.
    pub ok: u64,
    /// Envelopes answered `partial`.
    pub partial: u64,
    /// Envelopes answered `pending`, having handed back a job.
    pub pending: u64,
    /// Envelopes answered `error`.
    pub errors: u64,
    /// Answers that named at least one gap in `coverage.missing`.
    pub gaps: u64,
    /// Response bytes produced, after budget enforcement.
    ///
    /// "Produced", not "written": a JSON-RPC *notification* is dispatched and answered with
    /// nothing, and its answer is measured here anyway. The work happened and the bytes were
    /// built; excluding them would make this number disagree with `requests` for no reason a
    /// reader could see.
    pub response_bytes: u64,
}

/// How verification work finished (§9.3, §14.3).
///
/// `unresolved` is its own outcome rather than a kind of failure: a probe the service could not
/// run -- an unqualified image, an unavailable profile -- says nothing about the code under
/// test, and counting it as a failure would overstate what was observed.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub struct VerificationCounters {
    /// Probes that ran and succeeded.
    pub succeeded: u64,
    /// Probes that ran and failed, which is an observation about the code.
    pub failed: u64,
    /// Probes that could not be run, which is an observation about the service.
    pub unresolved: u64,
}

/// Process-scoped native query diagnostics; neither library coverage nor a peak-RSS claim.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct NativeQueryCounters {
    /// Completed or interrupted physical executions recorded since startup.
    pub executions: u64,
    pub completed: u64,
    /// Failed, cancelled or dropped executions, including partial operator counters.
    pub incomplete: u64,
    /// Sum of observed planning time across recorded executions.
    pub planning_micros: u64,
    /// Sum of elapsed execution durations, which overlap for concurrent queries.
    pub elapsed_micros: u64,
    /// Queries currently holding a shared admission permit, including stream writers.
    pub admitted: usize,
    pub concurrency_limit: usize,
    /// Configured managed memory ceiling; external parser/scan allocations are separate.
    pub managed_memory_limit_bytes: usize,
    pub spill_limit_bytes: u64,
    pub metadata_cache_limit_bytes: usize,
}

/// Queue, cache and operational health.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Health {
    /// Jobs currently queued.
    pub queued_jobs: u64,
    /// Jobs currently running.
    pub running_jobs: u64,
    /// Whether the evidence store is open and writable.
    pub cache_ready: bool,
    /// The data root in use, when a store is open. Absent otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_root: Option<String>,
    /// Producer runs started, and callers served by a run someone else started (§8.2, C04).
    pub single_flight: SingleFlightCounts,
    /// Language-server starts, reuses and evictions in this daemon process.
    ///
    /// Published because gate C17 -- "inspection does not start an LSP unnecessarily" -- is only
    /// checkable against a counter. A caller reading `started` before and after a signature-only
    /// inspection can see for itself that nothing was started. Blueprint §14.3 asks for the same
    /// numbers as operational metrics.
    pub lsp: LspMetrics,
    /// HTTP cache outcomes and transferred bytes since this process started.
    pub fetch: FetchCounters,
    /// What this process has answered, and how completely.
    pub evidence: EvidenceCounters,
    /// How verification probes finished.
    pub verification: VerificationCounters,
    /// Native query counters and configured limits, absent when no runtime is open.
    pub native_queries: Option<NativeQueryCounters>,
    /// How long this process has been up, in seconds.
    ///
    /// Every counter beside it is scoped to that window. Without this an operator reading
    /// `fetch.misses = 3` cannot tell a quiet hour from a daemon that restarted a minute ago.
    pub uptime_seconds: u64,
}

/// The `service_status` result (§7.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StatusData {
    /// Component versions.
    pub versions: Versions,
    /// Which wire schema versions this daemon can speak.
    pub schema_compatibility: SchemaCompatibility,
    /// On-disk storage compatibility; independent of the MCP response envelope.
    pub snapshot_compatibility: SchemaCompatibility,
    /// Which execution profiles are actually usable here.
    pub sandbox: Sandbox,
    /// Evidence producers and whether each is installed.
    pub producers: Vec<ComponentStatus>,
    /// Optional capabilities and whether each is supported.
    pub features: Vec<ComponentStatus>,
    /// Job queue and cache health.
    pub health: Health,
}
