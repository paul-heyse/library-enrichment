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
    #[serde(alias = "run")]
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
        if self.profile != required || !self.profile.is_enabled(config) {
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
pub struct JobData {
    pub job_id: String,
    pub state: crate::wire::JobState,
    pub stage: String,
    pub interest_token: Option<String>,
    pub active_interests: usize,
    pub submitted_at: String,
    pub updated_at: String,
    pub result: Option<crate::wire::Envelope>,
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
