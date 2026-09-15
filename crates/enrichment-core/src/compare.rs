//! Deterministic differences between immutable observations, not a compatibility proof.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::canonical;

pub mod page;

/// Independent axes of a release comparison.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Api,
    Docs,
    Configuration,
    ReleaseNotes,
    Examples,
    Relationships,
}

/// A set or field difference, not a compatibility verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Added,
    Removed,
    Changed,
}

/// Each value retains its own qualified source, including a legitimate absent observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Alternative {
    pub value: AlternativeValue,
    pub source: Option<crate::evidence::relational::FactSource>,
}

/// Complete value delivery, independent of the alternative's fact provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum AlternativeValue {
    Inline {
        value: Value,
    },
    Artifact {
        artifact: crate::wire::ArtifactHandle,
        size_bytes: u64,
        sha256: String,
    },
}

/// A changed key with independently paged observational alternatives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Change {
    pub change_id: String,
    pub scope: Scope,
    pub kind: ChangeKind,
    pub subject: String,
    pub before: Option<Vec<Alternative>>,
    pub after: Option<Vec<Alternative>>,
    pub interpretation: String,
    /// Values and their source references share this bounded alternative order.
    pub before_page: crate::wire::Page,
    pub after_page: crate::wire::Page,
}

/// Construct a stable change from selected observations.
#[must_use]
pub fn change(
    scope: Scope,
    key: &str,
    subject: &str,
    before: Option<Vec<Alternative>>,
    after: Option<Vec<Alternative>>,
) -> Change {
    let kind = if before.is_none() {
        ChangeKind::Added
    } else if after.is_none() {
        ChangeKind::Removed
    } else {
        ChangeKind::Changed
    };
    let interpretation = match (scope, kind) {
        (Scope::Api, ChangeKind::Added) => "API observed only on the after side; confirm coverage before treating this as an addition. Execution and project compatibility have not been established.",
        (Scope::Api, _) => "Observed API representation changed; producer rendering, including Infallible versus never-type (!), can differ without a source-level compatibility change. Verify consequential usage.",
        _ => "Evidence changed within this scope; this is not an executed behavior assertion.",
    }.into();
    Change {
        change_id: format!(
            "change_{}",
            canonical::digest_hex(&json!(["typed-comparison/2", scope, key, before, after]))
        ),
        scope,
        kind,
        subject: subject.into(),
        before,
        after,
        interpretation,
        before_page: crate::wire::Page::default(),
        after_page: crate::wire::Page::default(),
    }
}
