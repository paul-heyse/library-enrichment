# `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.SparseMatrixIndexCSXBuilder.json).

<a id="op-0781ff77e23f401ef2aa9846"></a>
## SparseMatrixIndexCSXBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder` · arrow-ipc 59.3.0

```rust
struct SparseMatrixIndexCSXBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/SparseTensor.rs:671`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd3193e74ecfb023723246bb"></a>
## add_compressedAxis

`function` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder::add_compressedAxis` · arrow-ipc 59.3.0

```rust
fn add_compressedAxis(&mut self, compressedAxis: SparseMatrixCompressedAxis)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder", "path": "SparseMatrixIndexCSXBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [731, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:677`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dc646fd0c2609528e1ba824"></a>
## add_indicesBuffer

`function` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder::add_indicesBuffer` · arrow-ipc 59.3.0

```rust
fn add_indicesBuffer(&mut self, indicesBuffer: &Buffer)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder", "path": "SparseMatrixIndexCSXBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [731, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:704`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-506056994215e4aa630815d9"></a>
## add_indicesType

`function` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder::add_indicesType` · arrow-ipc 59.3.0

```rust
fn add_indicesType(&mut self, indicesType: flatbuffers::WIPOffset<Int<'b>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder", "path": "SparseMatrixIndexCSXBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [731, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:697`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee9dfa1940df112af212ac0e"></a>
## add_indptrBuffer

`function` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder::add_indptrBuffer` · arrow-ipc 59.3.0

```rust
fn add_indptrBuffer(&mut self, indptrBuffer: &Buffer)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder", "path": "SparseMatrixIndexCSXBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [731, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:692`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46f20aeda4d318b3dcc4667e"></a>
## add_indptrType

`function` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder::add_indptrType` · arrow-ipc 59.3.0

```rust
fn add_indptrType(&mut self, indptrType: flatbuffers::WIPOffset<Int<'b>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder", "path": "SparseMatrixIndexCSXBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [731, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:685`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f117bebac76a41bc1fb3610f"></a>
## finish

`function` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<SparseMatrixIndexCSX<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder", "path": "SparseMatrixIndexCSXBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [731, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:719`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2d15da2a10a03cc0fe66d93"></a>
## new

`function` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseMatrixIndexCSXBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder", "path": "SparseMatrixIndexCSXBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [731, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:709`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
