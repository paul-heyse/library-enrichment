# `arrow_ipc::gen::Tensor::TensorArgs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Tensor.TensorArgs.json).

<a id="op-b3dffc53476e8c84ef2f8deb"></a>
## TensorArgs

`struct` · `arrow_ipc::gen::Tensor::TensorArgs` · arrow-ipc 59.3.0

```rust
struct TensorArgs<'a>
```

Source: `src/gen/Tensor.rs:782`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b70509517ed0d1618cf54e48"></a>
## data

`struct_field` · `arrow_ipc::gen::Tensor::TensorArgs::data` · arrow-ipc 59.3.0

```rust
data: Option<&'a Buffer>
```

Source: `src/gen/Tensor.rs:791`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c40660fa8ce90050cc60350"></a>
## default

`function` · `arrow_ipc::gen::Tensor::TensorArgs::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Tensor::TensorArgs", "path": "TensorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [804, 2], "filename": "src/gen/Tensor.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Tensor.rs:795`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f271d62e45e1a27740aadbf"></a>
## shape

`struct_field` · `arrow_ipc::gen::Tensor::TensorArgs::shape` · arrow-ipc 59.3.0

```rust
shape: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim<'a>>>>>
```

Source: `src/gen/Tensor.rs:785`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05196f9de08981d330c8f6a0"></a>
## strides

`struct_field` · `arrow_ipc::gen::Tensor::TensorArgs::strides` · arrow-ipc 59.3.0

```rust
strides: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i64>>>
```

Source: `src/gen/Tensor.rs:790`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71a1b8e7b5669e40e33072cb"></a>
## type_

`struct_field` · `arrow_ipc::gen::Tensor::TensorArgs::type_` · arrow-ipc 59.3.0

```rust
type_: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>
```

Source: `src/gen/Tensor.rs:784`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e64941bd716dc392464b9546"></a>
## type_type

`struct_field` · `arrow_ipc::gen::Tensor::TensorArgs::type_type` · arrow-ipc 59.3.0

```rust
type_type: Type
```

Source: `src/gen/Tensor.rs:783`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
