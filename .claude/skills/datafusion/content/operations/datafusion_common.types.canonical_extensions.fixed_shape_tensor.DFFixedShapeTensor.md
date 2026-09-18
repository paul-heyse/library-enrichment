# `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.canonical_extensions.fixed_shape_tensor.DFFixedShapeTensor.json).

<a id="op-7f5891de80d85008f0780028"></a>
## DFFixedShapeTensor

`struct` · `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor` · datafusion-common 55.1.0

```rust
struct DFFixedShapeTensor
```

Source: `src/types/canonical_extensions/fixed_shape_tensor.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the extension type logic for the canonical `arrow.fixed_shape_tensor` extension type.
This extension type can be used to store a [tensor](https://en.wikipedia.org/wiki/Tensor) of
a fixed shape.

See [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) for information on DataFusion's extension type mechanism. See also
[`FixedShapeTensor`](../operations/arrow_schema.extension.canonical.fixed_shape_tensor.FixedShapeTensor.md#op-49f2fa285749532be29d6feb) for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#fixed-shape-tensor>

<a id="op-b9f70e7a33a13335de1aa0c5"></a>
## clone

`function` · `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFFixedShapeTensor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor", "path": "DFFixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "src/types/canonical_extensions/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/canonical_extensions/fixed_shape_tensor.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04c4274eda2ed96dd5606db4"></a>
## fmt

`function` · `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor", "path": "DFFixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/types/canonical_extensions/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/canonical_extensions/fixed_shape_tensor.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53873503910fe13cb592b956"></a>
## serialize_metadata

`function` · `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor", "path": "DFFixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [64, 2], "filename": "src/types/canonical_extensions/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/fixed_shape_tensor.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19b84734e11fd6f891d9341"></a>
## storage_type

`function` · `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor", "path": "DFFixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [64, 2], "filename": "src/types/canonical_extensions/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/fixed_shape_tensor.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc12c252df46b4df4858d1bc"></a>
## try_new

`function` · `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor::try_new` · datafusion-common 55.1.0

```rust
fn try_new(data_type: &DataType, metadata: <FixedShapeTensor as ExtensionType>::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor", "path": "DFFixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [54, 2], "filename": "src/types/canonical_extensions/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/canonical_extensions/fixed_shape_tensor.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`DFFixedShapeTensor`](../operations/datafusion_common.types.canonical_extensions.fixed_shape_tensor.DFFixedShapeTensor.md#op-7f5891de80d85008f0780028), validating that the storage type is compatible with
the extension type.
