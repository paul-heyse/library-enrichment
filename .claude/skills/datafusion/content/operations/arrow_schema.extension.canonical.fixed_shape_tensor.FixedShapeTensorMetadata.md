# `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.fixed_shape_tensor.FixedShapeTensorMetadata.json).

<a id="op-3cabb6122c042504db3176fe"></a>
## FixedShapeTensorMetadata

`struct` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata` · arrow-schema 59.3.0

```rust
struct FixedShapeTensorMetadata
```

Source: `src/extension/canonical/fixed_shape_tensor.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Extension type metadata for [`FixedShapeTensor`](../operations/arrow_schema.extension.canonical.fixed_shape_tensor.FixedShapeTensor.md#op-49f2fa285749532be29d6feb).

<a id="op-fbad4e57a9448d2253b7ae24"></a>
## clone

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> FixedShapeTensorMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 17], "end": [135, 22], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52807e869c0a42a2918c1b5a"></a>
## deserialize

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [282, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f58e34d012e064b1b65b57b"></a>
## dimension_names

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::dimension_names` · arrow-schema 59.3.0

```rust
fn dimension_names(&self) -> Option<&[String]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [358, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the names of the dimensions in this fixed shape tensor, if
set.

<a id="op-4fd5ab62243eb9dcd6211e30"></a>
## dimensions

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::dimensions` · arrow-schema 59.3.0

```rust
fn dimensions(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [358, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the number of dimensions in this fixed shape tensor.

<a id="op-13d871620b51bfefe4db9974"></a>
## eq

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &FixedShapeTensorMetadata) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 24], "end": [135, 33], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b9720a47f2a62e6bfbb393a"></a>
## fmt

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 10], "end": [135, 15], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0383b6a2a383f37cfb339128"></a>
## list_size

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::list_size` · arrow-schema 59.3.0

```rust
fn list_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [358, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:338`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the product of all the elements in tensor shape.

<a id="op-a488f9dda4133d1f48700e8a"></a>
## permutations

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::permutations` · arrow-schema 59.3.0

```rust
fn permutations(&self) -> Option<&[usize]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [358, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:355`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the indices of the desired ordering of the original
dimensions, if set.

<a id="op-98647149fa8cfb009606d1a1"></a>
## serialize

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::serialize` · arrow-schema 59.3.0

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [158, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42e5b86ab98dbeedb7de6777"></a>
## try_new

`function` · `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata::try_new` · arrow-schema 59.3.0

```rust
fn try_new(shape: impl IntoIterator<Item = usize>, dimension_names: Option<Vec<String>>, permutations: Option<Vec<usize>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensorMetadata", "path": "FixedShapeTensorMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [358, 2], "filename": "src/extension/canonical/fixed_shape_tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension/canonical/fixed_shape_tensor.rs:291`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns metadata for a fixed shape tensor extension type.

# Error

Return an error if the provided dimension names or permutations are
invalid.
