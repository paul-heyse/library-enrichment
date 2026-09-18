# `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.root_as_sparse_tensor_unchecked.json).

<a id="op-576fdf27c4f06d738e6fb449"></a>
## root_as_sparse_tensor_unchecked

`function` · `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn root_as_sparse_tensor_unchecked(buf: &[u8]) -> SparseTensor<'_>
```

Source: `src/gen/SparseTensor.rs:2269`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a SparseTensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `SparseTensor`.
