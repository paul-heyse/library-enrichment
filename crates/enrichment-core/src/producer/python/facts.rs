//! Mechanical PyPI file facts. No artifact ranking or environment policy is evaluated here.
use super::DistributionFile;
use arrow::{
    datatypes::{Schema, SchemaRef},
    record_batch::RecordBatch,
};
use std::{collections::BTreeMap, sync::Arc};

use crate::native_union::{NativeStruct, Rule};
crate::native_struct! {
    pub struct Fact {
        version: String => Rule::Text,
        file: DistributionFile => Rule::Text,
    }
}
pub fn schema() -> SchemaRef {
    Arc::new(Schema::new(Fact::fields()))
}

pub fn decode(
    releases: &BTreeMap<String, Vec<DistributionFile>>,
    batch_rows: usize,
) -> arrow::error::Result<Vec<RecordBatch>> {
    if batch_rows == 0 || batch_rows > 1_000_000 {
        return Err(arrow::error::ArrowError::InvalidArgumentError(
            "invalid fact batch bound".into(),
        ));
    }
    let contract = schema();
    let mut batches = Vec::new();
    let mut rows = Vec::with_capacity(batch_rows.min(1024));
    for (version, files) in releases {
        for file in files {
            rows.push(Fact {
                version: version.clone(),
                file: file.clone(),
            });
            if rows.len() == batch_rows {
                batches.push(Fact::batch(&rows)?);
                rows.clear();
            }
        }
    }
    if !rows.is_empty() {
        batches.push(Fact::batch(&rows)?);
    }
    if batches.is_empty() {
        batches.push(RecordBatch::new_empty(contract));
    }
    Ok(batches)
}
