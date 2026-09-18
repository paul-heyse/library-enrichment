# `arrow_ipc::gen::SparseTensor::SparseTensorBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.SparseTensorBuilder.json).

<a id="op-52b2a31db418f7e0be296f68"></a>
## SparseTensorBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder` · arrow-ipc 59.3.0

```rust
struct SparseTensorBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/SparseTensor.rs:1829`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38a4a17d942889d2012c14e7"></a>
## add_data

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::add_data` · arrow-ipc 59.3.0

```rust
fn add_data(&mut self, data: &Buffer)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1878`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d84e013aacd5b82bc56e3fb"></a>
## add_non_zero_length

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::add_non_zero_length` · arrow-ipc 59.3.0

```rust
fn add_non_zero_length(&mut self, non_zero_length: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1855`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b503f26c32bb48ed60cbb888"></a>
## add_shape

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::add_shape` · arrow-ipc 59.3.0

```rust
fn add_shape(&mut self, shape: flatbuffers::WIPOffset<flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<TensorDim<'b>>>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1845`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6afe14e1828c02d5f52c9df"></a>
## add_sparseIndex

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::add_sparseIndex` · arrow-ipc 59.3.0

```rust
fn add_sparseIndex(&mut self, sparseIndex: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1868`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-456bc7f1653b7d51c92dbd5a"></a>
## add_sparseIndex_type

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::add_sparseIndex_type` · arrow-ipc 59.3.0

```rust
fn add_sparseIndex_type(&mut self, sparseIndex_type: SparseTensorIndex)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1860`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f25ce3e7f35bd7d72504343"></a>
## add_type_

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::add_type_` · arrow-ipc 59.3.0

```rust
fn add_type_(&mut self, type_: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1840`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40f52717078ae478ac229174"></a>
## add_type_type

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::add_type_type` · arrow-ipc 59.3.0

```rust
fn add_type_type(&mut self, type_type: Type)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1835`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6579863ab436d7e3ec0cf8f7"></a>
## finish

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<SparseTensor<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1893`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cca02e1149fa60d3103076f"></a>
## new

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseTensorBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorBuilder", "path": "SparseTensorBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1833, 1], "end": [1902, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:1883`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
