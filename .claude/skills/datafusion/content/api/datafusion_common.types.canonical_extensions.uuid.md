# `datafusion_common::types::canonical_extensions::uuid`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.canonical_extensions.uuid.json`](../model/datafusion_common.types.canonical_extensions.uuid.json)

## DFUuid

`struct` · `datafusion_common::types::canonical_extensions::uuid::DFUuid`

```rust
struct DFUuid
```

**Implements**: `datafusion_common::types::extension::DFExtensionType`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, metadata: <Uuid as ExtensionType>::Metadata) -> Result<Self>
```

**via `datafusion_common::types::extension::DFExtensionType`**

```rust
fn create_array_formatter<'fmt>(&self, array: &'fmt dyn Array, options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

Defines the extension type logic for the canonical `arrow.uuid` extension type. This extension
type defines that a field should be interpreted as a
[UUID](https://de.wikipedia.org/wiki/Universally_Unique_Identifier).

See [`DFExtensionType`] for information on DataFusion's extension type mechanism. See also
[`Uuid`] for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#uuid>

---
