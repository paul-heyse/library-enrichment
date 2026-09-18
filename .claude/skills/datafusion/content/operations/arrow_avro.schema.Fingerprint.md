# `arrow_avro::schema::Fingerprint`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.schema.Fingerprint.json).

<a id="op-cc45a13829ed050ac8b51e76"></a>
## Fingerprint

`enum` · `arrow_avro::schema::Fingerprint` · arrow-avro 59.3.0

```rust
enum Fingerprint
```

Source: `src/schema.rs:684`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A schema fingerprint in one of the supported formats.

This is used as the key inside `SchemaStore` `HashMap`. Each `SchemaStore`
instance always stores only one variant, matching its configured
`FingerprintAlgorithm`, but the enum makes the API uniform.

<https://avro.apache.org/docs/1.11.1/specification/#schema-fingerprints>
<https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format>

<a id="op-68b6c129bea240d03db05031"></a>
## Id

`variant` · `arrow_avro::schema::Fingerprint::Id` · arrow-avro 59.3.0

```rust
Id
```

Source: `src/schema.rs:688`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A 32-bit Schema Registry ID.

<a id="op-83f4da808d3812cb0bded096"></a>
## Id64

`variant` · `arrow_avro::schema::Fingerprint::Id64` · arrow-avro 59.3.0

```rust
Id64
```

Source: `src/schema.rs:690`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A 64-bit Schema Registry ID.

<a id="op-fc428005d5c82f3b99586d49"></a>
## MD5

`variant` · `arrow_avro::schema::Fingerprint::MD5` · arrow-avro 59.3.0

```rust
MD5
```

Source: `src/schema.rs:693`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A 128-bit MD5 fingerprint.

<a id="op-8e5b115a0a7152ef41985121"></a>
## Rabin

`variant` · `arrow_avro::schema::Fingerprint::Rabin` · arrow-avro 59.3.0

```rust
Rabin
```

Source: `src/schema.rs:686`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A 64-bit Rabin fingerprint.

<a id="op-775f19885b0f3450c42658e9"></a>
## SHA256

`variant` · `arrow_avro::schema::Fingerprint::SHA256` · arrow-avro 59.3.0

```rust
SHA256
```

Source: `src/schema.rs:696`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A 256-bit SHA-256 fingerprint.

<a id="op-64f8da3b248d72e9a6574bf4"></a>
## clone

`function` · `arrow_avro::schema::Fingerprint::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> Fingerprint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 10], "end": [683, 15], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema.rs:683`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9f99b8f8774ad5dce92a424"></a>
## eq

`function` · `arrow_avro::schema::Fingerprint::eq` · arrow-avro 59.3.0

```rust
fn eq(&self, other: &Fingerprint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 30], "end": [683, 39], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema.rs:683`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd23091f200fcd8d26df6c03"></a>
## fmt

`function` · `arrow_avro::schema::Fingerprint::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 23], "end": [683, 28], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema.rs:683`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01b3318e0e776a539612f389"></a>
## from

`function` · `arrow_avro::schema::Fingerprint::from` · arrow-avro 59.3.0

```rust
fn from(s: FingerprintAlgorithm) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [719, 1], "end": [731, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:720`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-277655fb23f3a9dc7b384961"></a>
## from

`function` · `arrow_avro::schema::Fingerprint::from` · arrow-avro 59.3.0

```rust
fn from(s: FingerprintStrategy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [699, 1], "end": [703, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:700`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95f8475339aebb4d344d370d"></a>
## from

`function` · `arrow_avro::schema::Fingerprint::from` · arrow-avro 59.3.0

```rust
fn from(s: &FingerprintStrategy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [705, 1], "end": [717, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:706`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8344c44f6af20d3d8a3023a7"></a>
## hash

`function` · `arrow_avro::schema::Fingerprint::hash` · arrow-avro 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 45], "end": [683, 49], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/schema.rs:683`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8623be41806383dd29d3ed7a"></a>
## load_fingerprint_id

`function` · `arrow_avro::schema::Fingerprint::load_fingerprint_id` · arrow-avro 59.3.0

```rust
fn load_fingerprint_id(id: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [733, 1], "end": [792, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:741`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Loads the 32-bit Schema Registry fingerprint (Confluent Schema Registry ID).

The provided `id` is in big-endian wire order; this converts it to host order
and returns `Fingerprint::Id`.

# Returns
A `Fingerprint::Id` variant containing the 32-bit fingerprint.

<a id="op-f31a080d27889c406d7e5558"></a>
## load_fingerprint_id64

`function` · `arrow_avro::schema::Fingerprint::load_fingerprint_id64` · arrow-avro 59.3.0

```rust
fn load_fingerprint_id64(id: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [733, 1], "end": [792, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:752`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Loads the 64-bit Schema Registry fingerprint (Apicurio Schema Registry ID).

The provided `id` is in big-endian wire order; this converts it to host order
and returns `Fingerprint::Id64`.

# Returns
A `Fingerprint::Id64` variant containing the 64-bit fingerprint.
