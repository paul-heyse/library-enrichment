//! Explicit Arrow contracts for normalized evidence (ADR-0022).
//!
//! Nested values remain typed through Parquet and DataFusion. Encoders are also the schema
//! authority: empty input produces exactly the same fields and metadata as populated input.

mod acquisitions;
pub(crate) mod browse;
pub mod catalog;
mod cells;
pub(crate) mod comparison;
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
pub const VERSION: &str = "5.0";
