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

crate::native_struct! {
/// One producer or component and whether it is actually usable.
pub struct ComponentStatus {
    /// The component's stable name.
    name: String => crate::native_union::Rule::Text,
    /// Whether it can be used right now.
    available: bool => crate::native_union::Rule::Text,
    /// The exact version when known, `None` when the component is absent.
    version: Option<String> => crate::native_union::Rule::Text,
    /// Why it is unavailable, or what it covers when it is. Never left blank on an absent
    /// component -- "blocked" must always name its missing prerequisite.
    detail: String => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// Versions of the daemon and its toolchain.
pub struct Versions {
    /// The daemon crate version.
    daemon: String => crate::native_union::Rule::Text,
    /// The wire schema version this build emits.
    schema: String => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Wire schema compatibility.
pub struct SchemaCompatibility {
    /// The version emitted on every response.
    emits: String => crate::native_union::Rule::Text,
    /// Every version this daemon can accept.
    accepts: Vec<String> => crate::native_union::Rule::Sequence,
}
}

crate::native_struct! {
/// Execution-profile availability (blueprint §10).
pub struct Sandbox {
    /// Implemented execution routes assessed by the same admission logic used by tools.
    /// Empty when no running service is available to assess qualification and cleanup.
    execution_routes: Vec<ExecutionReadiness> => crate::native_union::Rule::Sequence,
    /// Profiles configuration has enabled.
    ///
    /// Read from `LIBENR_CONFIG`; `enabled_profiles_source` names the file, or says these are
    /// built-in defaults. A caller selects from this list and never grants itself permission.
    enabled_profiles: Vec<String> => crate::native_union::Rule::Sequence,
    /// Where `enabled_profiles` came from, so a caller is never misled about policy.
    enabled_profiles_source: String => crate::native_union::Rule::Text,
    /// Container/isolation runtimes detected on this host.
    ///
    /// Detection only. A runtime being present is not the same as a profile being enabled, and
    /// this never enables one.
    available_runtimes: Vec<String> => crate::native_union::Rule::Sequence,
    /// Whether an actual containment run has qualified the configured execution images.
    ///
    /// Three different facts live next to each other here on purpose: a profile can be
    /// *enabled*, a runtime can be *present*, and images can be *configured*, and none of the
    /// three means the service can contain anything. Only `execution_qualified` says that, and
    /// it is set only by a receipt from `just execution-qualify`.
    execution_qualified: bool => crate::native_union::Rule::Text,
    /// When qualification happened and against which images, or the missing prerequisite.
    execution_readiness: String => crate::native_union::Rule::Text,
    /// The image IDs a qualification receipt covers. Empty when there is none.
    admitted_images: std::collections::BTreeMap<String, String> => crate::native_union::Rule::Map,
}
}

crate::native_struct! {
/// Producer runs started and callers served by a shared run (§8.2, gate C04).
#[derive(Copy, Default, PartialOrd, Ord)]
pub struct SingleFlightCounts {
    /// Producer runs this process actually started.
    started: u64 => crate::native_union::Rule::Text,
    /// Callers that attached to a run someone else started.
    shared: u64 => crate::native_union::Rule::Text,
    /// Distinct pieces of work running right now.
    inflight: u64 => crate::native_union::Rule::Text,
    /// Total wall time spent inside producer work, across every started run.
    ///
    /// Summed rather than averaged: a mean loses the one run that took a minute, and §14.3 wants
    /// a diagnostic, not a statistic. Divide by `started` if a mean is what you want.
    total_millis: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Language-server starts, reuses and evictions in this daemon process (§9.2, gate C17).
#[derive(Copy, Default, PartialOrd, Ord)]
pub struct LspMetrics {
    /// Sessions actually started, each one a language-server process.
    started: u64 => crate::native_union::Rule::Text,
    /// Queries answered by a session that was already warm.
    reused: u64 => crate::native_union::Rule::Text,
    /// Sessions shut down to stay within the configured bound, or after idling out.
    evicted: u64 => crate::native_union::Rule::Text,
    /// Sessions warm right now.
    warm: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
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
#[derive(Copy, Default, PartialOrd, Ord)]
pub struct FetchCounters {
    /// Requests answered from the service cache without contacting the origin.
    hits: u64 => crate::native_union::Rule::Text,
    /// Requests where validators matched and the origin returned `304`.
    revalidated: u64 => crate::native_union::Rule::Text,
    /// Requests that transferred a body from the origin.
    misses: u64 => crate::native_union::Rule::Text,
    /// Requests that failed before producing a response.
    failures: u64 => crate::native_union::Rule::Text,
    /// Body bytes actually transferred, excluding reused cache bytes.
    fetched_bytes: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// What this process has answered, and how truthfully it could (§14.3, §6.2).
///
/// `gaps` counts answers that declared at least one missing piece of coverage -- not answers
/// that failed. An `ok` result with a named gap is a correct answer to a bounded question, and
/// the count exists so an operator can see how often the service is working with partial
/// evidence, which is a different question from how often it errors.
#[derive(Copy, Default, PartialOrd, Ord)]
pub struct EvidenceCounters {
    /// Requests dispatched, of every method.
    requests: u64 => crate::native_union::Rule::Text,
    /// Envelopes answered `ok`.
    ok: u64 => crate::native_union::Rule::Text,
    /// Envelopes answered `partial`.
    partial: u64 => crate::native_union::Rule::Text,
    /// Envelopes answered `pending`, having handed back a job.
    pending: u64 => crate::native_union::Rule::Text,
    /// Envelopes answered `error`.
    errors: u64 => crate::native_union::Rule::Text,
    /// Answers that named at least one gap in `coverage.missing`.
    gaps: u64 => crate::native_union::Rule::Text,
    /// Response bytes produced, after budget enforcement.
    ///
    /// "Produced", not "written": a JSON-RPC *notification* is dispatched and answered with
    /// nothing, and its answer is measured here anyway. The work happened and the bytes were
    /// built; excluding them would make this number disagree with `requests` for no reason a
    /// reader could see.
    response_bytes: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// How verification work finished (§9.3, §14.3).
///
/// `unresolved` is its own outcome rather than a kind of failure: a probe the service could not
/// run -- an unqualified image, an unavailable profile -- says nothing about the code under
/// test, and counting it as a failure would overstate what was observed.
#[derive(Copy, Default, PartialOrd, Ord)]
pub struct VerificationCounters {
    /// Probes that ran and succeeded.
    succeeded: u64 => crate::native_union::Rule::Text,
    /// Probes that ran and failed, which is an observation about the code.
    failed: u64 => crate::native_union::Rule::Text,
    /// Probes that could not be run, which is an observation about the service.
    unresolved: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Process-scoped native query diagnostics; neither library coverage nor a peak-RSS claim.
#[derive(Default)]
pub struct NativeQueryCounters {
    diagnostic_observations_dropped: u64 => crate::native_union::Rule::Text,
    /// Current shared-runtime managed reservations, not process RSS.
    managed_memory_reserved_bytes: usize => crate::native_union::Rule::Text,
    /// Shared-runtime maximum since startup; never reset or attributed to one query.
    managed_memory_peak_bytes: usize => crate::native_union::Rule::Text,
    /// The read-only values consumed by native source construction.
    effective_native_settings: std::collections::BTreeMap<String, String> => crate::native_union::Rule::Map,
    /// Completed or interrupted physical executions recorded since startup.
    executions: u64 => crate::native_union::Rule::Text,
    completed: u64 => crate::native_union::Rule::Text,
    /// Failed, cancelled or dropped executions, including partial operator counters.
    incomplete: u64 => crate::native_union::Rule::Text,
    /// Sum of observed planning time across recorded executions.
    planning_micros: u64 => crate::native_union::Rule::Text,
    /// Sum of elapsed execution durations, which overlap for concurrent queries.
    elapsed_micros: u64 => crate::native_union::Rule::Text,
    /// Queries currently holding a shared admission permit, including stream writers.
    admitted: usize => crate::native_union::Rule::Text,
    concurrency_limit: usize => crate::native_union::Rule::Text,
    /// Configured managed memory ceiling; external parser/scan allocations are separate.
    managed_memory_limit_bytes: usize => crate::native_union::Rule::Text,
    spill_limit_bytes: u64 => crate::native_union::Rule::Text,
    metadata_cache_limit_bytes: usize => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Queue, cache and operational health.
pub struct Health {
    /// Jobs currently queued.
    queued_jobs: u64 => crate::native_union::Rule::Text,
    /// Jobs currently running.
    running_jobs: u64 => crate::native_union::Rule::Text,
    /// Whether the evidence store is open and writable.
    cache_ready: bool => crate::native_union::Rule::Text,
    /// The data root in use, when a store is open. Absent otherwise.
    data_root: Option<String> => crate::native_union::Rule::Text,
    /// Producer runs started, and callers served by a run someone else started (§8.2, C04).
    single_flight: SingleFlightCounts => crate::native_union::Rule::Text,
    /// Language-server starts, reuses and evictions in this daemon process.
    ///
    /// Published because gate C17 -- "inspection does not start an LSP unnecessarily" -- is only
    /// checkable against a counter. A caller reading `started` before and after a signature-only
    /// inspection can see for itself that nothing was started. Blueprint §14.3 asks for the same
    /// numbers as operational metrics.
    lsp: LspMetrics => crate::native_union::Rule::Text,
    /// HTTP cache outcomes and transferred bytes since this process started.
    fetch: FetchCounters => crate::native_union::Rule::Text,
    /// What this process has answered, and how completely.
    evidence: EvidenceCounters => crate::native_union::Rule::Text,
    /// How verification probes finished.
    verification: VerificationCounters => crate::native_union::Rule::Text,
    /// Native query counters and configured limits, absent when no runtime is open.
    native_queries: Option<NativeQueryCounters> => crate::native_union::Rule::Text,
    /// How long this process has been up, in seconds.
    ///
    /// Every counter beside it is scoped to that window. Without this an operator reading
    /// `fetch.misses = 3` cannot tell a quiet hour from a daemon that restarted a minute ago.
    uptime_seconds: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// The `service_status` result (§7.1).
pub struct StatusData {
    /// Component versions.
    versions: Versions => crate::native_union::Rule::Text,
    /// Which wire schema versions this daemon can speak.
    schema_compatibility: SchemaCompatibility => crate::native_union::Rule::Text,
    /// On-disk storage compatibility; independent of the MCP response envelope.
    snapshot_compatibility: SchemaCompatibility => crate::native_union::Rule::Text,
    /// Which execution profiles are actually usable here.
    sandbox: Sandbox => crate::native_union::Rule::Text,
    /// Evidence producers and whether each is installed.
    producers: Vec<ComponentStatus> => crate::native_union::Rule::Sequence,
    /// Optional capabilities and whether each is supported.
    features: Vec<ComponentStatus> => crate::native_union::Rule::Sequence,
    /// Job queue and cache health.
    health: Health => crate::native_union::Rule::Text,
}
}

crate::native_vocabulary! {
/// A concrete execution prerequisite; configuration presence never implies qualification.
pub enum ExecutionPrerequisite {
    EnabledProfile = "enabled_profile",
    ImmutableImage = "immutable_image",
    Qualification = "qualification",
    Cleanup = "cleanup",
}
}

crate::native_struct! {
pub struct ExecutionReadiness {
    ecosystem: crate::identity::Ecosystem => crate::native_union::Rule::Text,
    profile: crate::policy::ExecutionProfile => crate::native_union::Rule::Text,
    available: bool => crate::native_union::Rule::Text,
    image_id: Option<String> => crate::native_union::Rule::Text,
    prerequisites: Vec<ExecutionPrerequisite> => crate::native_union::Rule::Sequence,
    actions: Vec<super::RecoveryAction> => crate::native_union::Rule::Sequence,
}
}
