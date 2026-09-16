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
pub const SCHEMA_VERSION: &str = "3.0";

/// Canonical snapshot storage, independent of the frozen response envelope.
pub const SNAPSHOT_SCHEMA_VERSION: &str = evidence::snapshot::FORMAT;

pub mod archive;
pub mod canonical;
pub mod clock;
pub mod compare;
pub mod config;
pub mod evidence;
pub mod execution;
pub mod http;
pub mod identity;
pub mod policy;
pub mod producer;
pub mod registry;
pub mod request;
pub mod search;
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

/// Shared Arrow kernels consumed by native DataFusion identity plans.
pub mod native_identity;
pub mod native_key;

pub mod native_url;
/// Library-format kernels for optimizer-visible native version operations.
pub mod native_version;

pub mod operation;

pub mod telemetry;
