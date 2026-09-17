//! JSON Schema emission from the authoritative Rust wire types (blueprint §6.3).

use schemars::generate::SchemaSettings;

use super::envelope::Envelope;

/// The filename `emit-schemas` writes and `scripts/schema-conformance.py` reads.
pub const ENVELOPE_SCHEMA_FILE: &str = "research-envelope.schema.json";

/// Generate the response-envelope schema.
///
/// # Why the serialize contract
///
/// Under `schemars`' default deserialize contract, `Option<T>` fields are omitted from
/// `required` -- which would drop `context_id`, `snapshot_id`, `job` and `error` from the
/// fourteen root fields the frozen contract requires, and fail
/// `scripts/schema-conformance.py`'s `root required fields` check. `for_serialize` keeps them
/// required *and* nullable, which is also the semantically correct reading: this schema
/// describes what the service emits.
///
/// Do not reach for `#[schemars(required)]` to fix that instead -- it strips the `null` from
/// the type union and would reject every fixture's `"job": null`.
///
/// # Determinism
///
/// The output is canonicalized to lexicographic key order by [`canonicalize`] before it is
/// written, and that is not belt-and-braces -- it is load-bearing.
///
/// `serde_json::Map` is a `BTreeMap` normally but an `IndexMap` when the `preserve_order`
/// feature is on, and **DataFusion enables it**: `enrichment-store` -> `datafusion` ->
/// `serde_json/preserve_order`. Cargo unifies features across whatever is being built, so
/// `cargo run -p enrichment-core --bin emit-schemas` (what `scripts/schemas-generate.sh` runs)
/// gets the `BTreeMap` and emits sorted keys, while `cargo nextest run --workspace` gets the
/// `IndexMap` and emits insertion order. Without canonicalizing, the bytes on disk would depend
/// on which cargo invocation produced them, and `scripts/schema-conformance.sh`'s
/// `git diff --exit-code` over `schemas/generated/` would churn.
#[must_use]
pub fn envelope_schema() -> serde_json::Value {
    canonicalize(
        SchemaSettings::draft2020_12()
            .for_serialize()
            .into_generator()
            .into_root_schema_for::<Envelope>()
            .to_value(),
    )
}

/// Rebuild every object with its keys in lexicographic order.
///
/// Sorting explicitly makes emission independent of whether `serde_json::Map` is a `BTreeMap`
/// or an `IndexMap` -- see the determinism note on [`envelope_schema`]. Under a `BTreeMap` this
/// is a no-op; under an `IndexMap` it is what makes the output reproducible. Array order is
/// preserved: `enum` order in particular is what `scripts/schema-conformance.py` compares
/// against schemas/frozen/enums.json. The same canonical form is what content identities are
/// hashed over, so it lives in [`crate::canonical`].
fn canonicalize(value: serde_json::Value) -> serde_json::Value {
    crate::canonical::canonicalize(crate::native_wire::schema(value))
}

/// The schema as the exact bytes `emit-schemas` writes.
///
/// Pretty-printed with a trailing newline: `.gitattributes` pins `eol=lf`, and a file without a
/// final newline shows up as a permanent diff.
#[must_use]
pub fn envelope_schema_json() -> String {
    let mut json = serde_json::to_string_pretty(&envelope_schema())
        .expect("a generated schema is always serializable");
    json.push('\n');
    json
}
