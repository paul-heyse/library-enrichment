# `arrow_avro::schema::FingerprintStrategy`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.schema.FingerprintStrategy.json).

<a id="op-116d21f1e5d929bac771bced"></a>
## FingerprintStrategy

`enum` · `arrow_avro::schema::FingerprintStrategy` · arrow-avro 59.3.0

```rust
enum FingerprintStrategy
```

Source: `src/schema.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Defines the strategy for generating the per-record prefix for an Avro binary stream.

<a id="op-a407e114e41d64f760938974"></a>
## Id

`variant` · `arrow_avro::schema::FingerprintStrategy::Id` · arrow-avro 59.3.0

```rust
Id
```

Source: `src/schema.rs:576`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Use a Confluent Schema Registry 32-bit ID.

<a id="op-edab3084c23bb9231b0f7c9b"></a>
## Id64

`variant` · `arrow_avro::schema::FingerprintStrategy::Id64` · arrow-avro 59.3.0

```rust
Id64
```

Source: `src/schema.rs:578`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Use an Apicurio Schema Registry 64-bit ID.

<a id="op-44037b33de74fc21e1395fbf"></a>
## MD5

`variant` · `arrow_avro::schema::FingerprintStrategy::MD5` · arrow-avro 59.3.0

```rust
MD5
```

Source: `src/schema.rs:581`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Use the 128-bit MD5 fingerprint.

<a id="op-a5c94a640ceb27d4461ca1bb"></a>
## Rabin

`variant` · `arrow_avro::schema::FingerprintStrategy::Rabin` · arrow-avro 59.3.0

```rust
Rabin
```

Source: `src/schema.rs:574`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Use the 64-bit Rabin fingerprint (default for single-object encoding).

<a id="op-eb69e19fd97fddd55cfe3494"></a>
## SHA256

`variant` · `arrow_avro::schema::FingerprintStrategy::SHA256` · arrow-avro 59.3.0

```rust
SHA256
```

Source: `src/schema.rs:584`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Use the 256-bit SHA-256 fingerprint.

<a id="op-db7f1b92b895f192042f1f43"></a>
## clone

`function` · `arrow_avro::schema::FingerprintStrategy::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> FingerprintStrategy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 17], "end": [570, 22], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c44dd2b30fbd0e0743ea3b7f"></a>
## default

`function` · `arrow_avro::schema::FingerprintStrategy::default` · arrow-avro 59.3.0

```rust
fn default() -> FingerprintStrategy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 45], "end": [570, 52], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/schema.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8648a2d2819be69ad2f837d"></a>
## eq

`function` · `arrow_avro::schema::FingerprintStrategy::eq` · arrow-avro 59.3.0

```rust
fn eq(&self, other: &FingerprintStrategy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 30], "end": [570, 39], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eaaff33a1523058a87eaeb1"></a>
## fmt

`function` · `arrow_avro::schema::FingerprintStrategy::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 10], "end": [570, 15], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema.rs:570`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0515b53d5f9b5e100ec99df7"></a>
## from

`function` · `arrow_avro::schema::FingerprintStrategy::from` · arrow-avro 59.3.0

```rust
fn from(f: Fingerprint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [591, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:588`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51546e6931831cf11d812ae2"></a>
## from

`function` · `arrow_avro::schema::FingerprintStrategy::from` · arrow-avro 59.3.0

```rust
fn from(f: &Fingerprint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [607, 1], "end": [619, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:608`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8f5025738d0554591a9dc92"></a>
## from

`function` · `arrow_avro::schema::FingerprintStrategy::from` · arrow-avro 59.3.0

```rust
fn from(f: FingerprintAlgorithm) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [605, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:594`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
