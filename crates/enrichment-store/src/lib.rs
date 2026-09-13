//! Content-addressed artifacts, immutable snapshot manifests, and Arrow/DataFusion retrieval.
//!
//! Only the daemon publishes (blueprint §8.2). This crate stages, validates, and then
//! atomically swaps a context's current pointer, so a reader never observes a partially
//! written table -- including after a crash mid-publication (gate C06).
//!
//! Every function here takes an explicit root. Resolving the roots from the environment happens
//! once, in the daemon binary, through [`paths::StatePaths::from_env`]; library code and tests
//! never read `LIBENR_*` themselves, so a parallel test run cannot race on a shared directory.

pub mod blob;
pub mod catalog;
pub mod paths;
pub mod query;
pub mod snapshot;
pub mod tables;

pub use blob::{BlobStore, StoredBlob};
pub use catalog::Catalog;
pub use paths::{StatePathError, StatePaths};
pub use query::{Overview, QueryError, SnapshotReader};
pub use snapshot::{PublishError, SnapshotTables, publish};
