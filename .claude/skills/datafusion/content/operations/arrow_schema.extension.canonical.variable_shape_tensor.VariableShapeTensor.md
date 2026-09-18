# `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.variable_shape_tensor.VariableShapeTensor.json).

<a id="op-1cbb0485c49d5171a60c3365"></a>
## VariableShapeTensor

`struct` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor` · arrow-schema 59.3.0

```rust
struct VariableShapeTensor
```

Source: `src/extension/canonical/variable_shape_tensor.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

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

<a id="op-8f95f8994b3c59fe40148c9a"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [479, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:381`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82b24826bbb14239a7e4fec0"></a>
## NAME

`assoc_const` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [479, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:379`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b26f4734a3043d618add46d9"></a>
## clone

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> VariableShapeTensor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 17], "end": [74, 22], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e03f9cfe32d3d940d21c4434"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [479, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:391`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-341da4d55920e4785c3d836f"></a>
## dimension_names

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::dimension_names` · arrow-schema 59.3.0

```rust
fn dimension_names(&self) -> Option<&[String]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [142, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:126`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the names of the dimensions in this variable shape tensor, if
set.

<a id="op-2fc34339654c77459f086f9f"></a>
## dimensions

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::dimensions` · arrow-schema 59.3.0

```rust
fn dimensions(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [142, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the number of dimensions  in this variable shape tensor.

<a id="op-87889e26ed59ecbcae38a9e6"></a>
## eq

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &VariableShapeTensor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 24], "end": [74, 33], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38ca6eefe9d173d363360e3b"></a>
## fmt

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 10], "end": [74, 15], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da0aa1de9d6d9faceaa36396"></a>
## metadata

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [479, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:383`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3131d60ce7504661e0a82c8"></a>
## permutations

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::permutations` · arrow-schema 59.3.0

```rust
fn permutations(&self) -> Option<&[usize]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [142, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:132`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the indices of the desired ordering of the original
dimensions, if set.

<a id="op-c3df87680b91426ddc951c76"></a>
## serialize_metadata

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [479, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49acd81a4191c815d548a177"></a>
## supports_data_type

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [479, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:408`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d50fcad344effe6d8702655"></a>
## try_new

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::try_new` · arrow-schema 59.3.0

```rust
fn try_new(value_type: DataType, dimensions: usize, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>, uniform_shapes: Option<Vec<Option<i32>>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [142, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a new variable shape tensor extension type.

# Error

Return an error if the provided dimension names, permutations or
uniform shapes are invalid.

<a id="op-9103ba63c3aaa7048317d7d4"></a>
## try_new

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::try_new` · arrow-schema 59.3.0

```rust
fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [479, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:439`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec521269fd99c3f26378abf6"></a>
## uniform_shapes

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::uniform_shapes` · arrow-schema 59.3.0

```rust
fn uniform_shapes(&self) -> Option<&[Option<i32>]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [142, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns sizes of individual tensor’s dimensions which are guaranteed
to stay constant in uniform dimensions and can vary in non-uniform
dimensions.

<a id="op-59943748aa58184bb485c3c4"></a>
## value_type

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor::value_type` · arrow-schema 59.3.0

```rust
fn value_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [142, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the value type of the individual tensor elements.
