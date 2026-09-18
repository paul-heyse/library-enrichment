# `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.canonical_extensions.variable_shape_tensor.DFVariableShapeTensor.json).

<a id="op-728a9dff3d99c396b44d840a"></a>
## DFVariableShapeTensor

`struct` · `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor` · datafusion-common 55.1.0

```rust
struct DFVariableShapeTensor
```

Source: `src/types/canonical_extensions/variable_shape_tensor.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the extension type logic for the canonical `arrow.variable_shape_tensor` extension type.
This extension type can be used to store a [tensor](https://en.wikipedia.org/wiki/Tensor) with
variable shape that can change for each element.

See [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) for information on DataFusion's extension type mechanism. See also
[`VariableShapeTensor`](../operations/arrow_schema.extension.canonical.variable_shape_tensor.VariableShapeTensor.md#op-1cbb0485c49d5171a60c3365) for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#variable-shape-tensor>

<a id="op-fdd02f6262005366e20589bc"></a>
## clone

`function` · `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFVariableShapeTensor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor", "path": "DFVariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "src/types/canonical_extensions/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/canonical_extensions/variable_shape_tensor.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ff84d625462f548e76e1c67"></a>
## fmt

`function` · `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor", "path": "DFVariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/types/canonical_extensions/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/canonical_extensions/variable_shape_tensor.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-698c6b39232d62e82ead09dc"></a>
## serialize_metadata

`function` · `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor", "path": "DFVariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [62, 2], "filename": "src/types/canonical_extensions/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/variable_shape_tensor.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3de822d18d1e26406b962631"></a>
## storage_type

`function` · `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor", "path": "DFVariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [62, 2], "filename": "src/types/canonical_extensions/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/variable_shape_tensor.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e0c67a7b3d7a8fe91e8eb7b"></a>
## try_new

`function` · `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor::try_new` · datafusion-common 55.1.0

```rust
fn try_new(data_type: &DataType, metadata: <VariableShapeTensor as ExtensionType>::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor", "path": "DFVariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [52, 2], "filename": "src/types/canonical_extensions/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/canonical_extensions/variable_shape_tensor.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`DFVariableShapeTensor`](../operations/datafusion_common.types.canonical_extensions.variable_shape_tensor.DFVariableShapeTensor.md#op-728a9dff3d99c396b44d840a), validating that the storage type is compatible with
the extension type.
