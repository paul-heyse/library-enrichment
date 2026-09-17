//! Native catalog, operation and delivery projections using core Arrow evidence contracts.

pub(crate) mod browse;
pub mod catalog;
pub(crate) mod comparison;
pub(crate) mod comparison_value;
pub(crate) mod contracts;
pub mod publication;
pub(crate) mod render;
pub(crate) mod score;
pub(crate) mod search;
pub(crate) mod staging;

// The core owns the fixed evidence Arrow contracts. These imports are the store's
// boundary facade, not independent schema definitions or historical decoders.
pub use enrichment_core::evidence::arrow_model::{
    TextColumn, acquisitions, bindings, cells, coverage, coverage_from_batch, decode, definitions,
    encode, execution, fragments, fragments_from_batch, input_artifacts,
    input_artifacts_from_batch, metadata, observations, producer_runs, producer_runs_from_batch,
    provenance, relations, relationships, relationships_from_batch,
};

/// One storage contract; historical formats have no decoder in this module.
pub const VERSION: &str = enrichment_core::evidence::arrow_model::VERSION;

/// One named projection contract shared by the Parquet provider and inspection decoder.
pub(crate) fn inspection_schema(
    schema: &arrow_schema::Schema,
) -> datafusion::error::Result<arrow_schema::SchemaRef> {
    use arrow_schema::Schema;
    use std::sync::Arc;
    schema.index_of("docs")?;
    let fields: Vec<_> = schema
        .fields()
        .iter()
        .filter(|field| field.name() != "docs")
        .cloned()
        .collect();
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
