//! Typed Arrow observations for native runtime, operation and failure diagnostics.
use crate::native_union::{Cell, NativeStruct, Rule};
pub mod kernel;
crate::native_struct! {
#[derive(Default)]
pub struct InventorySummary {
    relations: Vec<String> => Rule::Set,
    nested_fields: usize => Rule::Text,
    truncated: bool => Rule::Text,
}
}

crate::native_struct! {
pub struct OperationDescriptor {
    method: String => Rule::Text,
    request_digest: String => Rule::Text,
    policy_digest: String => Rule::Text,
}
}

crate::native_struct! {
pub struct SnapshotBinding {
    snapshot_id: crate::identity::SnapshotId => Rule::Text,
    context_id: crate::identity::ContextId => Rule::Text,
    environment_id: crate::identity::EnvironmentId => Rule::Text,
    control: u64 => Rule::Text,
    manifest_digest: String => Rule::Text,
    projection_version: String => Rule::Text,
}
}

crate::native_struct! {
pub struct OperationBinding {
    request: OperationDescriptor => Rule::Text,
    snapshots: Vec<SnapshotBinding> => Rule::Sequence,
    retained_byte_limit: usize => Rule::Text,
    artifact_byte_limit: u64 => Rule::Text,
    deadline_millis: u64 => Rule::Text,
}
}

crate::native_struct! {
pub struct Metric {
    node: usize => Rule::Text,
    name: String => Rule::Text,
    partition: Option<usize> => Rule::Text,
    labels: Vec<Label> => Rule::Sequence,
    /// Counts/gauges/time values are copied, never live shared counters.
    value: Option<usize> => Rule::Text,
    unit: String => Rule::Text,
    pruned: Option<usize> => Rule::Text,
    matched: Option<usize> => Rule::Text,
    part: Option<usize> => Rule::Text,
    total: Option<usize> => Rule::Text,
}
}

crate::native_struct! {
pub struct QueryDiagnostics {
    catalog: InventorySummary => Rule::Text,
    rules: Vec<RuleTransition> => Rule::Sequence,
    rules_truncated: bool => Rule::Text,
    query_id: u64 => Rule::Text,
    operation_id: Option<String> => Rule::Text,
    binding: Option<OperationBinding> => Rule::Text,
    relations: Vec<String> => Rule::Set,
    functions: Vec<String> => Rule::Set,
    inventory_truncated: bool => Rule::Text,
    logical: String => Rule::Text,
    stage: String => Rule::Text,
    family: Option<String> => Rule::Text,
    analyzed: String => Rule::Text,
    analysis_micros: u64 => Rule::Text,
    optimization_micros: u64 => Rule::Text,
    physical: String => Rule::Text,
    truncated: bool => Rule::Text,
    metrics_truncated: bool => Rule::Text,
    /// False includes failed execution, timeout and dropped requests; counters may be partial.
    completed: bool => Rule::Text,
    /// Time waiting for the shared query permit before native planning starts.
    queue_micros: u64 => Rule::Text,
    planning_micros: u64 => Rule::Text,
    elapsed_micros: u64 => Rule::Text,
    output_rows: usize => Rule::Text,
    output_arrow_bytes: usize => Rule::Text,
    metrics: Vec<Metric> => Rule::Sequence,
}
}

crate::native_struct! {
pub struct RuleTransition {
    phase: String => Rule::Text,
    rule: String => Rule::Text,
    /// Unknown if the plan exceeded the bounded fingerprint traversal.
    changed: Option<bool> => Rule::Text,
}
}

crate::native_struct! {
#[derive(Default)]
pub struct Summary {
    executions: u64 => Rule::Text,
    completed: u64 => Rule::Text,
    incomplete: u64 => Rule::Text,
    planning_micros: u64 => Rule::Text,
    /// Sum of query elapsed durations; concurrent durations overlap and are not wall time.
    elapsed_micros: u64 => Rule::Text,
    materialization_fills: u64 => Rule::Text,
    materialization_spill_bytes: u64 => Rule::Text,
    materialization_reads: u64 => Rule::Text,
    queue_micros: u64 => Rule::Text,
    output_rows: u64 => Rule::Text,
    output_arrow_bytes: u64 => Rule::Text,
}
}
crate::native_struct! {
pub struct OperationDiagnostics {
    operation_id: String => Rule::Text,
    request: OperationDescriptor => Rule::Text,
    snapshots: Vec<SnapshotBinding> => Rule::Sequence,
    owned_lifetime_micros: u64 => Rule::Text,
    retained_output_bytes: usize => Rule::Text,
    artifact_bytes: usize => Rule::Text,
    native: Summary => Rule::Text,
}
}

crate::native_struct! {
pub struct OperationEnd {
    operation_id: String => Rule::Text,
    request: OperationDescriptor => Rule::Text,
    snapshots: Vec<SnapshotBinding> => Rule::Sequence,
    owned_lifetime_micros: u64 => Rule::Text,
    retained_output_bytes: usize => Rule::Text,
    artifact_bytes: usize => Rule::Text,
}
}

crate::native_struct! {
pub struct Label {
    name: String => Rule::Text,
    value: String => Rule::Text,
}
}

crate::native_struct! {
/// The recovery payload is an opaque MCP wire value. Cause, scope and bounds are native fields.
pub struct Failure {
    cause: crate::wire::DiagnosticCause => Rule::Text,
    stage: String => Rule::Text,
    affected_ids: Vec<String> => Rule::Set,
    rule: Option<String> => Rule::Text,
    observed: Option<u64> => Rule::Text,
    allowed: Option<u64> => Rule::Text,
    correlation_id: Option<String> => Rule::Text,
    recovery_payload: String => Rule::Json,
}
}
impl TryFrom<crate::wire::Diagnostic> for Failure {
    type Error = serde_json::Error;
    fn try_from(d: crate::wire::Diagnostic) -> Result<Self, Self::Error> {
        Ok(Self {
            cause: d.cause,
            stage: d.stage,
            affected_ids: d.affected_ids,
            rule: d.rule,
            observed: d.observed,
            allowed: d.allowed,
            correlation_id: d.correlation_id,
            recovery_payload: serde_json::to_string(&d.actions)?,
        })
    }
}

crate::native_vocabulary! {
    pub enum CacheOutcome { Hit = "hit", Revalidated = "revalidated", Miss = "miss" }
}
crate::native_vocabulary! {
    pub enum ProbeOutcome { Succeeded = "succeeded", Failed = "failed", Unresolved = "unresolved" }
}
crate::native_vocabulary! {
    pub enum MaterializationFamily {
        SearchIndex = "search_index", ComparisonKeys = "comparison_keys",
        OverviewChildren = "overview_children", OverviewNamespaces = "overview_namespaces"
    }
}
crate::native_union! {
    pub enum MaterializationActivity {
        Fill = "fill", Wait = "wait", Failed = "failed", Cancelled = "cancelled",
        Ready = "ready" { spill_bytes: u64 => Rule::Text },
        Read = "read" { reserved_bytes: usize => Rule::Text },
        ReaderReleased = "reader_released" { reserved_bytes: usize => Rule::Text },
    }
}
crate::native_struct! {
    pub struct MaterializationObservation {
        binding: u64 => Rule::Text,
        family: MaterializationFamily => Rule::Text,
        activity: MaterializationActivity => Rule::Text,
    }
}
crate::native_union! {
    /// Mechanically captured service outcomes. Native aggregates own their interpretation.
    pub enum ServiceObservation {
        Fetch = "fetch" { outcome: CacheOutcome => Rule::Text, transferred: u64 => Rule::Text },
        FetchFailure = "fetch_failure",
        Probe = "probe" { outcome: ProbeOutcome => Rule::Text },
        Response = "response" {
            method: String => Rule::NonEmpty,
            status: Option<crate::wire::Status> => Rule::Text,
            has_gap: bool => Rule::Text,
            elapsed_micros: u64 => Rule::Text,
            bytes: u64 => Rule::Text,
        },
    }
}
crate::native_struct! {
    #[derive(Default)]
    pub struct ServiceCounters {
        fetch: crate::wire::status::FetchCounters => Rule::Text,
        evidence: crate::wire::status::EvidenceCounters => Rule::Text,
        verification: crate::wire::status::VerificationCounters => Rule::Text,
    }
}

crate::native_union! {
    pub enum EventPayload {
        Query = "query" { value: Box<QueryDiagnostics> => Rule::Text },
        Operation = "operation" { value: OperationEnd => Rule::Text },
        Failure = "failure" { value: Failure => Rule::Text },
        Materialization = "materialization" { value: MaterializationObservation => Rule::Text },
        Service = "service" { value: ServiceObservation => Rule::Text },
        Kernel = "kernel" { value: kernel::Observation => Rule::Text },
    }
}
crate::native_struct! {
    pub struct Event {
        runtime_id: String => Rule::NonEmpty,
        sequence: u64 => Rule::Text,
        recorded_at: crate::native_time::EventTime => Rule::Text,
        operation_id: Option<String> => Rule::Text,
        payload: EventPayload => Rule::Text,
    }
}

pub mod schema {
    use super::*;
    use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
    use std::sync::Arc;
    pub fn summary_fields() -> Vec<Field> {
        Summary::fields()
            .iter()
            .map(|f| f.as_ref().clone())
            .collect()
    }
    pub fn query() -> DataType {
        QueryDiagnostics::data_type()
    }
    pub fn operation() -> DataType {
        OperationEnd::data_type()
    }
    pub fn events() -> SchemaRef {
        Arc::new(Schema::new(Event::fields()))
    }
}
