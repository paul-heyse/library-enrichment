//! Coverage, freshness, evidence fragments and artifact handles (blueprint §6.2, §7.2).

use std::collections::BTreeSet;

use super::ids::ArtifactUri;

crate::native_struct! {
/// What was actually looked at, and what was not.
///
/// `ok` means successful within this scope, never complete knowledge of a library. An empty
/// `search_evidence` result with `indexed: ["public_api"]` is a valid negative answer; the same
/// result with `missing: ["public_api"]` is an evidence gap. The two must never collapse.
pub struct Coverage {
    /// When present, additional limitation text is retained in the exact coverage section.
    /// Scope, assessments, indexed and missing remain the authoritative inline assessment.
    #[serde(default)]
    details: Option<super::research::RecoveryAction> => crate::native_union::Rule::Text,
    /// Requested evidence scopes assessed against qualified native facts.
    assessments: Vec<ScopeAssessment> => crate::native_union::Rule::Sequence,
    /// What the result claims to cover, in prose.
    scope: String => crate::native_union::Rule::Text,
    /// Evidence kinds that were successfully indexed.
    indexed: BTreeSet<String> => crate::native_union::Rule::Set,
    /// Evidence kinds that were expected but are absent.
    missing: BTreeSet<String> => crate::native_union::Rule::Set,
    /// Known reasons this result may not generalize.
    limitations: Vec<String> => crate::native_union::Rule::Sequence,
}
}

crate::native_vocabulary! {
/// Unknown means no qualified coverage fact exists; it is distinct from a declared gap.
pub enum ScopeState {
    Indexed = "indexed",
    Partial = "partial",
    Missing = "missing",
    Unknown = "unknown",
}
}

crate::native_struct! {
pub struct ScopeAssessment {
    snapshot_id: crate::identity::SnapshotId => crate::native_union::Rule::Text,
    subject: crate::evidence::relational::SubjectRef => crate::native_union::Rule::Text,
    kind: crate::evidence::EvidenceKind => crate::native_union::Rule::Text,
    state: ScopeState => crate::native_union::Rule::Text,
    /// A qualified fact establishing the selected state. Historical attempts remain retained.
    witness_id: Option<String> => crate::native_union::Rule::Text,
}
}

impl Coverage {
    /// No assessment has established any evidence kind yet.
    #[must_use]
    pub fn unassessed(scope: impl Into<String>) -> Self {
        Self {
            details: None,
            assessments: Vec::new(),
            scope: scope.into(),
            indexed: BTreeSet::new(),
            missing: BTreeSet::new(),
            limitations: Vec::new(),
        }
    }

    /// Project the already evaluated scope states; do not infer coverage from payload rows.
    pub fn refresh_kinds(&mut self) {
        self.indexed.clear();
        self.missing.clear();
        for assessment in &self.assessments {
            if assessment.state == ScopeState::Indexed {
                self.indexed.insert(assessment.kind.as_str().into());
            } else {
                self.missing.insert(assessment.kind.as_str().into());
            }
        }
        self.indexed.retain(|kind| !self.missing.contains(kind));
    }

    #[must_use]
    pub fn complete(&self) -> bool {
        !self.assessments.is_empty()
            && self
                .assessments
                .iter()
                .all(|item| item.state == ScopeState::Indexed)
    }
}

crate::native_vocabulary! {
    /// How well the evidence's source version matches the version that was asked about.
    ///
    /// The values are, in order: `exact`, `compatible_claimed`, `mismatched`, `unknown`.
    /// `compatible_claimed` is a claim by the source, not a verified fact.
    ///
    /// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
    #[derive(Hash)]
    #[schemars(inline)]
    pub enum SourceVersionMatch { Exact = "exact", CompatibleClaimed = "compatible_claimed", Mismatched = "mismatched", Unknown = "unknown" }
}

crate::native_struct! {
/// Registry freshness. Blueprint §3.3: freshness is not one timestamp.
pub struct Freshness {
    /// When the registry was last consulted, or `null` if it was not consulted at all.
    registry_checked_at: Option<crate::native_time::AcquisitionTime> => crate::native_union::Rule::Text,
    /// How the evidence's source version relates to the requested one.
    source_version_match: SourceVersionMatch => crate::native_union::Rule::Text,
    /// Whether "this is the latest release" was actually revalidated. A cache hit alone is not
    /// evidence that a release is still latest.
    latest_verified: bool => crate::native_union::Rule::Text,
}
}

crate::native_vocabulary! {
    /// The six epistemic classes (blueprint §6.2).
    ///
    /// These are categories, not a confidence scale. An API signature can be `compiler_derived`
    /// while "this replaces our orchestration layer" is `agent_inferred`, and the two never merge.
    ///
    /// The values are, in order: `declared`, `statically_extracted`, `compiler_derived`,
    /// `typechecker_observed`, `runtime_observed`, `agent_inferred`.
    ///
    /// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
    #[derive(Hash)]
    #[schemars(inline)]
    pub enum EvidenceClass { Declared = "declared", StaticallyExtracted = "statically_extracted", CompilerDerived = "compiler_derived", TypecheckerObserved = "typechecker_observed", RuntimeObserved = "runtime_observed", AgentInferred = "agent_inferred" }
}

crate::native_struct! {
    /// Citation identity excludes the bounded presentation excerpt.
    pub struct CitationIdentity {
        fact_id: String => crate::native_union::Rule::NonEmpty,
        subject: crate::evidence::relational::SubjectRef => crate::native_union::Rule::Text,
        source: crate::evidence::relational::FactSource => crate::native_union::Rule::Text,
    }
}

crate::native_struct! {
    /// One supporting fact with its exact subject, typed coordinates and producer binding.
    pub struct Evidence {
        evidence_id: String => crate::native_union::Rule::NonEmpty,
        subject: crate::evidence::relational::SubjectRef => crate::native_union::Rule::Text,
        display_subject: String => crate::native_union::Rule::Text,
        source: crate::evidence::relational::FactSource => crate::native_union::Rule::Text,
        excerpt: String => crate::native_union::Rule::Text,
    }
}

impl Evidence {
    pub fn new(
        fact_id: String,
        subject: crate::evidence::relational::SubjectRef,
        display_subject: String,
        source: crate::evidence::relational::FactSource,
        excerpt: String,
    ) -> datafusion::common::Result<Self> {
        let evidence_id = crate::native_key::Key::Citation.record(&CitationIdentity {
            fact_id,
            subject: subject.clone(),
            source: source.clone(),
        })?;
        Ok(Self {
            evidence_id,
            subject,
            display_subject,
            source,
            excerpt,
        })
    }
}

crate::native_struct! {
/// A pointer to a bounded, readable artifact.
pub struct ArtifactHandle {
    /// Exact content identity and this acquisition's provenance. Pass receipt.artifact_id
    /// to read_artifact; repeated bytes may have different acquisition receipts.
    receipt: crate::evidence::Artifact => crate::native_union::Rule::Text,
    /// Always a `library-evidence://` URI -- enforced by [`ArtifactUri`], not only by the schema.
    uri: ArtifactUri => crate::native_union::Rule::Text,
    /// What the artifact contains, so a caller can decide whether to read it.
    description: String => crate::native_union::Rule::Text,
}
}

#[cfg(test)]
mod citation_tests {
    use super::*;
    use crate::{
        evidence::relational::{FactSource, Locator, SubjectRef},
        native_union::NativeStruct,
    };

    #[test]
    fn typed_citation_identity_preserves_provenance_and_ignores_excerpt() {
        let source = FactSource {
            producer_binding_id: format!("producer_{}", "1".repeat(64)),
            extractor: "rustdoc".into(),
            extractor_version: "61".into(),
            artifact_id: format!("art_{}", "2".repeat(64)),
            source_uri: None,
            source_version_match: SourceVersionMatch::Exact,
            locator: Locator::RustdocItem {
                item: 8,
                reported_file: None,
                reported_line: None,
            },
            evidence_class: EvidenceClass::CompilerDerived,
        };
        let subject = SubjectRef::Definition {
            definition_id: format!("def_{}", "3".repeat(64)),
        };
        let make = |source, excerpt: &str| {
            Evidence::new(
                "observation".into(),
                subject.clone(),
                "λ::Type".into(),
                source,
                excerpt.into(),
            )
            .unwrap()
        };
        let first = make(source.clone(), "short");
        assert_eq!(
            first.evidence_id,
            make(source.clone(), "longer quoted \\\"λ\\\"").evidence_id
        );
        let mut other = source.clone();
        other.locator = Locator::RustdocItem {
            item: 9,
            reported_file: None,
            reported_line: None,
        };
        assert_ne!(first.evidence_id, make(other, "short").evidence_id);
        let batch = Evidence::batch(std::slice::from_ref(&first)).unwrap();
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
        assert_eq!(Evidence::decode(rows.row(0)).unwrap(), first);
        let wire = serde_json::to_value(&first).unwrap();
        assert_eq!(wire["subject"]["kind"], "definition");
        assert_eq!(wire["source"]["locator"]["kind"], "rustdoc_item");
        assert_eq!(wire["source"]["source_version_match"], "exact");
        assert!(wire["source"]["source_uri"].is_null());
        assert_eq!(serde_json::from_value::<Evidence>(wire).unwrap(), first);
    }
}
