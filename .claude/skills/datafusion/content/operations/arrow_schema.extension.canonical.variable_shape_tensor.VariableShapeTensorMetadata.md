# `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.variable_shape_tensor.VariableShapeTensorMetadata.json).

<a id="op-b277b44cc7bc76a53e927347"></a>
## VariableShapeTensorMetadata

`struct` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata` · arrow-schema 59.3.0

```rust
struct VariableShapeTensorMetadata
```

Source: `src/extension/canonical/variable_shape_tensor.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Extension type metadata for [`VariableShapeTensor`](../operations/arrow_schema.extension.canonical.variable_shape_tensor.VariableShapeTensor.md#op-1cbb0485c49d5171a60c3365).

<a id="op-ca9ffc1ff1c87679d9b5a8da"></a>
## clone

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> VariableShapeTensorMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 17], "end": [145, 22], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db11ad4639120e9ea45273d9"></a>
## deserialize

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [292, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76743b6c30437c86798ceb97"></a>
## dimension_names

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::dimension_names` · arrow-schema 59.3.0

```rust
fn dimension_names(&self) -> Option<&[String]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [376, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:360`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the names of the dimensions in this variable shape tensor, if
set.

<a id="op-3a2a7637a7a927d5498ccee9"></a>
## eq

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &VariableShapeTensorMetadata) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 24], "end": [145, 33], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20731de475090995b32a73ea"></a>
## fmt

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 10], "end": [145, 15], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0266791d73c69ef85edfaa66"></a>
## permutations

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::permutations` · arrow-schema 59.3.0

```rust
fn permutations(&self) -> Option<&[usize]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [376, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:366`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the indices of the desired ordering of the original dimensions,
if set.

<a id="op-002c65955c56209c57d4a92b"></a>
## serialize

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::serialize` · arrow-schema 59.3.0

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [170, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/extension/canonical/variable_shape_tensor.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64a9abe154a4dd67e930aa03"></a>
## try_new

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::try_new` · arrow-schema 59.3.0

```rust
fn try_new(dimensions: usize, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>, uniform_shapes: Option<Vec<Option<i32>>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [376, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns metadata for a variable shape tensor extension type.

# Error

Return an error if the provided dimension names, permutations or
uniform shapes are invalid.

<a id="op-18ec6cb3c68f063e52a83ad2"></a>
## uniform_shapes

`function` · `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata::uniform_shapes` · arrow-schema 59.3.0

```rust
fn uniform_shapes(&self) -> Option<&[Option<i32>]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensorMetadata", "path": "VariableShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [376, 2], "filename": "src/extension/canonical/variable_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/variable_shape_tensor.rs:373`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns sizes of individual tensor’s dimensions which are guaranteed
to stay constant in uniform dimensions and can vary in non-uniform
dimensions.
