# `arrow_schema::extension::canonical::uuid::Uuid`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.uuid.Uuid.json).

<a id="op-0e9bc650e20d322ee4024590"></a>
## Uuid

`struct` · `arrow_schema::extension::canonical::uuid::Uuid` · arrow-schema 59.3.0

```rust
struct Uuid
```

Source: `src/extension/canonical/uuid.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `UUID`.

Extension name: `arrow.uuid`.

The storage type of the extension is `FixedSizeBinary` with a length of
16 bytes.

Note:
A specific UUID version is not required or guaranteed. This extension
represents UUIDs as `FixedSizeBinary(16)` with big-endian notation and
does not interpret the bytes in any way.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#uuid>

<a id="op-9f2ebd2d91549a9c40ef86ce"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::canonical::uuid::Uuid::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c60957d328f26266c798179d"></a>
## NAME

`assoc_const` · `arrow_schema::extension::canonical::uuid::Uuid::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb13a185cedc7332fe1fb901"></a>
## clone

`function` · `arrow_schema::extension::canonical::uuid::Uuid::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> Uuid
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 26], "end": [37, 31], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/uuid.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ed5d0434aebe06965b85468"></a>
## default

`function` · `arrow_schema::extension::canonical::uuid::Uuid::default` · arrow-schema 59.3.0

```rust
fn default() -> Uuid
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 24], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extension/canonical/uuid.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b01bbd3672a5ef977ca3380"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::canonical::uuid::Uuid::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7da37c7de23c48c8df4b3370"></a>
## eq

`function` · `arrow_schema::extension::canonical::uuid::Uuid::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &Uuid) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 39], "end": [37, 48], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/uuid.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bab8e3805a52519cf2ece53"></a>
## fmt

`function` · `arrow_schema::extension::canonical::uuid::Uuid::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/uuid.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ab884720ed3450bd8d876e1"></a>
## metadata

`function` · `arrow_schema::extension::canonical::uuid::Uuid::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-742fea5d134e79253c057e5c"></a>
## serialize_metadata

`function` · `arrow_schema::extension::canonical::uuid::Uuid::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc385d3369a82844f99b5128"></a>
## supports_data_type

`function` · `arrow_schema::extension::canonical::uuid::Uuid::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1140c91c8ebcbf852284a45b"></a>
## try_new

`function` · `arrow_schema::extension::canonical::uuid::Uuid::try_new` · arrow-schema 59.3.0

```rust
fn try_new(data_type: &DataType, _metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2946c1876fac0cab99826979"></a>
## validate

`function` · `arrow_schema::extension::canonical::uuid::Uuid::validate` · arrow-schema 59.3.0

```rust
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [84, 2], "filename": "src/extension/canonical/uuid.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/uuid.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
