# `arrow_avro::schema::FingerprintAlgorithm`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.schema.FingerprintAlgorithm.json).

<a id="op-1907322c8fa6ae7a21567f05"></a>
## FingerprintAlgorithm

`enum` · `arrow_avro::schema::FingerprintAlgorithm` · arrow-avro 59.3.0

```rust
enum FingerprintAlgorithm
```

Source: `src/schema.rs:624`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Supported fingerprint algorithms for Avro schema identification.
For use with Confluent Schema Registry IDs, set to None.

<a id="op-ffce675108a151195e27ab23"></a>
## Id

`variant` · `arrow_avro::schema::FingerprintAlgorithm::Id` · arrow-avro 59.3.0

```rust
Id
```

Source: `src/schema.rs:629`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Represents a 32 bit fingerprint not based on a hash algorithm, (e.g., a 32-bit Schema Registry ID.)

<a id="op-f96e96837b4f3501148fc43d"></a>
## Id64

`variant` · `arrow_avro::schema::FingerprintAlgorithm::Id64` · arrow-avro 59.3.0

```rust
Id64
```

Source: `src/schema.rs:631`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Represents a 64 bit fingerprint not based on a hash algorithm, (e.g., a 64-bit Schema Registry ID.)

<a id="op-19f37a48d99938b3a993b4a4"></a>
## MD5

`variant` · `arrow_avro::schema::FingerprintAlgorithm::MD5` · arrow-avro 59.3.0

```rust
MD5
```

Source: `src/schema.rs:634`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

128-bit MD5 message digest.

<a id="op-aacd74fd01f81cdb9ac5359d"></a>
## Rabin

`variant` · `arrow_avro::schema::FingerprintAlgorithm::Rabin` · arrow-avro 59.3.0

```rust
Rabin
```

Source: `src/schema.rs:627`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

64‑bit CRC‑64‑AVRO Rabin fingerprint.

<a id="op-5a8d6bcc110c9b61eccf2a9f"></a>
## SHA256

`variant` · `arrow_avro::schema::FingerprintAlgorithm::SHA256` · arrow-avro 59.3.0

```rust
SHA256
```

Source: `src/schema.rs:637`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

256-bit SHA-256 digest.

<a id="op-8a6aba42be31a1f696148119"></a>
## clone

`function` · `arrow_avro::schema::FingerprintAlgorithm::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> FingerprintAlgorithm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 10], "end": [623, 15], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema.rs:623`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c43730729716c07f5e6f81"></a>
## default

`function` · `arrow_avro::schema::FingerprintAlgorithm::default` · arrow-avro 59.3.0

```rust
fn default() -> FingerprintAlgorithm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 51], "end": [623, 58], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/schema.rs:623`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f15008e03a27f3d227ccac23"></a>
## eq

`function` · `arrow_avro::schema::FingerprintAlgorithm::eq` · arrow-avro 59.3.0

```rust
fn eq(&self, other: &FingerprintAlgorithm) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 30], "end": [623, 39], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema.rs:623`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d0e9d7bee0274a1fe4ae5b2"></a>
## fmt

`function` · `arrow_avro::schema::FingerprintAlgorithm::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 23], "end": [623, 28], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema.rs:623`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7baeb49e6e9ff216a894e698"></a>
## from

`function` · `arrow_avro::schema::FingerprintAlgorithm::from` · arrow-avro 59.3.0

```rust
fn from(s: FingerprintStrategy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [655, 1], "end": [659, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:656`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dda377742f25591facfa856"></a>
## from

`function` · `arrow_avro::schema::FingerprintAlgorithm::from` · arrow-avro 59.3.0

```rust
fn from(fp: &Fingerprint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [653, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::Fingerprint", "path": "Fingerprint"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:642`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b968d42e0a4d0668c078c690"></a>
## from

`function` · `arrow_avro::schema::FingerprintAlgorithm::from` · arrow-avro 59.3.0

```rust
fn from(s: &FingerprintStrategy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 1], "end": [673, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintStrategy", "path": "FingerprintStrategy"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:662`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b241bb01bc905f3cce946497"></a>
## hash

`function` · `arrow_avro::schema::FingerprintAlgorithm::hash` · arrow-avro 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::schema::FingerprintAlgorithm", "path": "FingerprintAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 45], "end": [623, 49], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/schema.rs:623`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
