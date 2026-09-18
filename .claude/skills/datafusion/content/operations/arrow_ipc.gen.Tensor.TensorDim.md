# `arrow_ipc::gen::Tensor::TensorDim`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Tensor.TensorDim.json).

<a id="op-8bbc726eaaa71465190bd6da"></a>
## TensorDim

`struct` · `arrow_ipc::gen::Tensor::TensorDim` · arrow-ipc 59.3.0

```rust
struct TensorDim<'a>
```

Source: `src/gen/Tensor.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

----------------------------------------------------------------------
Data structures for dense tensors
Shape data for a single axis in a tensor

<a id="op-2689dc3874856c9f7fdf96f3"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Tensor::TensorDim::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [46, 2], "filename": "src/gen/Tensor.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Tensor.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-880db6024e4749c43fdb9f65"></a>
## VT_NAME

`assoc_const` · `arrow_ipc::gen::Tensor::TensorDim::VT_NAME` · arrow-ipc 59.3.0

```rust
VT_NAME
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [88, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3132b9c2b52d8e3538573b7f"></a>
## VT_SIZE

`assoc_const` · `arrow_ipc::gen::Tensor::TensorDim::VT_SIZE` · arrow-ipc 59.3.0

```rust
VT_SIZE
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [88, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22674d4d933d9fc5505a6761"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Tensor::TensorDim::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Tensor.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-622bb68bac659f74d75b0d55"></a>
## clone

`function` · `arrow_ipc::gen::Tensor::TensorDim::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> TensorDim<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 16], "end": [29, 21], "filename": "src/gen/Tensor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Tensor.rs:29`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-314fc28882dd9eb2e5d98b59"></a>
## create

`function` · `arrow_ipc::gen::Tensor::TensorDim::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args TensorDimArgs<'args>) -> flatbuffers::WIPOffset<TensorDim<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [88, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c85d2b25fa4ae35c70e18ee6"></a>
## eq

`function` · `arrow_ipc::gen::Tensor::TensorDim::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &TensorDim<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 23], "end": [29, 32], "filename": "src/gen/Tensor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Tensor.rs:29`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2f4157dbfc477bbe19da137"></a>
## fmt

`function` · `arrow_ipc::gen::Tensor::TensorDim::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [154, 2], "filename": "src/gen/Tensor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Tensor.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9ea65d5109ac3297f4a0494"></a>
## follow

`function` · `arrow_ipc::gen::Tensor::TensorDim::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [46, 2], "filename": "src/gen/Tensor.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Tensor.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b58da2dcfec4b13bb205be7"></a>
## init_from_table

`function` · `arrow_ipc::gen::Tensor::TensorDim::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [88, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14f42788baf0a7951f78670f"></a>
## name

`function` · `arrow_ipc::gen::Tensor::TensorDim::name` · arrow-ipc 59.3.0

```rust
fn name(&self) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [88, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Name of the dimension, optional

<a id="op-47a0c169511e5d90ea6a469a"></a>
## run_verifier

`function` · `arrow_ipc::gen::Tensor::TensorDim::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [103, 2], "filename": "src/gen/Tensor.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Tensor.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5460c717fd45986e7cc4c48"></a>
## size

`function` · `arrow_ipc::gen::Tensor::TensorDim::size` · arrow-ipc 59.3.0

```rust
fn size(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorDim", "path": "TensorDim"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [88, 2], "filename": "src/gen/Tensor.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Tensor.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Length of dimension
