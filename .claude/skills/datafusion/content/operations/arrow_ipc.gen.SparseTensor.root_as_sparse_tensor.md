# `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.root_as_sparse_tensor.json).

<a id="op-1f1ce74b668acbd924b16620"></a>
## root_as_sparse_tensor

`function` · `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor` · arrow-ipc 59.3.0

```rust
fn root_as_sparse_tensor(buf: &[u8]) -> Result<SparseTensor<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/SparseTensor.rs:2224`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a `SparseTensor`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_sparse_tensor_unchecked`.
