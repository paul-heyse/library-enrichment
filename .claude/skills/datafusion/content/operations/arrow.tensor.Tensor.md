# `arrow::tensor::Tensor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.tensor.Tensor.json).

<a id="op-e7dc421934ee956ed8a99ef1"></a>
## Tensor

`struct` · `arrow::tensor::Tensor` · arrow 59.3.0

```rust
struct Tensor<'a, T: ArrowPrimitiveType>
```

Source: `src/tensor.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Tensor of primitive types

<a id="op-b5249439e494616785cee4c3"></a>
## data

`function` · `arrow::tensor::Tensor::data` · arrow 59.3.0

```rust
fn data(&self) -> &Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:284`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Returns a reference to the underlying `Buffer`

<a id="op-c931d020f1b0d921898859f7"></a>
## data_type

`function` · `arrow::tensor::Tensor::data_type` · arrow 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:274`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The data type of the `Tensor`

<a id="op-87ff11ac2649bdab17b5975d"></a>
## dim_name

`function` · `arrow::tensor::Tensor::dim_name` · arrow 59.3.0

```rust
fn dim_name(&self, i: usize) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:307`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The name of dimension i

<a id="op-9502af3de17a8a3bc44b2f7d"></a>
## fmt

`function` · `arrow::tensor::Tensor::fmt` · arrow 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 15], "filename": "src/tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tensor.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7374fe34beacbb53c554269"></a>
## is_column_major

`function` · `arrow::tensor::Tensor::is_column_major` · arrow 59.3.0

```rust
fn is_column_major(&self) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:333`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Indicates if the memory layout column major

<a id="op-2d58adb8abd7eafe3d0bca05"></a>
## is_contiguous

`function` · `arrow::tensor::Tensor::is_contiguous` · arrow 59.3.0

```rust
fn is_contiguous(&self) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Indicates if the data is laid out contiguously in memory

<a id="op-0a12d95ffe440b3852cf1c32"></a>
## is_row_major

`function` · `arrow::tensor::Tensor::is_row_major` · arrow 59.3.0

```rust
fn is_row_major(&self) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:325`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Indicates if the memory layout row major

<a id="op-fa0e9f9086076c8b681494b0"></a>
## names

`function` · `arrow::tensor::Tensor::names` · arrow 59.3.0

```rust
fn names(&self) -> Option<&Vec<&'a str>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:294`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The names of the dimensions

<a id="op-a6a955bd8f095aa3d153a496"></a>
## ndim

`function` · `arrow::tensor::Tensor::ndim` · arrow 59.3.0

```rust
fn ndim(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The number of dimensions

<a id="op-4bcb4a92bfb4dce3ce2f9034"></a>
## new_column_major

`function` · `arrow::tensor::Tensor::new_column_major` · arrow 59.3.0

```rust
fn new_column_major(buffer: Buffer, shape: Option<Vec<usize>>, names: Option<Vec<&'a str>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:257`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a new Tensor using column major memory layout

<a id="op-a6ccc15f234aa0166b068eb8"></a>
## new_row_major

`function` · `arrow::tensor::Tensor::new_row_major` · arrow 59.3.0

```rust
fn new_row_major(buffer: Buffer, shape: Option<Vec<usize>>, names: Option<Vec<&'a str>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:240`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a new Tensor using row major memory layout

<a id="op-1409d2b8d3fd2c48098773a6"></a>
## shape

`function` · `arrow::tensor::Tensor::shape` · arrow 59.3.0

```rust
fn shape(&self) -> Option<&Vec<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:279`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The sizes of the dimensions

<a id="op-ea0d5f61e094fcfcba0c2ca7"></a>
## size

`function` · `arrow::tensor::Tensor::size` · arrow 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:312`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The total number of elements in the `Tensor`

<a id="op-71bb52800b2e57cac8396492"></a>
## strides

`function` · `arrow::tensor::Tensor::strides` · arrow 59.3.0

```rust
fn strides(&self) -> Option<&Vec<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

The number of bytes between elements in each dimension

<a id="op-eb892afada06fe6b7fb80668"></a>
## try_new

`function` · `arrow::tensor::Tensor::try_new` · arrow 59.3.0

```rust
fn try_new(buffer: Buffer, shape: Option<Vec<usize>>, strides: Option<Vec<usize>>, names: Option<Vec<&'a str>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow::tensor::Tensor", "path": "Tensor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [339, 2], "filename": "src/tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/tensor.rs:152`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Creates a new `Tensor`
