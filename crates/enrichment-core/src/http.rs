//! HTTP response facts shared by the physical client and native response plans.
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fetched {
    pub status: u16,
    #[serde(skip)]
    pub bytes: Vec<u8>,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub final_url: String,
    pub retrieved_at: String,
}

impl Fetched {
    /// Copy bounded response headers without copying the independently owned body buffer.
    pub fn metadata(&self) -> Self {
        Self {
            status: self.status,
            bytes: Vec::new(),
            content_type: self.content_type.clone(),
            etag: self.etag.clone(),
            last_modified: self.last_modified.clone(),
            final_url: self.final_url.clone(),
            retrieved_at: self.retrieved_at.clone(),
        }
    }
}

pub fn response_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("status", DataType::UInt16, false),
        Field::new("content_type", DataType::Utf8, true),
        Field::new("etag", DataType::Utf8, true),
        Field::new("last_modified", DataType::Utf8, true),
        Field::new("final_url", DataType::Utf8, false),
        Field::new("retrieved_at", DataType::Utf8, false),
    ]))
}

pub fn cache_schema() -> SchemaRef {
    let mut fields = response_schema()
        .fields()
        .iter()
        .map(|f| f.as_ref().clone())
        .collect::<Vec<_>>();
    fields.extend([
        Field::new("request_url", DataType::Utf8, false),
        Field::new("accept", DataType::Utf8, true),
        Field::new("body_digest", DataType::Utf8, false),
        Field::new("body_bytes", DataType::UInt64, false),
    ]);
    Arc::new(Schema::new(fields))
}
