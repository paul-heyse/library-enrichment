//! Concrete isolated producer intent. Callers cannot supply process arguments or host paths.
use crate::policy::ExecutionProfile;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum ProbeMode {
    Compile,
    Typecheck,
    Runtime,
}

/// Exact consumer input, pinned to a snapshot before scheduling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VerifyRequest {
    pub context_id: String,
    #[serde(default)]
    pub snapshot_id: Option<String>,
    pub snippet: String,
    pub mode: ProbeMode,
    pub profile: ExecutionProfile,
    #[serde(default)]
    pub test_intent: Option<String>,
    #[serde(default)]
    pub max_bytes: Option<usize>,
}

impl VerifyRequest {
    /// Validate before admitting work. An operator's permissive sandbox flags never permit host execution.
    pub fn validate(&self, config: &crate::config::Config) -> Result<(), String> {
        self.validate_shape(config)?;
        if !self.profile.is_enabled(config) {
            return Err(format!("{} execution profile is not enabled", self.profile));
        }
        Ok(())
    }

    /// Validate caller intent independently of live producer readiness.
    pub fn validate_shape(&self, config: &crate::config::Config) -> Result<(), String> {
        if self.snippet.trim().is_empty()
            || self.snippet.len() > config.limits.verification_input_bytes
        {
            return Err(
                "snippet is empty or exceeds the configured verification input budget".into(),
            );
        }
        let required = if self.mode == ProbeMode::Runtime {
            ExecutionProfile::Runtime
        } else {
            ExecutionProfile::Build
        };
        if self.profile != required {
            return Err(format!(
                "{} requires locally enabled {required} policy",
                match self.mode {
                    ProbeMode::Runtime => "runtime",
                    _ => "compilation/typechecking",
                }
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum JobAction {
    #[default]
    Status,
    Wait,
    Cancel,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct JobRequest {
    pub job_id: String,
    pub action: JobAction,
    pub interest_token: Option<String>,
    pub wait_seconds: u64,
    pub max_bytes: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct JobData {
    pub job_id: String,
    pub state: crate::wire::JobState,
    pub stage: String,
    pub interest_token: Option<String>,
    pub active_interests: usize,
    pub submitted_at: String,
    pub updated_at: String,
    pub result: Option<JobResult>,
}

/// Compact terminal research outcome; the complete answer is directly retrievable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "RawJobResult")]
#[schemars(deny_unknown_fields, transform = terminal_conditionals)]
pub struct JobResult {
    pub outcome: crate::wire::Status,
    pub summary: String,
    pub context_id: Option<String>,
    pub snapshot_id: Option<String>,
    pub coverage: crate::wire::Coverage,
    pub delivery: crate::wire::DeliveryDescriptor,
    pub error: Option<crate::wire::ErrorDetail>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
struct RawJobResult {
    outcome: crate::wire::Status,
    summary: String,
    #[serde(deserialize_with = "crate::wire::required_option")]
    context_id: Option<String>,
    #[serde(deserialize_with = "crate::wire::required_option")]
    snapshot_id: Option<String>,
    coverage: crate::wire::Coverage,
    delivery: crate::wire::DeliveryDescriptor,
    #[serde(deserialize_with = "crate::wire::required_option")]
    error: Option<crate::wire::ErrorDetail>,
}

impl TryFrom<RawJobResult> for JobResult {
    type Error = &'static str;
    fn try_from(raw: RawJobResult) -> Result<Self, Self::Error> {
        if raw.outcome == crate::wire::Status::Pending
            || (raw.outcome == crate::wire::Status::Error) != raw.error.is_some()
        {
            return Err("terminal results require a completed outcome and exactly its error");
        }
        Ok(Self {
            outcome: raw.outcome,
            summary: raw.summary,
            context_id: raw.context_id,
            snapshot_id: raw.snapshot_id,
            coverage: raw.coverage,
            delivery: raw.delivery,
            error: raw.error,
        })
    }
}

fn terminal_conditionals(schema: &mut schemars::Schema) {
    schema.insert(
        "allOf".into(),
        serde_json::json!([
            {"properties": {"outcome": {"enum": ["ok", "partial", "error"]}}},
            {"if": {"properties": {"outcome": {"const": "error"}}},
             "then": {"properties": {"error": {"not": {"type": "null"}}}},
             "else": {"properties": {"error": {"type": "null"}}}}
        ]),
    );
}

impl From<&crate::wire::Envelope> for JobResult {
    fn from(value: &crate::wire::Envelope) -> Self {
        assert_ne!(
            value.status(),
            crate::wire::Status::Pending,
            "terminal research outcome"
        );
        Self {
            outcome: value.status(),
            summary: value.summary.clone(),
            context_id: value.context_id.clone(),
            snapshot_id: value.snapshot_id.clone(),
            coverage: value.coverage.clone(),
            delivery: value.delivery.clone(),
            error: value.error().cloned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum ProcessEnd {
    Exited,
    Deadline,
    OutputLimit,
    Cancelled,
}

/// Raw bounded process observation, independent of what a probe establishes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ProcessObservation {
    pub image_id: String,
    pub command: Vec<String>,
    pub started_at: String,
    pub finished_at: String,
    pub exit_code: Option<i32>,
    pub end: ProcessEnd,
    pub stdout: String,
    pub stderr: String,
    pub cleanup_confirmed: bool,
}

/// Scoped consumer verification; original static evidence remains independently readable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VerificationData {
    pub evidence_class: crate::wire::EvidenceClass,
    pub producer_runs: Vec<crate::producer::ProducerRun>,
    pub source_context_id: String,
    pub source_snapshot_id: String,
    pub derived_context: Option<crate::identity::Context>,
    pub derived_snapshot_id: Option<String>,
    pub environment: Option<crate::identity::Environment>,
    pub mode: ProbeMode,
    pub profile: ExecutionProfile,
    pub snippet_origin: String,
    pub test_intent: Option<String>,
    pub snippet_artifact_id: String,
    pub lock_artifact_id: Option<String>,
    pub result_artifact_id: String,
    pub observations: Vec<ProcessObservation>,
    pub limitations: Vec<String>,
}

#[cfg(test)]
mod research_contract_tests {
    use super::*;
    #[test]
    fn terminal_outcome_and_error_are_consistent() {
        let source: crate::wire::Envelope = serde_json::from_str(include_str!(
            "../../../contracts/research-v2/examples/ok.fixture.json"
        ))
        .expect("fixture");
        let valid = serde_json::to_value(JobResult::from(&source)).expect("serialize");
        assert!(serde_json::from_value::<JobResult>(valid.clone()).is_ok());
        for outcome in ["pending", "error"] {
            let mut invalid = valid.clone();
            invalid["outcome"] = serde_json::json!(outcome);
            assert!(serde_json::from_value::<JobResult>(invalid).is_err());
        }
        for field in ["context_id", "snapshot_id", "error"] {
            let mut invalid = valid.clone();
            invalid.as_object_mut().expect("object").remove(field);
            assert!(serde_json::from_value::<JobResult>(invalid).is_err());
        }
    }
}
