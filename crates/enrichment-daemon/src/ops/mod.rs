//! One module per RPC method that does research work. Each takes the shared [`Service`] and a
//! typed request, and returns a complete wire envelope built in the core's terms.
//!
//! [`Service`]: crate::service::Service

pub mod artifact;
pub mod common;
pub mod inspect;
pub mod manifest;
pub mod overview;
pub mod resolve;
pub mod search;
