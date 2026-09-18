# `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.SparseTensorIndexCSFBuilder.json).

<a id="op-10a63b2f1bb05ec79ea35d10"></a>
## SparseTensorIndexCSFBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder` · arrow-ipc 59.3.0

```rust
struct SparseTensorIndexCSFBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/SparseTensor.rs:986`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ba4de970d6d3d65f47b163"></a>
## add_axisOrder

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder::add_axisOrder` · arrow-ipc 59.3.0

```rust
fn add_axisOrder(&mut self, axisOrder: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i32>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder", "path": "SparseTensorIndexCSFBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1060, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1026`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af6fee4986dd415017535ee2"></a>
## add_indicesBuffers

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder::add_indicesBuffers` · arrow-ipc 59.3.0

```rust
fn add_indicesBuffers(&mut self, indicesBuffers: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Buffer>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder", "path": "SparseTensorIndexCSFBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1060, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1016`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb48aac696b793d20b36b913"></a>
## add_indicesType

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder::add_indicesType` · arrow-ipc 59.3.0

```rust
fn add_indicesType(&mut self, indicesType: flatbuffers::WIPOffset<Int<'b>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder", "path": "SparseTensorIndexCSFBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1060, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1009`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0b22ea8500597da348edf73"></a>
## add_indptrBuffers

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder::add_indptrBuffers` · arrow-ipc 59.3.0

```rust
fn add_indptrBuffers(&mut self, indptrBuffers: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Buffer>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder", "path": "SparseTensorIndexCSFBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1060, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:999`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30aafb6f2cdee4bba4d45a20"></a>
## add_indptrType

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder::add_indptrType` · arrow-ipc 59.3.0

```rust
fn add_indptrType(&mut self, indptrType: flatbuffers::WIPOffset<Int<'b>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder", "path": "SparseTensorIndexCSFBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1060, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:992`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f36a66b30badba3c78824b7"></a>
## finish

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<SparseTensorIndexCSF<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder", "path": "SparseTensorIndexCSFBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1060, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1046`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f8c4e14e66398a7b7460720"></a>
## new

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseTensorIndexCSFBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder", "path": "SparseTensorIndexCSFBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1060, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1036`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
