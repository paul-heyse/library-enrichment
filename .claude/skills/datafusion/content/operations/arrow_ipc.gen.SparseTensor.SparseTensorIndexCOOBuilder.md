# `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.SparseTensorIndexCOOBuilder.json).

<a id="op-d4536ddf222a79a2cdba4754"></a>
## SparseTensorIndexCOOBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder` · arrow-ipc 59.3.0

```rust
struct SparseTensorIndexCOOBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/SparseTensor.rs:410`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0ac6c9e7e80f5005196d483"></a>
## add_indicesBuffer

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder::add_indicesBuffer` · arrow-ipc 59.3.0

```rust
fn add_indicesBuffer(&mut self, indicesBuffer: &Buffer)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder", "path": "SparseTensorIndexCOOBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [461, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:433`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7306c21992558b21c16ac05f"></a>
## add_indicesStrides

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder::add_indicesStrides` · arrow-ipc 59.3.0

```rust
fn add_indicesStrides(&mut self, indicesStrides: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i64>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder", "path": "SparseTensorIndexCOOBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [461, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:423`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6af4c47bd373afba5a85da6"></a>
## add_indicesType

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder::add_indicesType` · arrow-ipc 59.3.0

```rust
fn add_indicesType(&mut self, indicesType: flatbuffers::WIPOffset<Int<'b>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder", "path": "SparseTensorIndexCOOBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [461, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:416`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91c458799ee0f7e4712068dd"></a>
## add_isCanonical

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder::add_isCanonical` · arrow-ipc 59.3.0

```rust
fn add_isCanonical(&mut self, isCanonical: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder", "path": "SparseTensorIndexCOOBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [461, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:438`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d26b871a57b963be5f785afd"></a>
## finish

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<SparseTensorIndexCOO<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder", "path": "SparseTensorIndexCOOBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [461, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:453`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1aa92ce7f59968b12ab829da"></a>
## new

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseTensorIndexCOOBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder", "path": "SparseTensorIndexCOOBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [461, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/SparseTensor.rs:443`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
