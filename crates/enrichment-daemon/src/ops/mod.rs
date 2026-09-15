//! One module per RPC method that does research work. Each takes the shared [`Service`] and a
//! typed request, and returns a complete wire envelope built in the core's terms.
//!
//! [`Service`]: crate::service::Service

pub mod artifact;
mod artifact_window;
pub mod common;
pub mod inspect;
pub mod manifest;
pub mod overview;
mod publication;
pub mod resolve;
mod resolve_job;
pub mod search;
mod source_documents;
mod source_tree;

pub mod inspect_execution;
mod runtime_object;
/// Language-server observations behind the `semantics` aspect.
pub mod semantics;

pub mod python;

pub mod compare;
mod compare_job;

/// Immutable repository source resolution.
pub mod revision;

pub mod verify;
