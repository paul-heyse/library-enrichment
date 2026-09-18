# `datafusion_common::types::canonical_extensions::bool8`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.canonical_extensions.bool8.json`](../model/datafusion_common.types.canonical_extensions.bool8.json)

## DFBool8

`struct` · `datafusion_common::types::canonical_extensions::bool8::DFBool8`

```rust
struct DFBool8
```

**Implements**: `datafusion_common::types::extension::DFExtensionType`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, metadata: <Bool8 as ExtensionType>::Metadata) -> Result<Self>
```

**via `datafusion_common::types::extension::DFExtensionType`**

```rust
fn create_array_formatter<'fmt>(&self, array: &'fmt dyn Array, options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.types.canonical_extensions.bool8.DFBool8.md).


Defines the extension type logic for the canonical `arrow.bool8` extension type. This extension
type allows storing a Boolean value in a single byte, instead of a single bit.

See [`DFExtensionType`] for information on DataFusion's extension type mechanism. See also
[`Bool8`] for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#bit-boolean>

---
