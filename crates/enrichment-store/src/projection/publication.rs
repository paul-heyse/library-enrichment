//! Bounded publication metadata encoded as native fields in the atomic control transaction.
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    error::ArrowError,
    record_batch::RecordBatch,
};
use enrichment_core::evidence::catalog::SnapshotEntry;
use std::sync::Arc;

fn text(name: &str) -> Field {
    Field::new(name, DataType::Utf8, false)
}
fn number(name: &str) -> Field {
    Field::new(name, DataType::UInt64, false)
}
fn structure(name: &str, fields: Vec<Field>, nullable: bool) -> Field {
    Field::new(name, DataType::Struct(fields.into()), nullable)
}
fn list(name: &str, value: DataType) -> Field {
    Field::new(
        name,
        DataType::List(Arc::new(Field::new("item", value, false))),
        false,
    )
}

pub(crate) fn schema() -> SchemaRef {
    let observed = structure(
        "observed_configuration",
        vec![
            list("features", DataType::Utf8),
            Field::new("all_features", DataType::Boolean, false),
            Field::new("no_default_features", DataType::Boolean, false),
            text("target"),
            Field::new("format_version", DataType::UInt32, false),
            text("source"),
        ],
        true,
    );
    let metadata = structure(
        "metadata",
        vec![
            text("context_id"),
            text("release_id"),
            text("environment_id"),
            text("ecosystem"),
            text("symbol_package"),
            text("crate_name"),
            Field::new("crate_version", DataType::Utf8, true),
            text("normalizer_version"),
            observed,
            number("producer_items"),
        ],
        false,
    );
    let components = Field::new(
        "components",
        DataType::Map(
            Arc::new(structure(
                "entries",
                vec![text("keys"), text("values")],
                false,
            )),
            false,
        ),
        false,
    );
    let binding = binding_type();
    let counts = structure(
        "counts",
        [
            "symbols",
            "definitions",
            "reexports",
            "unresolved_reexports",
            "relationships",
            "fragments",
            "producer_items",
        ]
        .into_iter()
        .map(number)
        .collect(),
        false,
    );
    let publication = structure(
        "publication",
        vec![
            text("snapshot_id"),
            text("schema_version"),
            metadata,
            components,
            list("tables", binding),
            counts,
            list("indexed", DataType::Utf8),
            list("missing", DataType::Utf8),
            text("published_at"),
        ],
        false,
    );
    Arc::new(Schema::new(vec![
        text("snapshot_id"),
        text("context_id"),
        publication,
    ]))
}

pub(crate) fn descriptor_schema() -> SchemaRef {
    let schema = schema();
    let DataType::Struct(publication) = schema
        .field_with_name("publication")
        .expect("publication contract")
        .data_type()
    else {
        unreachable!("publication record")
    };
    let DataType::Struct(metadata) = publication
        .find("metadata")
        .expect("metadata contract")
        .1
        .data_type()
    else {
        unreachable!("metadata record")
    };
    Arc::new(Schema::new(metadata.clone()))
}

pub fn encode(rows: &[SnapshotEntry]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(super::cells::invalid)?;
    }
    crate::control_jobs::encode(schema(), rows).map_err(|e| super::cells::invalid(e.to_string()))
}

pub fn decode(batch: &RecordBatch) -> Result<Vec<SnapshotEntry>, ArrowError> {
    // One bounded wire/control record, never an evidence corpus or persisted JSON payload.
    if batch.num_rows() > 1024 {
        return Err(super::cells::invalid("publication batch exceeds bound"));
    }
    let mut writer = arrow::json::ArrayWriter::new(Vec::new());
    writer.write(batch)?;
    writer.finish()?;
    let rows: Vec<SnapshotEntry> = serde_json::from_slice(&writer.into_inner())
        .map_err(|e| super::cells::invalid(e.to_string()))?;
    for row in &rows {
        row.validate().map_err(super::cells::invalid)?;
    }
    Ok(rows)
}

pub(crate) fn binding_type() -> DataType {
    DataType::Struct(
        vec![
            text("relation"),
            text("table_uri"),
            text("table_id"),
            number("version"),
            text("cohort_id"),
            text("contract_id"),
            number("rows"),
        ]
        .into(),
    )
}
