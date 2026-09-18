# `arrow_schema::extension::canonical::bool8::Bool8`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.bool8.Bool8.json).

<a id="op-e2d6046499dad66c7afe7d66"></a>
## Bool8

`struct` · `arrow_schema::extension::canonical::bool8::Bool8` · arrow-schema 59.3.0

```rust
struct Bool8
```

Source: `src/extension/canonical/bool8.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `8-bit Boolean`.

Extension name: `arrow.bool8`.

The storage type of the extension is `Int8` where:
- false is denoted by the value 0.
- true can be specified using any non-zero value. Preferably 1.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#bit-boolean>

<a id="op-cc300432f8c198a4dca9b99a"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::canonical::bool8::Bool8::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a8283745e52e85d26d974c3"></a>
## NAME

`assoc_const` · `arrow_schema::extension::canonical::bool8::Bool8::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48716eeadbd6668fb237ea89"></a>
## clone

`function` · `arrow_schema::extension::canonical::bool8::Bool8::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> Bool8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 26], "end": [33, 31], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/bool8.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff5557d5856b0b61209b9d2c"></a>
## default

`function` · `arrow_schema::extension::canonical::bool8::Bool8::default` · arrow-schema 59.3.0

```rust
fn default() -> Bool8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 24], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extension/canonical/bool8.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faf3dcba4632b1bf380e9cb5"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::canonical::bool8::Bool8::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-693150e596abf2420d3d1973"></a>
## eq

`function` · `arrow_schema::extension::canonical::bool8::Bool8::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &Bool8) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 39], "end": [33, 48], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/bool8.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b4361ad24832207d2f7cccc"></a>
## fmt

`function` · `arrow_schema::extension::canonical::bool8::Bool8::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/bool8.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd6d4fed486d5e36ee278f28"></a>
## metadata

`function` · `arrow_schema::extension::canonical::bool8::Bool8::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af23d798f3791c6d80b00a1a"></a>
## serialize_metadata

`function` · `arrow_schema::extension::canonical::bool8::Bool8::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-546e18d5c652f71a42275cdd"></a>
## supports_data_type

`function` · `arrow_schema::extension::canonical::bool8::Bool8::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bd09ae99f26eb7501cb1c89"></a>
## try_new

`function` · `arrow_schema::extension::canonical::bool8::Bool8::try_new` · arrow-schema 59.3.0

```rust
fn try_new(data_type: &DataType, _metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-197c93c1ab4896b696b90489"></a>
## validate

`function` · `arrow_schema::extension::canonical::bool8::Bool8::validate` · arrow-schema 59.3.0

```rust
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [75, 2], "filename": "src/extension/canonical/bool8.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/bool8.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
