# `datafusion_common::types::canonical_extensions::fixed_shape_tensor`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.canonical_extensions.fixed_shape_tensor.json`](../model/datafusion_common.types.canonical_extensions.fixed_shape_tensor.json)

## DFFixedShapeTensor

`struct` · `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor`

```rust
struct DFFixedShapeTensor
```

**Implements**: `datafusion_common::types::extension::DFExtensionType`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, metadata: <FixedShapeTensor as ExtensionType>::Metadata) -> Result<Self>
```

**via `datafusion_common::types::extension::DFExtensionType`**

```rust
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.types.canonical_extensions.fixed_shape_tensor.DFFixedShapeTensor.md).


Defines the extension type logic for the canonical `arrow.fixed_shape_tensor` extension type.
This extension type can be used to store a [tensor](https://en.wikipedia.org/wiki/Tensor) of
a fixed shape.

See [`DFExtensionType`] for information on DataFusion's extension type mechanism. See also
[`FixedShapeTensor`] for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#fixed-shape-tensor>

---
