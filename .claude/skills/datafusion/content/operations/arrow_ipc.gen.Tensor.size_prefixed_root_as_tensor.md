# `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Tensor.size_prefixed_root_as_tensor.json).

<a id="op-1d1dfe17f7b5e1bb8dff3a12"></a>
## size_prefixed_root_as_tensor

`function` · `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_tensor(buf: &[u8]) -> Result<Tensor<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Tensor.rs:1151`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a size prefixed
`Tensor` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_tensor_unchecked`.
