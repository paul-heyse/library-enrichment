//! Mechanical PyPI file facts. No artifact ranking or environment policy is evaluated here.
use super::DistributionFile;
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use std::{collections::BTreeMap, sync::Arc};

pub fn schema() -> SchemaRef {
    let text = |name: &str, nullable| Field::new(name, DataType::Utf8, nullable);
    let digests = DataType::Map(
        Arc::new(Field::new(
            "entries",
            DataType::Struct(vec![text("keys", false), text("values", false)].into()),
            false,
        )),
        false,
    );
    Arc::new(Schema::new(vec![
        text("version", false),
        Field::new(
            "file",
            DataType::Struct(
                vec![
                    text("filename", false),
                    text("packagetype", false),
                    text("url", false),
                    Field::new("digests", digests, false),
                    text("requires_python", true),
                    Field::new("yanked", DataType::Boolean, false),
                ]
                .into(),
            ),
            false,
        ),
    ]))
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
    #[derive(serde::Serialize)]
    struct Fact<'a> {
        version: &'a str,
        file: &'a DistributionFile,
    }
    let contract = schema();
    let mut decoder = arrow::json::ReaderBuilder::new(contract.clone())
        .with_batch_size(batch_rows)
        .build_decoder()?;
    let mut batches = Vec::new();
    let mut rows = Vec::with_capacity(batch_rows.min(1024));
    for (version, files) in releases {
        for file in files {
            rows.push(Fact { version, file });
            if rows.len() == batch_rows {
                decoder.serialize(&rows)?;
                if let Some(batch) = decoder.flush()? {
                    batches.push(batch);
                }
                rows.clear();
            }
        }
    }
    if !rows.is_empty() {
        decoder.serialize(&rows)?;
        if let Some(batch) = decoder.flush()? {
            batches.push(batch);
        }
    }
    if batches.is_empty() {
        batches.push(RecordBatch::new_empty(contract));
    }
    Ok(batches)
}
