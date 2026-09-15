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

/// Source references survive even when the inline before/after value is paged out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Change {
    pub change_id: String,
    pub scope: Scope,
    pub kind: ChangeKind,
    pub subject: String,
    pub before: Option<Value>,
    pub after: Option<Value>,
    pub interpretation: String,
    pub before_sources: Vec<crate::evidence::relational::FactSource>,
    pub after_sources: Vec<crate::evidence::relational::FactSource>,
}

/// Construct a stable change from selected observations.
#[must_use]
pub fn change(
    scope: Scope,
    key: &str,
    subject: &str,
    before: Option<Value>,
    after: Option<Value>,
) -> Change {
    let kind = if before.is_none() {
        ChangeKind::Added
    } else if after.is_none() {
        ChangeKind::Removed
    } else {
        ChangeKind::Changed
    };
    let interpretation = match (scope, kind) {
        (Scope::Api, ChangeKind::Added) => "Additive observed API; execution and project compatibility have not been established.",
        (Scope::Api, _) => "Potentially breaking observed API change; check environment confounders and verify consequential usage.",
        _ => "Evidence changed within this scope; this is not an executed behavior assertion.",
    }.into();
    Change {
        change_id: format!(
            "change_{}",
            canonical::digest_hex(&json!(["typed-comparison/1", scope, key, before, after]))
        ),
        scope,
        kind,
        subject: subject.into(),
        before,
        after,
        interpretation,
        before_sources: Vec::new(),
        after_sources: Vec::new(),
    }
}
