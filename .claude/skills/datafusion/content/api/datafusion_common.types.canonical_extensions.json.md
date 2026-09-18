# `datafusion_common::types::canonical_extensions::json`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.canonical_extensions.json.json`](../model/datafusion_common.types.canonical_extensions.json.json)

## DFJson

`struct` · `datafusion_common::types::canonical_extensions::json::DFJson`

```rust
struct DFJson
```

**Implements**: `datafusion_common::types::extension::DFExtensionType`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, metadata: <Json as ExtensionType>::Metadata) -> Result<Self>
```

**via `datafusion_common::types::extension::DFExtensionType`**

```rust
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.types.canonical_extensions.json.DFJson.md).


Defines the extension type logic for the canonical `arrow.json` extension type. This extension
type defines that a particular string field stores JSON values.

See [`DFExtensionType`] for information on DataFusion's extension type mechanism. See also
[`Json`] for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#json>

---
