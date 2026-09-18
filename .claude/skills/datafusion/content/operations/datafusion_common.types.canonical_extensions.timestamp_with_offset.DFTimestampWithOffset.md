# `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.canonical_extensions.timestamp_with_offset.DFTimestampWithOffset.json).

<a id="op-566f848e320078eb76d18199"></a>
## DFTimestampWithOffset

`struct` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset` · datafusion-common 55.1.0

```rust
struct DFTimestampWithOffset
```

Source: `src/types/canonical_extensions/timestamp_with_offset.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the extension type logic for the canonical `arrow.timestamp_with_offset` extension type.
This extension type allows associating a different offset for each timestamp in a column.

See [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) for information on DataFusion's extension type mechanism. See also
[`TimestampWithOffset`](../operations/arrow_schema.extension.canonical.timestamp_with_offset.TimestampWithOffset.md#op-9338a1b8a578c65b76007334) for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#timestamp-with-offset>

<a id="op-952ce8ee99253ebd58fa5cae"></a>
## clone

`function` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFTimestampWithOffset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset", "path": "DFTimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 22], "filename": "src/types/canonical_extensions/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/canonical_extensions/timestamp_with_offset.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff8f36dd18579038f801fd42"></a>
## create_array_formatter

`function` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset::create_array_formatter` · datafusion-common 55.1.0

```rust
fn create_array_formatter<'fmt>(&self, array: &'fmt dyn Array, options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset", "path": "DFTimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [108, 2], "filename": "src/types/canonical_extensions/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/timestamp_with_offset.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-673344c4ecd115f4812a3cb9"></a>
## fmt

`function` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset", "path": "DFTimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/types/canonical_extensions/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/canonical_extensions/timestamp_with_offset.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-135f19f9ef725cca6c849524"></a>
## serialize_metadata

`function` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset", "path": "DFTimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [108, 2], "filename": "src/types/canonical_extensions/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/timestamp_with_offset.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5ae5b85248d85d45ab84edc"></a>
## storage_type

`function` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset", "path": "DFTimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [108, 2], "filename": "src/types/canonical_extensions/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/timestamp_with_offset.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-297789f140c7899c180a1343"></a>
## try_new

`function` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset::try_new` · datafusion-common 55.1.0

```rust
fn try_new(data_type: &DataType, metadata: <TimestampWithOffset as ExtensionType>::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset", "path": "DFTimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [59, 2], "filename": "src/types/canonical_extensions/timestamp_with_offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/canonical_extensions/timestamp_with_offset.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`DFTimestampWithOffset`](../operations/datafusion_common.types.canonical_extensions.timestamp_with_offset.DFTimestampWithOffset.md#op-566f848e320078eb76d18199), validating that the storage type is compatible with
the extension type.
