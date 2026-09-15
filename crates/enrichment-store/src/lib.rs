//! Content-addressed artifacts, immutable snapshot manifests, and Arrow/DataFusion retrieval.
//!
//! Only the daemon publishes (blueprint §8.2). This crate stages, validates, and then
//! atomically swaps a context's current pointer, so a reader never observes a partially
//! written table -- including after a crash mid-publication (gate C06).
//!
//! Every function here takes an explicit root. Resolving the roots from the environment happens
//! once, in the daemon binary, through [`paths::StatePaths::from_env`]; library code and tests
//! never read `LIBENR_*` themselves, so a parallel test run cannot race on a shared directory.

pub mod admission;
pub mod atomic;
pub mod blob;
pub mod browse;
pub mod bundle;
pub mod catalog_generation;
pub mod comparison;
pub mod coverage;
pub mod dataset;
pub mod ingest;
pub mod leases;
pub mod native_rustdoc;
mod operation_index;
pub mod paths;
pub mod preparation;
pub mod projection;
mod provider;
pub mod publication_probe;
pub mod query;
pub mod query_diagnostics;
pub mod query_failure;
pub mod repository;
pub mod result;
pub mod runtime;
pub mod scoring;
pub mod search_plan;
mod semantic;
pub mod semantic_scope;
pub mod state;
pub mod views;

pub use blob::{BlobStore, StoredBlob};
pub use paths::{StatePathError, StatePaths};
pub use query::{Overview, QueryError, SnapshotReader};

mod execution_documents;

pub mod record_writer;

#[doc(hidden)]
pub mod parquet_admission;
