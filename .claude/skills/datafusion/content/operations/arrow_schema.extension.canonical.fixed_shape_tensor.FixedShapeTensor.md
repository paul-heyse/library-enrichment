# `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.fixed_shape_tensor.FixedShapeTensor.json).

<a id="op-49f2fa285749532be29d6feb"></a>
## FixedShapeTensor

`struct` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor` · arrow-schema 59.3.0

```rust
struct FixedShapeTensor
```

Source: `src/extension/canonical/fixed_shape_tensor.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

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

<a id="op-32bc6c3e622171243e3a8d3b"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [433, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:363`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3342c0a5589eb9b4509cd31"></a>
## NAME

`assoc_const` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [433, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:361`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efcd257571a752a232b5fe90"></a>
## clone

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> FixedShapeTensor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 17], "end": [75, 22], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5754eaaa5ad1b9517c62387e"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [433, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:373`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5601fb931bb6a771d45be9f"></a>
## dimension_names

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::dimension_names` · arrow-schema 59.3.0

```rust
fn dimension_names(&self) -> Option<&[String]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [132, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the names of the dimensions in this fixed shape tensor, if
set.

<a id="op-8a50bd313c8896e49cf49157"></a>
## dimensions

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::dimensions` · arrow-schema 59.3.0

```rust
fn dimensions(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [132, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the number of dimensions in this fixed shape tensor.

<a id="op-234ce021c998c753635ae8d9"></a>
## eq

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &FixedShapeTensor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 24], "end": [75, 33], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61cc395bc07a1e39d0ecd217"></a>
## fmt

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 10], "end": [75, 15], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c7d2b0b713aaaefb1116657"></a>
## list_size

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::list_size` · arrow-schema 59.3.0

```rust
fn list_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [132, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the product of all the elements in tensor shape.

<a id="op-660b1f5070ad5e66bd468774"></a>
## metadata

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [433, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0458b8a12a3226788790a7d8"></a>
## permutations

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::permutations` · arrow-schema 59.3.0

```rust
fn permutations(&self) -> Option<&[usize]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [132, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the indices of the desired ordering of the original
dimensions, if set.

<a id="op-0ab0d6f22c884e060ace3faf"></a>
## serialize_metadata

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [433, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13a1a5d0776e0db4b5167185"></a>
## supports_data_type

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [433, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0830456f26a41c114e457358"></a>
## try_new

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::try_new` · arrow-schema 59.3.0

```rust
fn try_new(value_type: DataType, shape: impl IntoIterator<Item = usize>, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [132, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a new fixed shape tensor extension type.

# Error

Return an error if the provided dimension names or permutations are
invalid.

<a id="op-4e24781c12f44a363c84a339"></a>
## try_new

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::try_new` · arrow-schema 59.3.0

```rust
fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [360, 1], "end": [433, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:406`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02c2a9edc7ad55f33b8829fe"></a>
## value_type

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor::value_type` · arrow-schema 59.3.0

```rust
fn value_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [132, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the value type of the individual tensor elements.
