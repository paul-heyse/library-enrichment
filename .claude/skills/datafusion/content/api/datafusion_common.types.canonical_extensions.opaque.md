# `datafusion_common::types::canonical_extensions::opaque`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.canonical_extensions.opaque.json`](../model/datafusion_common.types.canonical_extensions.opaque.json)

## DFOpaque

`struct` · `datafusion_common::types::canonical_extensions::opaque::DFOpaque`

```rust
struct DFOpaque
```

**Implements**: `datafusion_common::types::extension::DFExtensionType`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, metadata: <Opaque as ExtensionType>::Metadata) -> Result<Self>
```

**via `datafusion_common::types::extension::DFExtensionType`**

```rust
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

Defines the extension type logic for the canonical `arrow.opaque` extension type. This extension
type represents types that DataFusion cannot interpret.

See [`DFExtensionType`] for information on DataFusion's extension type mechanism. See also
[`Opaque`] for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#opaque>

---
