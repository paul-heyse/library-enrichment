# `datafusion_common::types::canonical_extensions::variable_shape_tensor`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.canonical_extensions.variable_shape_tensor.json`](../model/datafusion_common.types.canonical_extensions.variable_shape_tensor.json)

## DFVariableShapeTensor

`struct` · `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor`

```rust
struct DFVariableShapeTensor
```

**Implements**: `datafusion_common::types::extension::DFExtensionType`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, metadata: <VariableShapeTensor as ExtensionType>::Metadata) -> Result<Self>
```

**via `datafusion_common::types::extension::DFExtensionType`**

```rust
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

Defines the extension type logic for the canonical `arrow.variable_shape_tensor` extension type.
This extension type can be used to store a [tensor](https://en.wikipedia.org/wiki/Tensor) with
variable shape that can change for each element.

See [`DFExtensionType`] for information on DataFusion's extension type mechanism. See also
[`VariableShapeTensor`] for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#variable-shape-tensor>

---
