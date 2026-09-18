//! One declared request, effect and exact Arrow-output receipt for the trusted decoder.
use crate::{
    native_union::Rule,
    producer::rustdoc::facts::{self, Fact},
};
use std::path::PathBuf;

pub const PROTOCOL: &str = "native-rustdoc-arrow/4";
pub const INPUT_FILE: &str = "input.json";
pub const CONTROL_BYTES: usize = 16 * 1024;
pub const MEMORY_BYTES: u64 = 1024 * 1024 * 1024;
pub const DEADLINE_SECONDS: u64 = 30;
pub const STDERR_BYTES: usize = 4096;

crate::native_struct! { pub struct Request {
    protocol: String => Rule::Vocabulary(vec![PROTOCOL.into()]),
    artifact_id: String => Rule::ArtifactIdentity { digest: "sha256".into() },
    sha256: String => Rule::Sha256,
    bytes: u64 => Rule::UnsignedRange { min: 1, max: facts::MAX_BYTES },
    root: PathBuf => Rule::NonEmpty,
    deadline_seconds: u64 => Rule::UnsignedRange { min: 1, max: DEADLINE_SECONDS },
} }
impl Request {
    pub fn input(&self) -> PathBuf {
        self.root.join(INPUT_FILE)
    }
}

crate::native_struct! { pub struct StreamReceipt {
    fact: Fact => Rule::Text,
    digest: String => Rule::Sha256,
    bytes: u64 => Rule::UnsignedRange { min: 8, max: facts::MAX_BYTES },
    rows: u64 => Rule::UnsignedRange { min: 0, max: facts::MAX_ROWS },
} }

crate::native_struct! { pub struct Report {
    protocol: String => Rule::Vocabulary(vec![PROTOCOL.into()]),
    request_digest: String => Rule::Sha256,
    producer_revision: String => Rule::NonEmpty,
    streams: Vec<StreamReceipt> => Rule::SequenceBounds {
        min: Fact::ALL.len() as u64, max: Fact::ALL.len() as u64,
    },
} }

crate::native_struct! { pub struct Effect {
    grant_id: crate::identity::GrantId => Rule::Text,
    job_id: crate::identity::JobId => Rule::Text,
    policy_id: crate::identity::OperationPolicyId => Rule::Text,
    executable: PathBuf => Rule::NonEmpty,
    argv: Vec<String> => Rule::SequenceBounds { min: 0, max: 0 },
    environment: std::collections::BTreeMap<String, String> => Rule::Map,
    cwd: PathBuf => Rule::NonEmpty,
    memory_bytes: u64 => Rule::UnsignedRange { min: MEMORY_BYTES, max: MEMORY_BYTES },
    report_bytes: u64 => Rule::UnsignedRange { min: CONTROL_BYTES as u64, max: CONTROL_BYTES as u64 },
    stderr_bytes: u64 => Rule::UnsignedRange { min: STDERR_BYTES as u64, max: STDERR_BYTES as u64 },
    implementation: String => Rule::NonEmpty,
    request: Request => Rule::Text,
} }
