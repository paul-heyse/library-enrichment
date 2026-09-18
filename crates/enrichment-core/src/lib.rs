//! Identities, wire types, the evidence model, producer plans, and execution policy.
//!
//! This crate is authoritative. JSON Schemas are generated from its wire types, and the Python
//! boundary DTOs are generated from those schemas (blueprint §6.3). Nothing downstream
//! hand-maintains a parallel definition.
//!
//! Policy is enforced here, not in MCP tool annotations and not in the companion skill
//! (blueprint §10). An annotation describes behaviour to a client; it constrains nothing.

/// The wire schema version carried by every tool response.
///
/// The native runtime epoch exposes complete artifact receipts and one current contract.
extern crate self as enrichment_core;

pub const SCHEMA_VERSION: &str = wire::SchemaVersion::Current.as_str();

/// Canonical snapshot storage, independent of the frozen response envelope.
pub const SNAPSHOT_SCHEMA_VERSION: &str = evidence::snapshot::FORMAT;

pub mod archive;
pub mod canonical;
pub mod compare;
pub mod config;
pub mod delta_reference;
pub mod evidence;
pub mod execution;
pub mod http;
pub mod identity;
pub mod json_output;
pub mod mcp_delivery;
pub mod native_bytes;
pub mod policy;
pub mod producer;
pub mod registry;
pub mod request;
pub mod search;
pub mod status_components;
pub mod wire;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_matches_the_wire_type() {
        assert_eq!(
            serde_json::to_value(wire::SchemaVersion::default()).unwrap(),
            SCHEMA_VERSION
        );
    }
}
pub mod capsule_protocol;

pub mod native_analysis;
pub mod native_artifact;
pub mod native_collections;
pub mod native_comparison;
pub mod native_contract;
pub(crate) mod native_cursor;
pub mod native_digest;
pub mod native_id;
/// Shared Arrow kernels consumed by native DataFusion identity plans.
pub mod native_identity;
pub mod native_json;
pub mod native_key;
pub mod native_lsp;
pub mod native_payload;
pub mod native_predicate;
pub mod native_record;
pub mod native_runtime;
pub mod native_schema;
pub mod native_selection;
pub mod native_semantics;
pub mod native_text;
pub mod native_time;
pub mod native_transport;
pub mod native_types;
pub mod native_union;
pub mod native_wire;

pub mod native_cargo;
pub mod native_python_metadata;
pub mod native_url;
/// Library-format kernels for optimizer-visible native version operations.
pub mod native_version;

pub mod operation;

pub mod telemetry;
