# `arrow_schema::extension::canonical::fixed_shape_tensor`

Crate `arrow-schema` · 2 public items · structured records in [`model/arrow_schema.extension.canonical.fixed_shape_tensor.json`](../model/arrow_schema.extension.canonical.fixed_shape_tensor.json)

## FixedShapeTensor

`struct` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor`

```rust
struct FixedShapeTensor
```

**Implements**: `arrow_schema::extension::ExtensionType`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn dimension_names(&self) -> Option<&[String]>
fn dimensions(&self) -> usize
fn list_size(&self) -> usize
fn permutations(&self) -> Option<&[usize]>
fn try_new(value_type: DataType, shape: impl IntoIterator<Item = usize>, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>) -> Result<Self, ArrowError>
fn value_type(&self) -> &DataType
```

**via `arrow_schema::extension::ExtensionType`**

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
fn metadata(&self) -> &Self::Metadata
fn serialize_metadata(&self) -> Option<String>
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.canonical.fixed_shape_tensor.FixedShapeTensor.md).


The extension type for fixed shape tensor.

Extension name: `arrow.fixed_shape_tensor`.

The storage type of the extension: `FixedSizeList` where:
- `value_type` is the data type of individual tensor elements.
- `list_size` is the product of all the elements in tensor shape.

Extension type parameters:
- `value_type`: the Arrow data type of individual tensor elements.
- `shape`: the physical shape of the contained tensors as an array.

Optional parameters describing the logical layout:
- `dim_names`: explicit names to tensor dimensions as an array. The
  length of it should be equal to the shape length and equal to the
  number of dimensions.
  `dim_names` can be used if the dimensions have
  well-known names and they map to the physical layout (row-major).
- `permutation`: indices of the desired ordering of the original
  dimensions, defined as an array.
  The indices contain a permutation of the values `[0, 1, .., N-1]`
  where `N` is the number of dimensions. The permutation indicates
  which dimension of the logical layout corresponds to which dimension
  of the physical tensor (the i-th dimension of the logical view
  corresponds to the dimension with number `permutations[i]` of the
  physical tensor).
  Permutation can be useful in case the logical order of the tensor is
  a permutation of the physical order (row-major).
  When logical and physical layout are equal, the permutation will
  always be `([0, 1, .., N-1])` and can therefore be left out.

Description of the serialization:
The metadata must be a valid JSON object including shape of the
contained tensors as an array with key `shape` plus optional
dimension names with keys `dim_names` and ordering of the
dimensions with key `permutation`.
Example: `{ "shape": [2, 5]}`
Example with `dim_names` metadata for NCHW ordered data:
`{ "shape": [100, 200, 500], "dim_names": ["C", "H", "W"]}`
Example of permuted 3-dimensional tensor:
`{ "shape": [100, 200, 500], "permutation": [2, 0, 1]}`

This is the physical layout shape and the shape of the logical layout
would in this case be `[500, 100, 200]`.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#fixed-shape-tensor>

---

## FixedShapeTensorMetadata

`struct` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata`

```rust
struct FixedShapeTensorMetadata
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn dimension_names(&self) -> Option<&[String]>
fn dimensions(&self) -> usize
fn list_size(&self) -> usize
fn permutations(&self) -> Option<&[usize]>
fn try_new(shape: impl IntoIterator<Item = usize>, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>) -> Result<Self, ArrowError>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.canonical.fixed_shape_tensor.FixedShapeTensorMetadata.md).


Extension type metadata for [`FixedShapeTensor`].

---
