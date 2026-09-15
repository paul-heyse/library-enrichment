//! Explicit Arrow contracts for normalized evidence (ADR-0022).
//!
//! Nested values remain typed through Parquet and DataFusion. Encoders are also the schema
//! authority: empty input produces exactly the same fields and metadata as populated input.

mod acquisitions;
pub(crate) mod browse;
pub mod catalog;
mod cells;
pub(crate) mod comparison;
pub(crate) mod comparison_publication;
pub(crate) mod comparison_value;
pub mod decode;
mod encode;
pub mod execution;
pub mod metadata;
mod provenance;
mod relations;
pub(crate) mod render;
pub(crate) mod score;
pub(crate) mod search;
pub(crate) mod staging;

pub use cells::TextColumn;
pub use encode::{bindings, definitions, observations};
pub use provenance::{
    coverage, coverage_from_batch, input_artifacts, input_artifacts_from_batch, producer_runs,
    producer_runs_from_batch,
};
pub use relations::{fragments, fragments_from_batch, relationships, relationships_from_batch};

/// One storage contract; historical formats have no decoder in this module.
pub const VERSION: &str = "6.0";

/// One named projection contract shared by the Parquet provider and inspection decoder.
pub(crate) fn inspection_schema(
    schema: &arrow_schema::Schema,
) -> datafusion::error::Result<arrow_schema::SchemaRef> {
    use arrow_schema::{DataType, Schema};
    use std::sync::Arc;
    let mut fields = schema.fields().to_vec();
    let index = schema.index_of("payload")?;
    let DataType::Struct(payload) = fields[index].data_type() else {
        return Err(datafusion::error::DataFusionError::Internal(
            "API payload is not a struct".into(),
        ));
    };
    let narrow = payload
        .iter()
        .filter(|f| f.name() != "docs")
        .cloned()
        .collect();
    fields[index] = Arc::new(
        fields[index]
            .as_ref()
            .clone()
            .with_data_type(DataType::Struct(narrow)),
    );
    let mut metadata = schema.metadata().clone();
    metadata.insert(
        "enrichment.relation".into(),
        "inspection_observations".into(),
    );
    metadata.insert(
        "enrichment.projection".into(),
        "api_without_documentation".into(),
    );
    Ok(Arc::new(Schema::new_with_metadata(fields, metadata)))
}

/// Encode a typed domain subject for a bound native query parameter.
pub(crate) fn subject_scalar(
    subject: &enrichment_core::evidence::relational::SubjectRef,
) -> datafusion::error::Result<datafusion::common::ScalarValue> {
    let array = encode::subject(&[subject])?;
    datafusion::common::ScalarValue::try_from_array(array.as_ref(), 0)
}
