# `arrow_avro::schema::SchemaStore`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.schema.SchemaStore.json).

<a id="op-3ef84c94db2621c1fb7a1dad"></a>
## SchemaStore

`struct` · `arrow_avro::schema::SchemaStore` · arrow-avro 59.3.0

```rust
struct SchemaStore
```

Source: `src/schema.rs:834`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

An in-memory cache of Avro schemas, indexed by their fingerprint.

`SchemaStore` provides a mechanism to store and retrieve Avro schemas efficiently.
Each schema is associated with a unique [`Fingerprint`](../operations/arrow_avro.schema.Fingerprint.md#op-cc45a13829ed050ac8b51e76), which is generated based
on the schema's canonical form and a specific hashing algorithm.

A `SchemaStore` instance is configured to use a single [`FingerprintAlgorithm`](../operations/arrow_avro.schema.FingerprintAlgorithm.md#op-1907322c8fa6ae7a21567f05) such as Rabin,
MD5 (not yet supported), or SHA256 (not yet supported) for all its operations.
This ensures consistency when generating fingerprints and looking up schemas.
All schemas registered will have their fingerprint computed with this algorithm, and
lookups must use a matching fingerprint.

# Examples

```no_run
// Create a new store with the default Rabin fingerprinting.
use arrow_avro::schema::{AvroSchema, SchemaStore};

let mut store = SchemaStore::new();
let schema = AvroSchema::new("\"string\"".to_string());
// Register the schema to get its fingerprint.
let fingerprint = store.register(schema.clone()).unwrap();
// Use the fingerprint to look up the schema.
let retrieved_schema = store.lookup(&fingerprint).cloned();
assert_eq!(retrieved_schema, Some(schema));
```

<a id="op-1c504eb9c5e356f864835f0f"></a>
## Error

`assoc_type` · `arrow_avro::schema::SchemaStore::Error` · arrow-avro 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [841, 1], "end": [852, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/schema.rs:842`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-690e719bc36570a3928ff5c2"></a>
## clone

`function` · `arrow_avro::schema::SchemaStore::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> SchemaStore
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [833, 17], "end": [833, 22], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema.rs:833`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7783cf08edc823799248fa2f"></a>
## default

`function` · `arrow_avro::schema::SchemaStore::default` · arrow-avro 59.3.0

```rust
fn default() -> SchemaStore
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [833, 24], "end": [833, 31], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/schema.rs:833`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab6ff6e3ca5d645f1eac5067"></a>
## fingerprints

`function` · `arrow_avro::schema::SchemaStore::fingerprints` · arrow-avro 59.3.0

```rust
fn fingerprints(&self) -> Vec<Fingerprint>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [854, 1], "end": [963, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:955`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns a `Vec` containing **all unique [`Fingerprint`](../operations/arrow_avro.schema.Fingerprint.md#op-cc45a13829ed050ac8b51e76)s** currently
held by this [`SchemaStore`](../operations/arrow_avro.schema.SchemaStore.md#op-3ef84c94db2621c1fb7a1dad).

The order of the returned fingerprints is unspecified and should not be
relied upon.

<a id="op-ffdc92fbebc077006df1cddb"></a>
## fmt

`function` · `arrow_avro::schema::SchemaStore::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [833, 10], "end": [833, 15], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema.rs:833`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f6e2dcd55463996245d7a74"></a>
## lookup

`function` · `arrow_avro::schema::SchemaStore::lookup` · arrow-avro 59.3.0

```rust
fn lookup(&self, fingerprint: &Fingerprint) -> Option<&AvroSchema>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [854, 1], "end": [963, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:946`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Looks up a schema by its `Fingerprint`.

# Arguments

* `fingerprint` - A reference to the `Fingerprint` of the schema to look up.

# Returns

An `Option` containing a clone of the `AvroSchema` if found, otherwise `None`.

<a id="op-8fb958ff3d768eedd8475f20"></a>
## new

`function` · `arrow_avro::schema::SchemaStore::new` · arrow-avro 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [854, 1], "end": [963, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:856`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Creates an empty `SchemaStore` using the default fingerprinting algorithm (64-bit Rabin).

<a id="op-db515137e661b95f57f77bc8"></a>
## new_with_type

`function` · `arrow_avro::schema::SchemaStore::new_with_type` · arrow-avro 59.3.0

```rust
fn new_with_type(fingerprint_algorithm: FingerprintAlgorithm) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [854, 1], "end": [963, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:861`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Creates an empty `SchemaStore` using the default fingerprinting algorithm (64-bit Rabin).

<a id="op-156db3c2243c8f9ad62aea96"></a>
## register

`function` · `arrow_avro::schema::SchemaStore::register` · arrow-avro 59.3.0

```rust
fn register(&mut self, schema: AvroSchema) -> Result<Fingerprint, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [854, 1], "end": [963, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Registers a schema with the store and returns its fingerprint.

A fingerprint is calculated for the given schema using the store's configured
hash type. If a schema with the same fingerprint does not already exist in the
store, the new schema is inserted. If the fingerprint already exists, the
existing schema is not overwritten. If FingerprintAlgorithm is set to Id or Id64, this
method will return an error. Confluent wire format implementations should leverage the
set method instead.

# Arguments

* `schema` - The `AvroSchema` to register.

# Returns

A `Result` containing the `Fingerprint` of the schema if successful,
or an `ArrowError` on failure.

<a id="op-4765d06e422847433755be39"></a>
## set

`function` · `arrow_avro::schema::SchemaStore::set` · arrow-avro 59.3.0

```rust
fn set(&mut self, fingerprint: Fingerprint, schema: AvroSchema) -> Result<Fingerprint, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [854, 1], "end": [963, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:884`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Registers a schema with the store and the provided fingerprint.
Note: Confluent wire format implementations should leverage this method.

A schema is set in the store, using the provided fingerprint. If a schema
with the same fingerprint does not already exist in the store, the new schema
is inserted. If the fingerprint already exists, the existing schema is not overwritten.

# Arguments

* `fingerprint` - A reference to the `Fingerprint` of the schema to register.
* `schema` - The `AvroSchema` to register.

# Returns

A `Result` returning the provided `Fingerprint` of the schema if successful,
or an `ArrowError` on failure.

<a id="op-6dd8e4fd3468cc4afa2156ef"></a>
## try_from

`function` · `arrow_avro::schema::SchemaStore::try_from` · arrow-avro 59.3.0

```rust
fn try_from(schemas: HashMap<Fingerprint, AvroSchema>) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::SchemaStore", "path": "SchemaStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [841, 1], "end": [852, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/schema.rs:846`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Creates a `SchemaStore` from a HashMap of schemas.
Each schema in the HashMap is registered with the new store.
