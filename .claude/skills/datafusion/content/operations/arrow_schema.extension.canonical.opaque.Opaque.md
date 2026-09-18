# `arrow_schema::extension::canonical::opaque::Opaque`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.opaque.Opaque.json).

<a id="op-fb0ef27770fce0cdd79de96e"></a>
## Opaque

`struct` · `arrow_schema::extension::canonical::opaque::Opaque` · arrow-schema 59.3.0

```rust
struct Opaque
```

Source: `src/extension/canonical/opaque.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `Opaque`.

Extension name: `arrow.opaque`.

Opaque represents a type that an Arrow-based system received from an
external (often non-Arrow) system, but that it cannot interpret. In this
case, it can pass on Opaque to its clients to at least show that a field
exists and preserve metadata about the type from the other system.

The storage type of this extension is any type. If there is no underlying
data, the storage type should be Null.

<a id="op-e1c019c5171d91fa31ed56da"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::canonical::opaque::Opaque::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95a0c1b8aad283cbe1757dd0"></a>
## NAME

`assoc_const` · `arrow_schema::extension::canonical::opaque::Opaque::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ccb31551c88a161d61ae284"></a>
## clone

`function` · `arrow_schema::extension::canonical::opaque::Opaque::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> Opaque
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 22], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/opaque.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-019fdffbf378742f315237c3"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::canonical::opaque::Opaque::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:235`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fb2373e846c3a67538ec242"></a>
## eq

`function` · `arrow_schema::extension::canonical::opaque::Opaque::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &Opaque) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 24], "end": [41, 33], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/opaque.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3855dfe663e2a831fb09b47"></a>
## fmt

`function` · `arrow_schema::extension::canonical::opaque::Opaque::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/opaque.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1241d5564924f6c2a008b4a"></a>
## from

`function` · `arrow_schema::extension::canonical::opaque::Opaque::from` · arrow-schema 59.3.0

```rust
fn from(value: OpaqueMetadata) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [65, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::OpaqueMetadata", "path": "OpaqueMetadata"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/opaque.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f1e2ac25f7ed038fc2673d5"></a>
## metadata

`function` · `arrow_schema::extension::canonical::opaque::Opaque::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:227`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69924954ee86b883b68b59fa"></a>
## new

`function` · `arrow_schema::extension::canonical::opaque::Opaque::new` · arrow-schema 59.3.0

```rust
fn new(type_name: impl Into<String>, vendor_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [59, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/opaque.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a new `Opaque` extension type.

<a id="op-99a2e1b02bbd09207a2694ad"></a>
## serialize_metadata

`function` · `arrow_schema::extension::canonical::opaque::Opaque::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3039b0e5341d48d94fc455fa"></a>
## supports_data_type

`function` · `arrow_schema::extension::canonical::opaque::Opaque::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, _data_type: &DataType) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:252`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57474c0c2b1cbd04da1835c2"></a>
## try_new

`function` · `arrow_schema::extension::canonical::opaque::Opaque::try_new` · arrow-schema 59.3.0

```rust
fn try_new(_data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:257`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-319754d1c422a39cb5b8caf9"></a>
## type_name

`function` · `arrow_schema::extension::canonical::opaque::Opaque::type_name` · arrow-schema 59.3.0

```rust
fn type_name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [59, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/opaque.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the name of the unknown type in the external system.

<a id="op-f32345ab2ae19b9794a91625"></a>
## validate

`function` · `arrow_schema::extension::canonical::opaque::Opaque::validate` · arrow-schema 59.3.0

```rust
fn validate(_data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [264, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/opaque.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f13eb89da719a5b270b3051"></a>
## vendor_name

`function` · `arrow_schema::extension::canonical::opaque::Opaque::vendor_name` · arrow-schema 59.3.0

```rust
fn vendor_name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [59, 2], "filename": "src/extension/canonical/opaque.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/opaque.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the name of the external system.
