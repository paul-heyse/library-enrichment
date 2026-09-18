//! Concrete isolated producer intent. Callers cannot supply process arguments or host paths.
use crate::policy::ExecutionProfile;

pub mod environment;
pub mod facts;
pub mod producer;
pub mod rustdoc_decoder;
pub mod static_worker;

crate::native_vocabulary! {
    #[schemars(inline)]
    pub enum ProbeMode { Compile = "compile", Typecheck = "typecheck", Runtime = "runtime" }
}

crate::native_struct! {
/// Exact consumer input, pinned to a snapshot before scheduling.
pub struct VerifyRequest {
    context_id: crate::identity::ContextId => crate::native_union::Rule::Text,
    #[serde(default)]
    snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
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
    job_id: crate::identity::JobId => crate::native_union::Rule::Text,
    #[serde(default)]
    action: JobAction => crate::native_union::Rule::Text,
    #[serde(default)]
    interest_token: Option<crate::identity::InterestId> => crate::native_union::Rule::Text,
    #[serde(default)]
    wait_seconds: u64 => crate::native_union::Rule::UnsignedRange { min: 0, max: 10 },
    #[serde(default)]
    #[schemars(range(min = 1024))]
    max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
pub struct JobData {
    job_id: crate::identity::JobId => crate::native_union::Rule::Text,
    state: crate::wire::JobState => crate::native_union::Rule::Text,
    stage: String => crate::native_union::Rule::Text,
    interest_token: Option<crate::identity::InterestId> => crate::native_union::Rule::Text,
    active_interests: usize => crate::native_union::Rule::Text,
    submitted_at: crate::native_time::SubmissionTime => crate::native_union::Rule::Text,
    updated_at: crate::native_time::UpdateTime => crate::native_union::Rule::Text,
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
    context_id: Option<crate::identity::ContextId> => crate::native_union::Rule::Text,
    snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
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
        effect_id: crate::identity::ProcessEffectId => crate::native_union::Rule::Text,
        grant_id: crate::identity::GrantId => crate::native_union::Rule::Text,
        environment_id: Option<crate::identity::EnvironmentId> => crate::native_union::Rule::Text,
        snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
    },
    Qualification = "qualification" { definition_id: crate::identity::ProcessOperationId => crate::native_union::Rule::Text },
}
}

crate::native_struct! {
pub struct ProcessObservation {
    operation_id: crate::identity::ProcessOperationId => crate::native_union::Rule::Text,
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
/// Physical producer facts before normalization. Logs project this record for diagnostics;
/// publication consumes the native value directly, never parses the projected log back.
pub struct VerificationCapture {
    job_id: crate::identity::JobId => crate::native_union::Rule::Text,
    request: VerifyRequest => crate::native_union::Rule::Text,
    source_release: crate::identity::Release => crate::native_union::Rule::Text,
    source_snapshot: crate::identity::SnapshotId => crate::native_union::Rule::Text,
    environment: crate::identity::Environment => crate::native_union::Rule::Text,
    containment_identity: String => crate::native_union::Rule::NonEmpty,
    observations: Vec<ProcessObservation> => crate::native_union::Rule::SequenceBounds { min: 1, max: 64 },
    input_artifacts: Vec<crate::evidence::Artifact> => crate::native_union::Rule::SequenceBounds { min: 0, max: 4098 },
}
}

crate::native_struct! {
/// Scoped consumer verification; original static evidence remains independently readable.
pub struct VerificationData {
    evidence_class: crate::wire::EvidenceClass => crate::native_union::Rule::Text,
    producer_runs: Vec<crate::producer::ProducerRun> => crate::native_union::Rule::Sequence,
    source_context_id: crate::identity::ContextId => crate::native_union::Rule::Text,
    source_snapshot_id: crate::identity::SnapshotId => crate::native_union::Rule::Text,
    derived_context: Option<crate::identity::Context> => crate::native_union::Rule::Text,
    derived_snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
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
