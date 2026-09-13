//! Content-addressed artifacts, immutable snapshot manifests, and Arrow/DataFusion retrieval.
//!
//! Only the daemon publishes (blueprint §8.2). This crate stages, validates, and then
//! atomically swaps a context's current pointer, so a reader never observes a partially
//! written table -- including after a crash mid-publication (gate C06).
