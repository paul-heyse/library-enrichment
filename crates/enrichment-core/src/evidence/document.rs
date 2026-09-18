//! Mechanical document facts. Native plans supply subjects, source qualification and identities.
use super::{FragmentKind, relational::Locator};
use crate::native_union::{NativeStruct, Rule};
use crate::wire::{EvidenceClass, SourceVersionMatch};
use arrow::{array::UInt64Array, error::ArrowError, record_batch::RecordBatch};
use std::sync::Arc;

crate::native_struct! { pub struct DocumentFact {
    kind: FragmentKind => Rule::Text,
    subject: String => Rule::Text,
    artifact_id: String => Rule::Reference(crate::native_union::Domain::Artifact),
    locator: Locator => Rule::Text,
    text: String => Rule::Text,
    evidence_class: EvidenceClass => Rule::Text,
    producer: String => Rule::NonEmpty,
    producer_version: String => Rule::NonEmpty,
    source_uri: Option<String> => Rule::Text,
    source_version_match: Option<SourceVersionMatch> => Rule::Text,
} }
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
crate::native_struct! { pub struct Input {
    ordinal: u64 => Rule::Coordinate(crate::native_union::Unit::Ordinal),
    fact: DocumentFact => Rule::Text,
} }

/// The staging layout flattens the declared fact without a second field or vocabulary
/// inventory. `label` is the native fragment subject projection's physical input name.
pub fn encode(rows: &[Input]) -> Result<RecordBatch, ArrowError> {
    use arrow::array::AsArray;
    let facts = DocumentFact::encode(&rows.iter().map(|r| Some(&r.fact)).collect::<Vec<_>>())?;
    let facts = facts.as_struct();
    let mut columns = vec![(
        crate::native_union::field::<u64>(
            "ordinal",
            Rule::Coordinate(crate::native_union::Unit::Ordinal),
        ),
        Arc::new(UInt64Array::from_iter_values(
            rows.iter().map(|r| r.ordinal),
        )) as arrow::array::ArrayRef,
    )];
    columns.extend(
        DocumentFact::fields()
            .iter()
            .zip(facts.columns())
            .map(|(field, array)| {
                let field = if field.name() == "subject" {
                    field.as_ref().clone().with_name("label")
                } else {
                    field.as_ref().clone()
                };
                (field, array.clone())
            }),
    );
    super::arrow_model::cells::batch("document_facts", columns)
}
