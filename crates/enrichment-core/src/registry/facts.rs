//! Mechanical sparse-index decoding. Release policy is evaluated by native plans.
use super::{IndexEntry, RegistryError};
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use std::sync::Arc;

fn text(name: &str, nullable: bool) -> Field {
    Field::new(name, DataType::Utf8, nullable)
}
fn strings(name: &str) -> Field {
    Field::new(name, DataType::List(Arc::new(text("item", false))), false)
}

/// Complete upstream release facts, including physical input-line provenance.
pub fn schema() -> SchemaRef {
    let dependency = DataType::Struct(
        vec![
            text("name", false),
            text("req", false),
            strings("features"),
            Field::new("optional", DataType::Boolean, false),
            Field::new("default_features", DataType::Boolean, false),
            text("target", true),
            text("kind", true),
            text("package", true),
        ]
        .into(),
    );
    let features = DataType::Map(
        Arc::new(Field::new(
            "entries",
            DataType::Struct(vec![text("keys", false), strings("values")].into()),
            false,
        )),
        false,
    );
    let release = DataType::Struct(
        vec![
            text("name", false),
            text("vers", false),
            Field::new(
                "deps",
                DataType::List(Arc::new(Field::new("item", dependency, false))),
                false,
            ),
            text("cksum", false),
            Field::new("features", features.clone(), false),
            Field::new("features2", features, false),
            Field::new("yanked", DataType::Boolean, false),
            text("links", true),
            Field::new("v", DataType::UInt32, false),
            text("rust_version", true),
            text("pubtime", true),
        ]
        .into(),
    );
    Arc::new(Schema::new(vec![
        Field::new("source_line", DataType::UInt64, false),
        Field::new("release", release, false),
    ]))
}

#[derive(serde::Serialize)]
struct Fact {
    source_line: u64,
    release: IndexEntry,
}

/// Decode only input syntax/defaults; do not select, sort or derive semantic rows here.
/// # Errors
/// Malformed records retain their physical line number; Arrow conversion failures fail closed.
pub fn decode(text: &str, batch_rows: usize) -> Result<Vec<RecordBatch>, RegistryError> {
    if batch_rows == 0 || batch_rows > 1_000_000 {
        return Err(RegistryError::ArrowFacts("invalid fact batch bound".into()));
    }
    let contract = schema();
    let mut decoder = arrow::json::ReaderBuilder::new(contract.clone())
        .with_batch_size(batch_rows)
        .build_decoder()
        .map_err(|e| RegistryError::ArrowFacts(e.to_string()))?;
    let mut batches = Vec::new();
    let mut rows = Vec::with_capacity(batch_rows.min(1024));
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        rows.push(Fact {
            source_line: (index + 1) as u64,
            release: serde_json::from_str(line).map_err(|e| RegistryError::MalformedIndexLine {
                line: index + 1,
                message: e.to_string(),
            })?,
        });
        if rows.len() == batch_rows {
            decoder
                .serialize(&rows)
                .map_err(|e| RegistryError::ArrowFacts(e.to_string()))?;
            if let Some(batch) = decoder
                .flush()
                .map_err(|e| RegistryError::ArrowFacts(e.to_string()))?
            {
                batches.push(batch);
            }
            rows.clear();
        }
    }
    if !rows.is_empty() {
        decoder
            .serialize(&rows)
            .map_err(|e| RegistryError::ArrowFacts(e.to_string()))?;
        if let Some(batch) = decoder
            .flush()
            .map_err(|e| RegistryError::ArrowFacts(e.to_string()))?
        {
            batches.push(batch);
        }
    }
    if batches.is_empty() {
        batches.push(RecordBatch::new_empty(contract));
    }
    Ok(batches)
}
