# `datafusion_common::types::canonical_extensions::timestamp_with_offset`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.canonical_extensions.timestamp_with_offset.json`](../model/datafusion_common.types.canonical_extensions.timestamp_with_offset.json)

## DFTimestampWithOffset

`struct` · `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset`

```rust
struct DFTimestampWithOffset
```

**Implements**: `datafusion_common::types::extension::DFExtensionType`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, metadata: <TimestampWithOffset as ExtensionType>::Metadata) -> Result<Self>
```

**via `datafusion_common::types::extension::DFExtensionType`**

```rust
fn create_array_formatter<'fmt>(&self, array: &'fmt dyn Array, options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

Defines the extension type logic for the canonical `arrow.timestamp_with_offset` extension type.
This extension type allows associating a different offset for each timestamp in a column.

See [`DFExtensionType`] for information on DataFusion's extension type mechanism. See also
[`TimestampWithOffset`] for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#timestamp-with-offset>

---
