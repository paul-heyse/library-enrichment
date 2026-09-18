//! Durable job records derive from the operation contracts.
use super::Arguments;
crate::native_vocabulary! { pub enum CleanupState { Owned = "owned", Settled = "settled", Unresolved = "unresolved" } }

crate::native_struct! {
pub struct Command {
    job_id: crate::identity::JobId => crate::native_union::Rule::Text,
    job_key: String => crate::native_union::Rule::Text,
    submitted_at: crate::native_time::SubmissionTime => crate::native_union::Rule::Text,
    operation_revision: String => crate::native_union::Rule::Text,
    policy_id: crate::identity::OperationPolicyId => crate::native_union::Rule::Text,
    policy_binding: crate::delta_reference::DeltaVersionRef => crate::native_union::Rule::Text,
    arguments: Arguments => crate::native_union::Rule::Text,
}
}
crate::native_struct! {
pub struct Resolution {
    release_id: crate::identity::ReleaseId => crate::native_union::Rule::Text,
    environment_id: crate::identity::EnvironmentId => crate::native_union::Rule::Text,
    context_id: crate::identity::ContextId => crate::native_union::Rule::Text,
    attempt_id: crate::identity::AttemptId => crate::native_union::Rule::Text,
    input_artifact_ids: Vec<String> => crate::native_union::Rule::SequenceBounds { min: 1, max: 8192 },
    result_artifact_id: String => crate::native_union::Rule::NonEmpty,
}
}
crate::native_struct! {
pub struct JobSnapshot {
    job_id: crate::identity::JobId => crate::native_union::Rule::Text,
    key: String => crate::native_union::Rule::NonEmpty,
    specification: Arguments => crate::native_union::Rule::Text,
    state: crate::wire::JobState => crate::native_union::Rule::Text,
    stage: String => crate::native_union::Rule::Text,
    interests: Vec<crate::identity::InterestId> => crate::native_union::Rule::Set,
    active_interests: usize => crate::native_union::Rule::Text,
    submitted_at: crate::native_time::SubmissionTime => crate::native_union::Rule::Text,
    updated_at: crate::native_time::UpdateTime => crate::native_union::Rule::Text,
    result_artifact: Option<crate::evidence::Artifact> => crate::native_union::Rule::Text,
    resolution: Option<Resolution> => crate::native_union::Rule::Text,
}
}
crate::native_struct! {
pub struct Transition {
    job_id: crate::identity::JobId => crate::native_union::Rule::foreign_key("commands", "job_id"),
    sequence: u64 => crate::native_union::Rule::Text,
    predecessor: Option<u64> => crate::native_union::Rule::Text,
    state: crate::wire::JobState => crate::native_union::Rule::Text,
    stage: String => crate::native_union::Rule::Text,
    updated_at: crate::native_time::UpdateTime => crate::native_union::Rule::Text,
    result: Option<crate::evidence::Artifact> => crate::native_union::Rule::Text,
    resolution: Option<Resolution> => crate::native_union::Rule::Text,
}
}
crate::native_struct! {
pub struct Interest {
    interest_id: crate::identity::InterestId => crate::native_union::Rule::Text,
    job_id: crate::identity::JobId => crate::native_union::Rule::foreign_key("commands", "job_id"),
    sequence: u64 => crate::native_union::Rule::Text,
    attached: bool => crate::native_union::Rule::Text,
}
}
crate::native_struct! {
pub struct Claim {
    job_id: crate::identity::JobId => crate::native_union::Rule::ForeignKey {
        table: "commands".into(), field: vec!["job_id".into()], scope: vec![
            crate::native_union::ScopeKey::exact(&["job_key"], &["job_key"]),
            crate::native_union::ScopeKey::exact(&["policy_id"], &["policy_id"]),
        ],
    },
    owner: String => crate::native_union::Rule::Text,
    attempt_id: crate::identity::AttemptId => crate::native_union::Rule::Text,
    fence: u64 => crate::native_union::Rule::Text,
    sequence: u64 => crate::native_union::Rule::Text,
    lease_expires_at: crate::native_time::ExpiryTime => crate::native_union::Rule::Text,
    cleanup_state: CleanupState => crate::native_union::Rule::Text,
    job_key: String => crate::native_union::Rule::Text,
    policy_id: crate::identity::OperationPolicyId => crate::native_union::Rule::Text,
    grant_id: crate::identity::GrantId => crate::native_union::Rule::Text,
    profile: crate::policy::ExecutionProfile => crate::native_union::Rule::Text,
    ecosystem: crate::identity::Ecosystem => crate::native_union::Rule::Text,
    environment_id: Option<crate::identity::EnvironmentId> => crate::native_union::Rule::Text,
    snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
    image_id: Option<String> => crate::native_union::Rule::Text,
    cleanup_observer: Option<String> => crate::native_union::Rule::Text,
    cleanup_confirmed_at: Option<crate::native_time::ObservationTime> => crate::native_union::Rule::Text,
}
}

crate::native_struct! { pub struct Counts { queued: u64 => crate::native_union::Rule::Text, running: u64 => crate::native_union::Rule::Text } }

crate::native_struct! { pub struct Effect {
    grant_id: crate::identity::GrantId => crate::native_union::Rule::Text,
    job_id: crate::identity::JobId => crate::native_union::Rule::Text,
    policy_id: crate::identity::OperationPolicyId => crate::native_union::Rule::Text,
    profile: crate::policy::ExecutionProfile => crate::native_union::Rule::Text,
    ecosystem: crate::identity::Ecosystem => crate::native_union::Rule::Text,
    environment_id: Option<crate::identity::EnvironmentId> => crate::native_union::Rule::Text,
    snapshot_id: Option<crate::identity::SnapshotId> => crate::native_union::Rule::Text,
    image_id: String => crate::native_union::Rule::NonEmpty,
    acquisition: bool => crate::native_union::Rule::Text,
    operation_id: crate::identity::ProcessOperationId => crate::native_union::Rule::Text,
    operation_binding: crate::delta_reference::DeltaVersionRef => crate::native_union::Rule::Text,
} }
