//! Deterministic differences between immutable observations, not a compatibility proof.

use serde_json::Value;

pub mod page;

crate::native_vocabulary! {
/// Independent axes of a release comparison.
#[derive(PartialOrd,Ord)]
pub enum Scope {
    Api = "api",
    Docs = "docs",
    Configuration = "configuration",
    ReleaseNotes = "release_notes",
    Examples = "examples",
    Relationships = "relationships",
}
}

crate::native_vocabulary! {
/// A set or field difference, not a compatibility verdict.
pub enum ChangeKind {
    Added = "added",
    Removed = "removed",
    Changed = "changed",
}
}

crate::native_struct! {
/// Each value retains its own qualified source, including a legitimate absent observation.
pub struct Alternative {
    value: AlternativeValue => crate::native_union::Rule::Text,
    source: Option<crate::evidence::relational::FactSource> => crate::native_union::Rule::Text,
}
}

crate::native_union! { @tag "mode";
/// A final encoded comparison cell or its exact immutable artifact.
pub enum AlternativeValue {
    Inline = "inline" { value: Value => crate::native_union::Rule::Text },
    Artifact = "artifact" {
        artifact: crate::wire::ArtifactHandle => crate::native_union::Rule::Text,
        size_bytes: u64 => crate::native_union::Rule::Text,
        sha256: String => crate::native_union::Rule::NonEmpty,
    },
}
}

crate::native_struct! {
/// A changed key with independently paged observational alternatives.
pub struct Change {
    change_id: String => crate::native_union::Rule::Text,
    scope: Scope => crate::native_union::Rule::Text,
    kind: ChangeKind => crate::native_union::Rule::Text,
    subject: String => crate::native_union::Rule::Text,
    before: Option<Vec<Alternative>> => crate::native_union::Rule::Sequence,
    after: Option<Vec<Alternative>> => crate::native_union::Rule::Sequence,
    interpretation: String => crate::native_union::Rule::Text,
    /// Values and their source references share this bounded alternative order.
    before_page: crate::wire::Page => crate::native_union::Rule::Text,
    after_page: crate::wire::Page => crate::native_union::Rule::Text,
}
}
