# `arrow_avro::schema::AvroSchema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.schema.AvroSchema.json).

<a id="op-f729faaac55e8a6344cab64e"></a>
## AvroSchema

`struct` · `arrow_avro::schema::AvroSchema` · arrow-avro 59.3.0

```rust
struct AvroSchema
```

Source: `src/schema.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A wrapper for an Avro schema in its JSON string representation.

<a id="op-98b214f5af07c3d966d0d81d"></a>
## Error

`assoc_type` · `arrow_avro::schema::AvroSchema::Error` · arrow-avro 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [355, 1], "end": [364, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/schema.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82627fb5fd7a1a31deff4d3b"></a>
## clone

`function` · `arrow_avro::schema::AvroSchema::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> AvroSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 17], "end": [349, 22], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ee77a75454589c32f3ca527"></a>
## deserialize

`function` · `arrow_avro::schema::AvroSchema::deserialize` · arrow-avro 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 50], "end": [349, 61], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/schema.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-867b65fc887d2b6f7b36504a"></a>
## eq

`function` · `arrow_avro::schema::AvroSchema::eq` · arrow-avro 59.3.0

```rust
fn eq(&self, other: &AvroSchema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 24], "end": [349, 33], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-261e9a0fcf5b7f66d5926264"></a>
## fingerprint

`function` · `arrow_avro::schema::AvroSchema::fingerprint` · arrow-avro 59.3.0

```rust
fn fingerprint(&self, hash_type: FingerprintAlgorithm) -> Result<Fingerprint, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [553, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:404`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the fingerprint of the schema, computed using the specified [`FingerprintAlgorithm`](../operations/arrow_avro.schema.FingerprintAlgorithm.md#op-1907322c8fa6ae7a21567f05).

The fingerprint is computed over the schema's Parsed Canonical Form
as defined by the Avro specification. Depending on `hash_type`, this
will return one of the supported [`Fingerprint`](../operations/arrow_avro.schema.Fingerprint.md#op-cc45a13829ed050ac8b51e76) variants:
- [`Fingerprint::Rabin`](../operations/arrow_avro.schema.Fingerprint.md#op-8e5b115a0a7152ef41985121) for [`FingerprintAlgorithm::Rabin`](../operations/arrow_avro.schema.FingerprintAlgorithm.md#op-aacd74fd01f81cdb9ac5359d)
- `Fingerprint::MD5` for `FingerprintAlgorithm::MD5`
- `Fingerprint::SHA256` for `FingerprintAlgorithm::SHA256`

Note: [`FingerprintAlgorithm::Id`](../operations/arrow_avro.schema.FingerprintAlgorithm.md#op-ffce675108a151195e27ab23) or [`FingerprintAlgorithm::Id64`](../operations/arrow_avro.schema.FingerprintAlgorithm.md#op-f96e96837b4f3501148fc43d) cannot be used to generate a fingerprint
and will result in an error. If you intend to use a Schema Registry ID-based
wire format, either use [`SchemaStore::set`](../operations/arrow_avro.schema.SchemaStore.md#op-4765d06e422847433755be39) or load the [`Fingerprint::Id`](../operations/arrow_avro.schema.Fingerprint.md#op-68b6c129bea240d03db05031) directly via [`Fingerprint::load_fingerprint_id`](../operations/arrow_avro.schema.Fingerprint.md#op-8623be41806383dd29d3ed7a) or for
[`Fingerprint::Id64`](../operations/arrow_avro.schema.Fingerprint.md#op-83f4da808d3812cb0bded096) via [`Fingerprint::load_fingerprint_id64`](../operations/arrow_avro.schema.Fingerprint.md#op-f31a080d27889c406d7e5558).

See also: <https://avro.apache.org/docs/1.11.1/specification/#schema-fingerprints>

# Errors
Returns an error if deserializing the schema fails, if generating the
canonical form of the schema fails, or if `hash_type` is [`FingerprintAlgorithm::Id`](../operations/arrow_avro.schema.FingerprintAlgorithm.md#op-ffce675108a151195e27ab23).

# Examples
```
use arrow_avro::schema::{AvroSchema, FingerprintAlgorithm};

let avro = AvroSchema::new("\"string\"".to_string());
let fp = avro.fingerprint(FingerprintAlgorithm::Rabin).unwrap();
```

<a id="op-e1a3df64e4dfd2b15e4f803e"></a>
## fmt

`function` · `arrow_avro::schema::AvroSchema::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 10], "end": [349, 15], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e4854d1d326786a69630b37"></a>
## json_string

`struct_field` · `arrow_avro::schema::AvroSchema::json_string` · arrow-avro 59.3.0

```rust
json_string: String
```

Source: `src/schema.rs:352`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

The Avro schema as a JSON string.

<a id="op-64aee39f1c56024554845e80"></a>
## new

`function` · `arrow_avro::schema::AvroSchema::new` · arrow-avro 59.3.0

```rust
fn new(json_string: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [553, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Creates a new `AvroSchema` from a JSON string.

<a id="op-3b089458d2e09ed79921130c"></a>
## serialize

`function` · `arrow_avro::schema::AvroSchema::serialize` · arrow-avro 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 39], "end": [349, 48], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/schema.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acc9cd5f060a4ac76110b7c1"></a>
## try_from

`function` · `arrow_avro::schema::AvroSchema::try_from` · arrow-avro 59.3.0

```rust
fn try_from(schema: &ArrowSchema) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::AvroSchema", "path": "AvroSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [355, 1], "end": [364, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/schema.rs:361`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Converts an `ArrowSchema` to `AvroSchema`, delegating to
`AvroSchema::from_arrow_with_options` with `None` so that the
union null ordering is decided by `Nullability::default()`.
