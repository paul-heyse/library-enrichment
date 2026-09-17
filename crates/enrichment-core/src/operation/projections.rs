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
        snapshot_id: String => Rule::NonEmpty,
        revision: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub struct Checkpoint {
        projection_id: String => Rule::NonEmpty,
        snapshot_id: String => Rule::NonEmpty,
        revision: String => Rule::NonEmpty,
        sequence: u64 => Rule::Text,
        mode: ProjectionMode => Rule::Text,
        predecessor: Option<String> => Rule::NonEmpty,
        inputs: Vec<DeltaBinding> => Rule::SequenceBounds { min: 1, max: 32 },
        outputs: Vec<DeltaBinding> => Rule::SequenceBounds { min: SURFACES.len() as u64, max: SURFACES.len() as u64 },
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
