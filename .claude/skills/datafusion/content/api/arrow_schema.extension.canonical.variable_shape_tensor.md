# `arrow_schema::extension::canonical::variable_shape_tensor`

Crate `arrow-schema` · 2 public items · structured records in [`model/arrow_schema.extension.canonical.variable_shape_tensor.json`](../model/arrow_schema.extension.canonical.variable_shape_tensor.json)

## VariableShapeTensor

`struct` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor`

```rust
struct VariableShapeTensor
```

**Implements**: `arrow_schema::extension::ExtensionType`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn dimension_names(&self) -> Option<&[String]>
fn dimensions(&self) -> usize
fn permutations(&self) -> Option<&[usize]>
fn try_new(value_type: DataType, dimensions: usize, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>, uniform_shapes: Option<Vec<Option<i32>>>) -> Result<Self, ArrowError>
fn uniform_shapes(&self) -> Option<&[Option<i32>]>
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

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.canonical.variable_shape_tensor.VariableShapeTensor.md).


The extension type for `VariableShapeTensor`.

Extension name: `arrow.variable_shape_tensor`.

The storage type of the extension is: StructArray where struct is composed
of data and shape fields describing a single tensor per row:
- `data` is a List holding tensor elements (each list element is a single
  tensor). The List’s value type is the value type of the tensor, such as
  an integer or floating-point type.
- `shape` is a `FixedSizeList<int32>[ndim]` of the tensor shape where the
  size of the list `ndim` is equal to the number of dimensions of the
  tensor.

Extension type parameters:
`value_type`: the Arrow data type of individual tensor elements.

Optional parameters describing the logical layout:
- `dim_names`: explicit names to tensor dimensions as an array. The length
  of it should be equal to the shape length and equal to the number of
  dimensions.
  `dim_names` can be used if the dimensions have well-known names and they
  map to the physical layout (row-major).
- `permutation`: indices of the desired ordering of the original
  dimensions, defined as an array.
  The indices contain a permutation of the values `[0, 1, .., N-1]` where
  `N` is the number of dimensions. The permutation indicates which
  dimension of the logical layout corresponds to which dimension of the
  physical tensor (the i-th dimension of the logical view corresponds to
  the dimension with number `permutations[i]` of the physical tensor).
  Permutation can be useful in case the logical order of the tensor is a
  permutation of the physical order (row-major).
  When logical and physical layout are equal, the permutation will always
  be (`[0, 1, .., N-1]`) and can therefore be left out.
- `uniform_shape`: sizes of individual tensor’s dimensions which are
  guaranteed to stay constant in uniform dimensions and can vary in non-
  uniform dimensions. This holds over all tensors in the array. Sizes in
  uniform dimensions are represented with int32 values, while sizes of the
  non-uniform dimensions are not known in advance and are represented with
  null. If `uniform_shape` is not provided it is assumed that all
  dimensions are non-uniform. An array containing a tensor with shape (2,
  3, 4) and whose first and last dimensions are uniform would have
  `uniform_shape` (2, null, 4). This allows for interpreting the tensor
  correctly without accounting for uniform dimensions while still
  permitting optional optimizations that take advantage of the uniformity.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#variable-shape-tensor>

---

## VariableShapeTensorMetadata

`struct` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata`

```rust
struct VariableShapeTensorMetadata
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn dimension_names(&self) -> Option<&[String]>
fn permutations(&self) -> Option<&[usize]>
fn try_new(dimensions: usize, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>, uniform_shapes: Option<Vec<Option<i32>>>) -> Result<Self, ArrowError>
fn uniform_shapes(&self) -> Option<&[Option<i32>]>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.canonical.variable_shape_tensor.VariableShapeTensorMetadata.md).


Extension type metadata for [`VariableShapeTensor`].

---
