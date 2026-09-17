//! Concrete isolated producer intent. Callers cannot supply process arguments or host paths.
use crate::policy::ExecutionProfile;

pub mod facts;

crate::native_vocabulary! {
    #[schemars(inline)]
    pub enum ProbeMode { Compile = "compile", Typecheck = "typecheck", Runtime = "runtime" }
}

crate::native_struct! {
/// Exact consumer input, pinned to a snapshot before scheduling.
pub struct VerifyRequest {
    context_id: String => crate::native_union::Rule::Text,
    #[serde(default)]
    snapshot_id: Option<String> => crate::native_union::Rule::Text,
    snippet: String => crate::native_union::Rule::Text,
    #[serde(default = "default_probe_mode")]
    mode: ProbeMode => crate::native_union::Rule::Text,
    #[serde(default = "default_probe_profile")]
    profile: ExecutionProfile => crate::native_union::Rule::Text,
    #[serde(default)]
    test_intent: Option<String> => crate::native_union::Rule::Text,
    #[serde(default)]
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
}

fn default_probe_mode() -> ProbeMode {
    ProbeMode::Typecheck
}
fn default_probe_profile() -> ExecutionProfile {
    ExecutionProfile::Build
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

crate::native_vocabulary! {
#[derive(Default)]
#[schemars(inline)]
pub enum JobAction {
    #[default]
    Status = "status",
    Wait = "wait",
    Cancel = "cancel",
}
}

crate::native_struct! {
#[derive(Default)]
pub struct JobRequest {
    #[schemars(length(min = 1))]
    job_id: String => crate::native_union::Rule::NonEmpty,
    #[serde(default)]
    action: JobAction => crate::native_union::Rule::Text,
    #[serde(default)]
    interest_token: Option<String> => crate::native_union::Rule::Text,
    #[serde(default)]
    #[schemars(range(max = 10))]
    wait_seconds: u64 => crate::native_union::Rule::Text,
    #[serde(default)]
    #[schemars(range(min = 1024))]
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
pub struct JobData {
    job_id: String => crate::native_union::Rule::Text,
    state: crate::wire::JobState => crate::native_union::Rule::Text,
    stage: String => crate::native_union::Rule::Text,
    interest_token: Option<String> => crate::native_union::Rule::Text,
    active_interests: usize => crate::native_union::Rule::Text,
    submitted_at: String => crate::native_union::Rule::Text,
    updated_at: String => crate::native_union::Rule::Text,
    result: Option<JobResult> => crate::native_union::Rule::Text,
}
}

crate::native_union! { @tag "status";
/// Only terminal states are representable; a failure always carries its typed diagnostic.
pub enum TerminalOutcome {
    Ok = "ok", Partial = "partial",
    Error = "error" { error: crate::wire::ErrorDetail => crate::native_union::Rule::Text },
}
}
crate::native_struct! {
/// Compact terminal research result with a directly retrievable complete answer.
pub struct JobResult {
    outcome: TerminalOutcome => crate::native_union::Rule::Text,
    summary: String => crate::native_union::Rule::Text,
    context_id: Option<String> => crate::native_union::Rule::Text,
    snapshot_id: Option<String> => crate::native_union::Rule::Text,
    coverage: crate::wire::Coverage => crate::native_union::Rule::Text,
    delivery: crate::wire::DeliveryDescriptor => crate::native_union::Rule::Text,
}
}

impl From<&crate::wire::Envelope> for JobResult {
    fn from(value: &crate::wire::Envelope) -> Self {
        assert_ne!(
            value.status(),
            crate::wire::Status::Pending,
            "terminal research outcome"
        );
        Self {
            outcome: match value.outcome().expect("valid terminal envelope") {
                crate::wire::Outcome::Ok { .. } => TerminalOutcome::Ok,
                crate::wire::Outcome::Partial { .. } => TerminalOutcome::Partial,
                crate::wire::Outcome::Error { error, .. } => TerminalOutcome::Error { error },
                crate::wire::Outcome::Pending { .. } => {
                    unreachable!("terminal outcome checked above")
                }
            },
            summary: value.summary.clone(),
            context_id: value.context_id.clone(),
            snapshot_id: value.snapshot_id.clone(),
            coverage: value.coverage.clone(),
            delivery: value.delivery.clone(),
        }
    }
}

crate::native_vocabulary! {
    #[schemars(inline)]
    pub enum ProcessEnd { Exited = "exited", Deadline = "deadline", OutputLimit = "output_limit", Cancelled = "cancelled" }
}

crate::native_union! {
/// Actual process authority, independent of the probe's semantic outcome.
pub enum ProcessAuthority {
    Command = "command" {
        effect_id: String => crate::native_union::Rule::NonEmpty,
        grant_id: String => crate::native_union::Rule::NonEmpty,
        environment_id: Option<String> => crate::native_union::Rule::Text,
        snapshot_id: Option<String> => crate::native_union::Rule::Text,
    },
    Qualification = "qualification" { definition_id: String => crate::native_union::Rule::NonEmpty },
}
}

crate::native_struct! {
pub struct ProcessObservation {
    operation_id: String => crate::native_union::Rule::Text,
    authority: ProcessAuthority => crate::native_union::Rule::Text,
    image_id: String => crate::native_union::Rule::Text,
    command: Vec<String> => crate::native_union::Rule::Sequence,
    started_at: crate::native_time::ObservationTime => crate::native_union::Rule::Text,
    finished_at: crate::native_time::ObservationTime => crate::native_union::Rule::Text,
    exit_code: Option<i32> => crate::native_union::Rule::Text,
    end: ProcessEnd => crate::native_union::Rule::Text,
    stdout: String => crate::native_union::Rule::Text,
    stderr: String => crate::native_union::Rule::Text,
    cleanup_confirmed: bool => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Scoped consumer verification; original static evidence remains independently readable.
pub struct VerificationData {
    evidence_class: crate::wire::EvidenceClass => crate::native_union::Rule::Text,
    producer_runs: Vec<crate::producer::ProducerRun> => crate::native_union::Rule::Sequence,
    source_context_id: String => crate::native_union::Rule::Text,
    source_snapshot_id: String => crate::native_union::Rule::Text,
    derived_context: Option<crate::identity::Context> => crate::native_union::Rule::Text,
    derived_snapshot_id: Option<String> => crate::native_union::Rule::Text,
    environment: Option<crate::identity::Environment> => crate::native_union::Rule::Text,
    mode: ProbeMode => crate::native_union::Rule::Text,
    profile: ExecutionProfile => crate::native_union::Rule::Text,
    snippet_origin: String => crate::native_union::Rule::Text,
    test_intent: Option<String> => crate::native_union::Rule::Text,
    snippet_artifact_id: String => crate::native_union::Rule::Text,
    lock_artifact_id: Option<String> => crate::native_union::Rule::Text,
    result_artifact_id: String => crate::native_union::Rule::Text,
    observations: Vec<ProcessObservation> => crate::native_union::Rule::Sequence,
    limitations: Vec<String> => crate::native_union::Rule::Sequence,
}
}

#[cfg(test)]
mod research_contract_tests {
    use super::*;
    #[test]
    fn terminal_outcome_and_error_are_consistent() {
        let source: crate::wire::Envelope =
            serde_json::from_str(include_str!("../../../tests/fixtures/wire/ok.fixture.json"))
                .expect("fixture");
        let valid = serde_json::to_value(JobResult::from(&source)).expect("serialize");
        assert!(serde_json::from_value::<JobResult>(valid.clone()).is_ok());
        for outcome in ["pending", "error"] {
            let mut invalid = valid.clone();
            invalid["outcome"] = serde_json::json!({"status":outcome});
            assert!(serde_json::from_value::<JobResult>(invalid).is_err());
        }
        for field in ["context_id", "snapshot_id", "outcome"] {
            let mut invalid = valid.clone();
            invalid.as_object_mut().expect("object").remove(field);
            assert!(serde_json::from_value::<JobResult>(invalid).is_err());
        }
    }
}
