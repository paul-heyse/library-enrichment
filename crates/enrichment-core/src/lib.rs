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
/// Pinned to the frozen contract in `contracts/research-envelope.schema.json`, which is the
/// Phase-0 acceptance target. `just schema-conformance` asserts that generated schemas accept
/// and reject exactly the documents the frozen schema does.
pub const SCHEMA_VERSION: &str = "1.0";

#[cfg(test)]
mod tests {
    use super::*;

    /// The frozen contract pins `schema_version` to the literal "1.0". If this constant and the
    /// frozen schema disagree, every emitted envelope fails validation.
    #[test]
    fn schema_version_matches_the_frozen_contract() {
        let frozen = include_str!("../../../contracts/research-envelope.schema.json");
        let v: serde_json::Value = serde_json::from_str(frozen).expect("frozen schema parses");
        let pinned = v["properties"]["schema_version"]["const"]
            .as_str()
            .expect("frozen schema pins schema_version to a const");
        assert_eq!(
            pinned, SCHEMA_VERSION,
            "SCHEMA_VERSION disagrees with the frozen contract"
        );
    }
}
