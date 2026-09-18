# `arrow_schema::extension::canonical::opaque::OpaqueMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.opaque.OpaqueMetadata.json).

<a id="op-ee5a3484bc6b6840567b82d7"></a>
## OpaqueMetadata

`struct` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata` · arrow-schema 59.3.0

```rust
struct OpaqueMetadata
```

Source: `src/extension/canonical/opaque.rs:69`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Extension type metadata for [`Opaque`](../operations/arrow_schema.extension.canonical.opaque.Opaque.md#op-fb0ef27770fce0cdd79de96e).

<a id="op-81420b82f01f3a892177a69a"></a>
## clone

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> OpaqueMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 17], "end": [68, 22], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/opaque.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1820419f871d29ec8a19088"></a>
## deserialize

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [200, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/extension/canonical/opaque.rs:190`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-395a9612bee214e1ea80279b"></a>
## eq

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &OpaqueMetadata) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 24], "end": [68, 33], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/opaque.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3251be194d1a81b5bf1d2110"></a>
## fmt

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/opaque.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e91556815730349764d7f24"></a>
## new

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::new` · arrow-schema 59.3.0

```rust
fn new(type_name: impl Into<String>, vendor_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [220, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/opaque.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a new `OpaqueMetadata`.

<a id="op-2d0e08f6a394bb80ba6375df"></a>
## serialize

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::serialize` · arrow-schema 59.3.0

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [87, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/extension/canonical/opaque.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cc0363d49c1b4a18b138a1c"></a>
## type_name

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::type_name` · arrow-schema 59.3.0

```rust
fn type_name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [220, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/opaque.rs:212`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the name of the unknown type in the external system.

<a id="op-4c35b8378eadeb99e7f18e56"></a>
## vendor_name

`function` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata::vendor_name` · arrow-schema 59.3.0

```rust
fn vendor_name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [220, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/opaque.rs:217`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the name of the external system.
