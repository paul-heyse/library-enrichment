# `arrow_ipc::gen::SparseTensor::SparseTensorArgs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.SparseTensorArgs.json).

<a id="op-b8213c5cb746c5d8ade817a7"></a>
## SparseTensorArgs

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs` · arrow-ipc 59.3.0

```rust
struct SparseTensorArgs<'a>
```

Source: `src/gen/SparseTensor.rs:1801`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62b82aab1d648cf4eef4a1ed"></a>
## data

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::data` · arrow-ipc 59.3.0

```rust
data: Option<&'a Buffer>
```

Source: `src/gen/SparseTensor.rs:1812`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-633719e1b2a08f866af25ba8"></a>
## default

`function` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::SparseTensor::SparseTensorArgs", "path": "SparseTensorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1814, 1], "end": [1827, 2], "filename": "src/gen/SparseTensor.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/SparseTensor.rs:1816`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddb7c6df7a26f95dc8cd7d66"></a>
## non_zero_length

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::non_zero_length` · arrow-ipc 59.3.0

```rust
non_zero_length: i64
```

Source: `src/gen/SparseTensor.rs:1809`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ed15896139c89857cee2801"></a>
## shape

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::shape` · arrow-ipc 59.3.0

```rust
shape: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim<'a>>>>>
```

Source: `src/gen/SparseTensor.rs:1804`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f6b89c1068114e0defdb13d"></a>
## sparseIndex

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::sparseIndex` · arrow-ipc 59.3.0

```rust
sparseIndex: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>
```

Source: `src/gen/SparseTensor.rs:1811`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afbe6c1a6289a2e75cd2c6d7"></a>
## sparseIndex_type

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::sparseIndex_type` · arrow-ipc 59.3.0

```rust
sparseIndex_type: SparseTensorIndex
```

Source: `src/gen/SparseTensor.rs:1810`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b26839f322a0ebf7f9075618"></a>
## type_

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::type_` · arrow-ipc 59.3.0

```rust
type_: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>
```

Source: `src/gen/SparseTensor.rs:1803`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fc4fb033f2a74c4726e99ab"></a>
## type_type

`struct_field` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs::type_type` · arrow-ipc 59.3.0

```rust
type_type: Type
```

Source: `src/gen/SparseTensor.rs:1802`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
