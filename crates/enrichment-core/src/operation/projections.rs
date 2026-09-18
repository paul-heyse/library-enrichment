//! Durable projection checkpoints and native rebuild command inputs.
use crate::{evidence::snapshot::DeltaBinding, native_union::Rule};

pub const SURFACES: [(&str, &str); 2] = [
    ("api_surface", "symbol_id"),
    ("fragment_surface", "fragment_id"),
];
crate::native_vocabulary! {
    pub enum ProjectionMode {
        Initial = "initial", Incremental = "incremental",
        RevisionRebuild = "revision_rebuild", SourceRebuild = "source_rebuild",
        HistoryRebuild = "history_rebuild", ExportRebuild = "export_rebuild",
    }
}
crate::native_struct! {
    pub struct ProjectionIdentity {
        snapshot_id: crate::identity::SnapshotId => crate::native_union::Rule::Text,
        revision: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub struct Checkpoint {
        projection_id: String => Rule::NonEmpty,
        snapshot_id: crate::identity::SnapshotId => Rule::ForeignKey {
            table: "snapshots".into(), field: vec!["snapshot_id".into()],
            scope: vec![crate::native_union::ScopeKey::exact(&["inputs"], &["publication", "tables"])],
        },
        revision: String => Rule::NonEmpty,
        sequence: u64 => Rule::Text,
        mode: ProjectionMode => Rule::Text,
        predecessor: Option<String> => Rule::NonEmpty,
        inputs: Vec<DeltaBinding> => Rule::SequenceBounds { min: 1, max: 32 },
        outputs: Vec<DeltaBinding> => Rule::SequenceBounds { min: SURFACES.len() as u64, max: SURFACES.len() as u64 },
        read_descriptors: Vec<ReadDescriptor> => Rule::SequenceBounds { min: 0, max: SURFACES.len() as u64 },
    }
}
crate::native_struct! {
    pub struct ReadDescriptor {
        binding: DeltaBinding => Rule::Text,
        namespace: ReadNamespace => Rule::Text,
        codec: String => Rule::Vocabulary(vec!["delta-immutable-cbor/1".into()]),
        definition: String => Rule::NonEmpty,
        options: std::collections::BTreeMap<String, Option<String>> => Rule::Map,
        digest: crate::native_digest::Sha256Digest => Rule::Text,
        bytes: crate::native_bytes::NativeBytes => Rule::BinaryBytes { max: crate::native_bytes::MAX_BYTES as u64 },
    }
}
crate::native_struct! {
    pub struct ReadNamespace {
        path: String => Rule::NonEmpty,
        directories: Vec<DirectoryIdentity> => Rule::SequenceBounds { min: 2, max: 2 },
    }
}
crate::native_struct! {
    pub struct DirectoryIdentity {
        device: u64 => Rule::Text,
        inode: u64 => Rule::Text,
        created_seconds: u64 => Rule::Text,
        created_nanos: u32 => Rule::UnsignedRange { min: 0, max: 999_999_999 },
    }
}
crate::native_struct! {
    pub struct ProjectionCommand {
        revision: String => Rule::NonEmpty,
        prior: Option<Checkpoint> => Rule::Text,
        inputs: Vec<DeltaBinding> => Rule::SequenceBounds { min: 1, max: 32 },
        export: bool => Rule::Text,
        history_available: bool => Rule::Text,
    }
}
crate::native_struct! {
    pub struct ProjectionDecision {
        mode: ProjectionMode => Rule::Text,
    }
}
