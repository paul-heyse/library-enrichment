//! HTTP observations share generated field contracts with acquisition and cache plans.
use crate::evidence::{Artifact, ArtifactKind};
use crate::native_union::{NativeStruct, Rule};
use arrow::datatypes::{Schema, SchemaRef};
use std::sync::Arc;

crate::native_struct! {
    pub struct Fetched {
        status: u16 => Rule::UnsignedRange { min: 100, max: 599 },
        content_type: Option<String> => Rule::Text,
        etag: Option<String> => Rule::Text,
        last_modified: Option<String> => Rule::Text,
        final_url: String => Rule::NonEmpty,
        retrieved_at: crate::native_time::AcquisitionTime => Rule::Text,
    } ephemeral { bytes: Vec<u8> = Vec::new() }
}
impl Fetched {
    /// Capture response facts without copying the separately owned, bounded byte buffer.
    pub fn metadata(&self) -> Self {
        Self {
            status: self.status,
            bytes: Vec::new(),
            content_type: self.content_type.clone(),
            etag: self.etag.clone(),
            last_modified: self.last_modified.clone(),
            final_url: self.final_url.clone(),
            retrieved_at: self.retrieved_at,
        }
    }
}
crate::native_struct! {
    pub struct CacheRecord {
        response: Fetched => Rule::Text,
        request_url: String => Rule::NonEmpty,
        accept: Option<String> => Rule::Text,
        body_digest: String => Rule::Sha256,
        body_bytes: u64 => Rule::Text,
    }
}
pub fn response_schema() -> SchemaRef {
    Arc::new(Schema::new(Fetched::fields()))
}
pub fn cache_schema() -> SchemaRef {
    Arc::new(Schema::new(CacheRecord::fields()))
}

crate::native_struct! {
    pub struct Cached {
        record: CacheRecord => Rule::Text,
        reuse: bool => Rule::Text,
        validator_name: Option<String> => Rule::Text,
        validator_value: Option<String> => Rule::Text,
    }
}

impl Cached {
    pub fn response(&self) -> &Fetched {
        &self.record.response
    }
    pub fn artifact(&self) -> Artifact {
        // Decode the admitted cache's physical descriptor. No state selection occurs here.
        Artifact {
            artifact_id: crate::evidence::artifact_id_for(&self.record.body_digest),
            sha256: self.record.body_digest.clone(),
            size_bytes: self.record.body_bytes,
            media_type: self
                .record
                .response
                .content_type
                .clone()
                .unwrap_or_else(|| "application/octet-stream".into()),
            kind: ArtifactKind::Other,
            source_uri: self.record.request_url.clone(),
            final_url: Some(self.record.response.final_url.clone()),
            retrieved_at: self.record.response.retrieved_at,
            etag: self.record.response.etag.clone(),
            last_modified: self.record.response.last_modified.clone(),
            compression: None,
        }
    }
}
