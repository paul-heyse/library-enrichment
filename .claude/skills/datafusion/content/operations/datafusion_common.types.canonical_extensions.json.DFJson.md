# `datafusion_common::types::canonical_extensions::json::DFJson`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.canonical_extensions.json.DFJson.json).

<a id="op-72af1dee310bf13ef49ad4df"></a>
## DFJson

`struct` · `datafusion_common::types::canonical_extensions::json::DFJson` · datafusion-common 55.1.0

```rust
struct DFJson
```

Source: `src/types/canonical_extensions/json.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the extension type logic for the canonical `arrow.json` extension type. This extension
type defines that a particular string field stores JSON values.

See [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) for information on DataFusion's extension type mechanism. See also
[`Json`](../operations/arrow_schema.extension.canonical.json.Json.md#op-3263368fd37d8190e1022007) for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#json>

<a id="op-d59ae15703b3bfd674a98e96"></a>
## clone

`function` · `datafusion_common::types::canonical_extensions::json::DFJson::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFJson
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::json::DFJson", "path": "DFJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 22], "filename": "src/types/canonical_extensions/json.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/canonical_extensions/json.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7badd3fb995504ecb2682265"></a>
## fmt

`function` · `datafusion_common::types::canonical_extensions::json::DFJson::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::json::DFJson", "path": "DFJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/types/canonical_extensions/json.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/canonical_extensions/json.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1be85e5326f33a598b8159c"></a>
## serialize_metadata

`function` · `datafusion_common::types::canonical_extensions::json::DFJson::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::json::DFJson", "path": "DFJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [58, 2], "filename": "src/types/canonical_extensions/json.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/json.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d364d3d3dfa2f1191fffa8d3"></a>
## storage_type

`function` · `datafusion_common::types::canonical_extensions::json::DFJson::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::json::DFJson", "path": "DFJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [58, 2], "filename": "src/types/canonical_extensions/json.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/json.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1fb65864a092d82e5913ad1"></a>
## try_new

`function` · `datafusion_common::types::canonical_extensions::json::DFJson::try_new` · datafusion-common 55.1.0

```rust
fn try_new(data_type: &DataType, metadata: <Json as ExtensionType>::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::json::DFJson", "path": "DFJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [48, 2], "filename": "src/types/canonical_extensions/json.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/canonical_extensions/json.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`DFJson`](../operations/datafusion_common.types.canonical_extensions.json.DFJson.md#op-72af1dee310bf13ef49ad4df), validating that the storage type is compatible with the
extension type.
