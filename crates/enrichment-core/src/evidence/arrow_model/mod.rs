//! Authoritative Arrow evidence contracts and mechanical boundary codecs.
//! Native plans consume these exact fields; no storage crate maintains a second schema.

pub mod acquisitions;
pub mod cells;
pub mod checks;
pub mod decode;
pub mod encode;
pub mod execution;
pub mod expressions;
pub mod metadata;
pub mod provenance;
pub mod relations;

pub use cells::TextColumn;
pub use encode::{bindings, definitions, observations};
pub use provenance::{
    coverage, coverage_from_batch, input_artifacts, input_artifacts_from_batch, producer_runs,
    producer_runs_from_batch,
};
pub use relations::{fragments, fragments_from_batch, relationships, relationships_from_batch};

pub const VERSION: &str = super::snapshot::FORMAT;
