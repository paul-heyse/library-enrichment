# `arrow_ipc::gen::SparseTensor::size_prefixed_root_as_sparse_tensor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.size_prefixed_root_as_sparse_tensor.json).

<a id="op-31dfc8ae8cd899590c7c82e5"></a>
## size_prefixed_root_as_sparse_tensor

`function` · `arrow_ipc::gen::SparseTensor::size_prefixed_root_as_sparse_tensor` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_sparse_tensor(buf: &[u8]) -> Result<SparseTensor<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/SparseTensor.rs:2234`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a size prefixed
`SparseTensor` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_sparse_tensor_unchecked`.
