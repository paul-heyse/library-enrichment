//! Mechanical document facts. Native plans supply subjects, source qualification and identities.
use super::{FragmentKind, relational::Locator};
use crate::wire::{EvidenceClass, SourceVersionMatch};
use arrow::{array::UInt64Array, error::ArrowError, record_batch::RecordBatch};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct DocumentFact {
    pub kind: FragmentKind,
    pub subject: String,
    pub artifact_id: String,
    pub locator: Locator,
    pub text: String,
    pub evidence_class: EvidenceClass,
    pub producer: String,
    pub producer_version: String,
    pub source_uri: Option<String>,
    pub source_version_match: Option<SourceVersionMatch>,
}
impl DocumentFact {
    pub fn new(
        kind: FragmentKind,
        subject: &str,
        artifact: &str,
        locator: Locator,
        text: String,
        evidence_class: EvidenceClass,
        producer: &str,
        version: &str,
    ) -> Result<Self, String> {
        let value = Self {
            kind,
            subject: subject.into(),
            artifact_id: artifact.into(),
            locator,
            text,
            evidence_class,
            producer: producer.into(),
            producer_version: version.into(),
            source_uri: None,
            source_version_match: None,
        };
        crate::canonical::serialized_size(&value, 1024 * 1024).map_err(|e| e.to_string())?;
        Ok(value)
    }
}
#[derive(Serialize)]
pub struct Input {
    pub ordinal: u64,
    pub fact: DocumentFact,
}
pub fn encode(rows: &[Input]) -> Result<RecordBatch, ArrowError> {
    use super::arrow_model::{
        cells::{batch, column, optional, text},
        encode::locator,
    };
    batch(
        "document_facts",
        vec![
            column(
                "ordinal",
                Arc::new(UInt64Array::from_iter_values(
                    rows.iter().map(|r| r.ordinal),
                )),
                false,
                "input-ordinal",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.fact.kind.as_str())),
                false,
                "vocabulary:fragment-kind/1",
            ),
            column(
                "label",
                text(rows.iter().map(|r| r.fact.subject.as_str())),
                false,
                "document-label",
            ),
            column(
                "artifact_id",
                text(rows.iter().map(|r| r.fact.artifact_id.as_str())),
                false,
                "ref:artifact",
            ),
            column(
                "locator",
                locator(&rows.iter().map(|r| &r.fact.locator).collect::<Vec<_>>())?,
                false,
                "artifact-coordinates",
            ),
            column(
                "text",
                text(rows.iter().map(|r| r.fact.text.as_str())),
                false,
                "document-text",
            ),
            column(
                "producer",
                text(rows.iter().map(|r| r.fact.producer.as_str())),
                false,
                "extractor-name",
            ),
            column(
                "producer_version",
                text(rows.iter().map(|r| r.fact.producer_version.as_str())),
                false,
                "extractor-version",
            ),
            column(
                "source_uri",
                optional(rows.iter().map(|r| r.fact.source_uri.as_deref())),
                true,
                "acquisition-uri",
            ),
            column(
                "source_version_match",
                optional(rows.iter().map(|r| {
                    r.fact.source_version_match.map(|v| match v {
                        SourceVersionMatch::Exact => "exact",
                        SourceVersionMatch::CompatibleClaimed => "compatible_claimed",
                        SourceVersionMatch::Mismatched => "mismatched",
                        SourceVersionMatch::Unknown => "unknown",
                    })
                })),
                true,
                "vocabulary:source-version-match/1",
            ),
            column(
                "evidence_class",
                text(rows.iter().map(|r| match r.fact.evidence_class {
                    EvidenceClass::Declared => "declared",
                    EvidenceClass::StaticallyExtracted => "statically_extracted",
                    EvidenceClass::CompilerDerived => "compiler_derived",
                    EvidenceClass::TypecheckerObserved => "typechecker_observed",
                    EvidenceClass::RuntimeObserved => "runtime_observed",
                    EvidenceClass::AgentInferred => "agent_inferred",
                })),
                false,
                "vocabulary:evidence-class/1",
            ),
        ],
    )
}
