//! Native result section and dependency declarations, independent of acquisition receipts.
use crate::native_union::{Domain, Rule, Unit};
pub const JOB_URI: &str = "service:job-delivery/4";
pub const MEDIA_TYPE: &str = "application/vnd.library-enrichment.native-result";
pub const MAX_BYTES: u64 = 32 * 1024 * 1024;

crate::native_struct! {
/// Complete result authority before any inline or retained transport projection.
pub struct ResultRecord {
    header: ResultHeader => Rule::Text,
    data: crate::wire::data::ToolData => Rule::Text,
    evidence: Vec<crate::wire::Evidence> => Rule::Sequence,
    artifacts: Vec<crate::wire::ArtifactHandle> => Rule::Sequence,
    delivery: crate::wire::DeliveryDescriptor => Rule::Text,
}
}
impl ResultRecord {
    pub fn from_envelope(value: &crate::wire::Envelope) -> Result<Self, String> {
        Ok(Self {
            header: ResultHeader::from_envelope(value)?,
            data: value.data.clone(),
            evidence: value.evidence.clone(),
            artifacts: value.artifacts.clone(),
            delivery: value.delivery.clone(),
        })
    }
    /// Mechanical final transport projection; it makes no policy or outcome decision.
    pub fn into_envelope(self, request_id: crate::wire::RequestId) -> crate::wire::Envelope {
        crate::wire::Envelope::new(
            crate::wire::EnvelopeBody {
                request_id,
                summary: self.header.summary,
                context_id: self.header.context_id,
                snapshot_id: self.header.snapshot_id,
                data: self.data,
                coverage: self.header.coverage,
                freshness: self.header.freshness,
                evidence: self.evidence,
                artifacts: self.artifacts,
                delivery: self.delivery,
            },
            self.header.outcome,
        )
    }
}
crate::native_struct! {
/// Typed retained outcome. Recovery reads this admitted record without interpreting JSON fields.
pub struct ResultHeader {
    summary: String => Rule::Text,
    context_id: Option<crate::identity::ContextId> => Rule::Text,
    snapshot_id: Option<crate::identity::SnapshotId> => Rule::Text,
    coverage: crate::wire::Coverage => Rule::Text,
    freshness: crate::wire::Freshness => Rule::Text,
    outcome: crate::wire::Outcome => Rule::Text,
}
}
impl ResultHeader {
    pub fn from_envelope(result: &crate::wire::Envelope) -> Result<Self, String> {
        Ok(Self {
            summary: result.summary.clone(),
            context_id: result.context_id.clone(),
            snapshot_id: result.snapshot_id.clone(),
            coverage: result.coverage.clone(),
            freshness: result.freshness.clone(),
            outcome: result.outcome().ok_or("invalid native result outcome")?,
        })
    }
}
crate::native_struct! { pub struct ResultReference {
    artifact_id: String => Rule::Reference(Domain::Artifact),
    media_type: String => Rule::NonEmpty,
} }
crate::native_struct! { pub struct ResultSection {
    name: String => Rule::NonEmpty,
    start: u64 => Rule::Coordinate(Unit::ByteOffset),
    end: u64 => Rule::RangeEnd { unit: Unit::ByteOffset, start: "start".into() },
} }
crate::native_struct! {
    /// Exact purpose-specific Delta relation selected by a retained result.
    pub struct ResultVersion {
        tool: String => Rule::Vocabulary(crate::wire::data::ToolData::VALUES.iter().map(|value|(*value).to_owned()).collect()),
        table_id: String => Rule::NonEmpty,
        version: u64 => Rule::Text,
        contract_id: String => Rule::Sha256,
    }
}
crate::native_struct! { pub struct RetainedResult {
    result_artifact_id: String => Rule::Reference(Domain::Artifact),
    body_base: u64 => Rule::UnsignedRange { min: 0, max: 32 * 1024 * 1024 },
    version: ResultVersion => Rule::Text,
    sections: Vec<ResultSection> => Rule::SequenceBounds { min: 0, max: 128 },
    references: Vec<ResultReference> => Rule::SequenceBounds { min: 0, max: 1024 },
} }

crate::native_struct! { pub struct ArtifactReceipt {
    receipt_id: String => Rule::NonEmpty,
    artifact: crate::evidence::Artifact => Rule::Text,
} }
