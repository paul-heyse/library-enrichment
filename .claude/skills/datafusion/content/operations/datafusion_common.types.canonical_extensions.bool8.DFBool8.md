# `datafusion_common::types::canonical_extensions::bool8::DFBool8`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.canonical_extensions.bool8.DFBool8.json).

<a id="op-f4e46780a3da188b70c53752"></a>
## DFBool8

`struct` · `datafusion_common::types::canonical_extensions::bool8::DFBool8` · datafusion-common 55.1.0

```rust
struct DFBool8
```

Source: `src/types/canonical_extensions/bool8.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the extension type logic for the canonical `arrow.bool8` extension type. This extension
type allows storing a Boolean value in a single byte, instead of a single bit.

See [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) for information on DataFusion's extension type mechanism. See also
[`Bool8`](../operations/arrow_schema.extension.canonical.bool8.Bool8.md#op-e2d6046499dad66c7afe7d66) for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#bit-boolean>

<a id="op-8cdce20f6dd56be3260e1e38"></a>
## clone

`function` · `datafusion_common::types::canonical_extensions::bool8::DFBool8::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFBool8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::bool8::DFBool8", "path": "DFBool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 22], "filename": "src/types/canonical_extensions/bool8.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/canonical_extensions/bool8.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa9cd1564f3790f675d5ee8"></a>
## create_array_formatter

`function` · `datafusion_common::types::canonical_extensions::bool8::DFBool8::create_array_formatter` · datafusion-common 55.1.0

```rust
fn create_array_formatter<'fmt>(&self, array: &'fmt dyn Array, options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::bool8::DFBool8", "path": "DFBool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [82, 2], "filename": "src/types/canonical_extensions/bool8.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/bool8.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b692d67512d41c44076b8de9"></a>
## fmt

`function` · `datafusion_common::types::canonical_extensions::bool8::DFBool8::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::bool8::DFBool8", "path": "DFBool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/types/canonical_extensions/bool8.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/canonical_extensions/bool8.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72657e91898b2bad6e7142f5"></a>
## serialize_metadata

`function` · `datafusion_common::types::canonical_extensions::bool8::DFBool8::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::bool8::DFBool8", "path": "DFBool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [82, 2], "filename": "src/types/canonical_extensions/bool8.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/bool8.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6941a281214c9e4e12e05ef"></a>
## storage_type

`function` · `datafusion_common::types::canonical_extensions::bool8::DFBool8::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::bool8::DFBool8", "path": "DFBool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [82, 2], "filename": "src/types/canonical_extensions/bool8.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/bool8.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0d77ad77c72ea16eb75db93"></a>
## try_new

`function` · `datafusion_common::types::canonical_extensions::bool8::DFBool8::try_new` · datafusion-common 55.1.0

```rust
fn try_new(data_type: &DataType, metadata: <Bool8 as ExtensionType>::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::bool8::DFBool8", "path": "DFBool8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [53, 2], "filename": "src/types/canonical_extensions/bool8.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/canonical_extensions/bool8.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`DFBool8`](../operations/datafusion_common.types.canonical_extensions.bool8.DFBool8.md#op-f4e46780a3da188b70c53752), validating that the storage type is compatible with the
extension type.

Even though [`DFBool8`](../operations/datafusion_common.types.canonical_extensions.bool8.DFBool8.md#op-f4e46780a3da188b70c53752) only supports a single storage type ([`DataType::Int8`](../operations/arrow_schema.datatype.DataType.md#op-fb7865a9fb1164b7d5738475)), passing-in
the storage type allows conveniently validating whether this extension type is compatible
with a given [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).
