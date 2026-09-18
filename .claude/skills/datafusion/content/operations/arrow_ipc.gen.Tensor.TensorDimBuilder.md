# `arrow_ipc::gen::Tensor::TensorDimBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Tensor.TensorDimBuilder.json).

<a id="op-50a893e66cfa14af6a46801c"></a>
## TensorDimBuilder

`struct` · `arrow_ipc::gen::Tensor::TensorDimBuilder` · arrow-ipc 59.3.0

```rust
struct TensorDimBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/Tensor.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdd21e0d055b41650b3c659f"></a>
## add_name

`function` · `arrow_ipc::gen::Tensor::TensorDimBuilder::add_name` · arrow-ipc 59.3.0

```rust
fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDimBuilder", "path": "TensorDimBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [145, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da365976a13ee127ce63ed32"></a>
## add_size

`function` · `arrow_ipc::gen::Tensor::TensorDimBuilder::add_size` · arrow-ipc 59.3.0

```rust
fn add_size(&mut self, size: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDimBuilder", "path": "TensorDimBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [145, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:124`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f1a4b1ec010e3d35bab0e51"></a>
## finish

`function` · `arrow_ipc::gen::Tensor::TensorDimBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<TensorDim<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDimBuilder", "path": "TensorDimBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [145, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89bd0717b1d588bbf0417260"></a>
## new

`function` · `arrow_ipc::gen::Tensor::TensorDimBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> TensorDimBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDimBuilder", "path": "TensorDimBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [145, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:133`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
