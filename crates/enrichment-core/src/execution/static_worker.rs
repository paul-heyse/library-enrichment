//! Exact static extractor request and launch, independent of research handler code.
use crate::{native_union::Rule, producer::python::WorkerRequest};
use std::collections::BTreeMap;

pub const MEMORY_BYTES: u64 = 1024 * 1024 * 1024;
pub const STDERR_BYTES: u64 = 16 * 1024;
pub const REQUEST_BYTES: usize = 4 * 1024 * 1024;

crate::native_struct! { pub struct Launch {
    executable: String => Rule::NonEmpty,
    argv: Vec<String> => Rule::SequenceBounds { min: 4, max: 4 },
    environment: BTreeMap<String, String> => Rule::Map,
    cwd: String => Rule::NonEmpty,
    output: String => Rule::MemberPath,
    output_bytes: u64 => Rule::UnsignedRange { min: 1, max: crate::producer::python::worker::MAX_BYTES },
    stderr_bytes: u64 => Rule::UnsignedRange { min: 1, max: STDERR_BYTES },
    deadline_seconds: u64 => Rule::UnsignedRange { min: 1, max: 600 },
    request: WorkerRequest => Rule::Text,
} }

crate::native_struct! { pub struct Effect {
    grant_id: crate::identity::GrantId => Rule::Text,
    job_id: crate::identity::JobId => Rule::Text,
    policy_id: crate::identity::OperationPolicyId => Rule::Text,
    package: String => Rule::NonEmpty,
    artifact_id: String => Rule::ArtifactIdentity { digest: "artifact_sha256".into() },
    artifact_sha256: String => Rule::Sha256,
    implementation: String => Rule::NonEmpty,
    inputs: crate::capsule_protocol::inventory::Inventory => Rule::Map,
    launch: Launch => Rule::Text,
} }
