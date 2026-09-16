//! Typed Arrow observations for native runtime, operation and failure diagnostics.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct InventorySummary {
    pub relations: Vec<String>,
    pub nested_fields: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationDescriptor {
    pub method: String,
    pub request_digest: String,
    pub policy_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotBinding {
    pub snapshot_id: String,
    pub context_id: String,
    pub environment_id: String,
    pub control: u64,
    pub manifest_digest: String,
    pub projection_version: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationBinding {
    pub request: OperationDescriptor,
    pub snapshots: Vec<SnapshotBinding>,
    pub retained_byte_limit: usize,
    pub artifact_byte_limit: u64,
    pub deadline_millis: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    pub node: usize,
    pub name: String,
    pub partition: Option<usize>,
    pub labels: Vec<Label>,
    /// Counts/gauges/time values are copied, never live shared counters.
    pub value: Option<usize>,
    pub unit: String,
    pub pruned: Option<usize>,
    pub matched: Option<usize>,
    pub part: Option<usize>,
    pub total: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryDiagnostics {
    pub catalog: InventorySummary,
    pub rules: Vec<RuleTransition>,
    pub rules_truncated: bool,
    pub query_id: u64,
    pub operation_id: Option<String>,
    pub binding: Option<OperationBinding>,
    pub relations: Vec<String>,
    pub functions: Vec<String>,
    pub inventory_truncated: bool,
    pub logical: String,
    pub stage: String,
    pub family: Option<String>,
    pub analyzed: String,
    pub analysis_micros: u64,
    pub optimization_micros: u64,
    pub physical: String,
    pub truncated: bool,
    pub metrics_truncated: bool,
    /// False includes failed execution, timeout and dropped requests; counters may be partial.
    pub completed: bool,
    /// Time waiting for the shared query permit before native planning starts.
    #[serde(default)]
    pub queue_micros: u64,
    pub planning_micros: u64,
    pub elapsed_micros: u64,
    pub output_rows: usize,
    pub output_arrow_bytes: usize,
    pub metrics: Vec<Metric>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleTransition {
    pub phase: String,
    pub rule: String,
    /// Unknown if the plan exceeded the bounded fingerprint traversal.
    pub changed: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Summary {
    pub executions: u64,
    pub completed: u64,
    pub incomplete: u64,
    pub planning_micros: u64,
    /// Sum of query elapsed durations; concurrent durations overlap and are not wall time.
    pub elapsed_micros: u64,
    pub index_materializations: u64,
    pub index_spill_bytes: u64,
    pub index_reads: u64,
    pub queue_micros: u64,
    pub output_rows: u64,
    pub output_arrow_bytes: u64,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationDiagnostics {
    pub operation_id: String,
    pub request: OperationDescriptor,
    pub snapshots: Vec<SnapshotBinding>,
    pub owned_lifetime_micros: u64,
    pub retained_output_bytes: usize,
    pub artifact_bytes: usize,
    pub native: Summary,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationEnd {
    pub operation_id: String,
    pub request: OperationDescriptor,
    pub snapshots: Vec<SnapshotBinding>,
    pub owned_lifetime_micros: u64,
    pub retained_output_bytes: usize,
    pub artifact_bytes: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub value: String,
}

/// The recovery payload is an opaque MCP wire value. Cause, scope and bounds are native fields.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Failure {
    pub cause: crate::wire::DiagnosticCause,
    pub stage: String,
    pub affected_ids: Vec<String>,
    pub rule: Option<String>,
    pub observed: Option<u64>,
    pub allowed: Option<u64>,
    pub correlation_id: Option<String>,
    pub recovery_payload: String,
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

pub mod schema {
    use arrow::datatypes::{DataType as D, Field, Schema, SchemaRef, TimeUnit};
    use std::sync::Arc;
    fn field(name: &str, kind: D, nullable: bool) -> Field {
        Field::new(name, kind, nullable)
    }
    fn text(name: &str) -> Field {
        field(name, D::Utf8, false)
    }
    fn optional(name: &str) -> Field {
        field(name, D::Utf8, true)
    }
    fn number(name: &str) -> Field {
        field(name, D::UInt64, false)
    }
    fn boolean(name: &str) -> Field {
        field(name, D::Boolean, false)
    }
    fn record(fields: Vec<Field>) -> D {
        D::Struct(fields.into())
    }
    fn list(kind: D) -> D {
        D::List(Arc::new(field("item", kind, false)))
    }
    fn strings(name: &str) -> Field {
        field(name, list(D::Utf8), false)
    }
    fn descriptor() -> D {
        record(vec![
            text("method"),
            text("request_digest"),
            text("policy_digest"),
        ])
    }
    fn snapshot() -> D {
        record(vec![
            text("snapshot_id"),
            text("context_id"),
            text("environment_id"),
            number("control"),
            text("manifest_digest"),
            text("projection_version"),
        ])
    }
    fn binding() -> D {
        record(vec![
            field("request", descriptor(), false),
            field("snapshots", list(snapshot()), false),
            number("retained_byte_limit"),
            number("artifact_byte_limit"),
            number("deadline_millis"),
        ])
    }
    pub fn summary_fields() -> Vec<Field> {
        [
            "executions",
            "completed",
            "incomplete",
            "planning_micros",
            "elapsed_micros",
            "index_materializations",
            "index_spill_bytes",
            "index_reads",
            "queue_micros",
            "output_rows",
            "output_arrow_bytes",
        ]
        .map(number)
        .to_vec()
    }
    fn metric() -> D {
        let mut fields = vec![
            number("node"),
            text("name"),
            field("partition", D::UInt64, true),
            field(
                "labels",
                list(record(vec![text("name"), text("value")])),
                false,
            ),
            text("unit"),
        ];
        fields.extend(
            ["value", "pruned", "matched", "part", "total"]
                .map(|name| field(name, D::UInt64, true)),
        );
        record(fields)
    }
    pub fn query() -> D {
        record(vec![
            field(
                "catalog",
                record(vec![
                    strings("relations"),
                    number("nested_fields"),
                    boolean("truncated"),
                ]),
                false,
            ),
            field(
                "rules",
                list(record(vec![
                    text("phase"),
                    text("rule"),
                    field("changed", D::Boolean, true),
                ])),
                false,
            ),
            boolean("rules_truncated"),
            number("query_id"),
            optional("operation_id"),
            field("binding", binding(), true),
            strings("relations"),
            strings("functions"),
            boolean("inventory_truncated"),
            text("logical"),
            text("stage"),
            optional("family"),
            text("analyzed"),
            number("analysis_micros"),
            number("optimization_micros"),
            text("physical"),
            boolean("truncated"),
            boolean("metrics_truncated"),
            boolean("completed"),
            number("queue_micros"),
            number("planning_micros"),
            number("elapsed_micros"),
            number("output_rows"),
            number("output_arrow_bytes"),
            field("metrics", list(metric()), false),
        ])
    }
    pub fn operation() -> D {
        record(vec![
            text("operation_id"),
            field("request", descriptor(), false),
            field("snapshots", list(snapshot()), false),
            number("owned_lifetime_micros"),
            number("retained_output_bytes"),
            number("artifact_bytes"),
        ])
    }
    fn failure() -> D {
        record(vec![
            text("cause"),
            text("stage"),
            strings("affected_ids"),
            optional("rule"),
            field("observed", D::UInt64, true),
            field("allowed", D::UInt64, true),
            optional("correlation_id"),
            text("recovery_payload"),
        ])
    }
    pub fn events() -> SchemaRef {
        Arc::new(Schema::new(vec![
            text("runtime_id"),
            number("sequence"),
            field(
                "recorded_at",
                D::Timestamp(TimeUnit::Microsecond, Some("UTC".into())),
                false,
            ),
            text("kind"),
            optional("operation_id"),
            field("query", query(), true),
            field("operation", operation(), true),
            field("failure", failure(), true),
            field("index_bytes", D::UInt64, true),
        ]))
    }
}
