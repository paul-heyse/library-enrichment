# `arrow_ipc::gen::SparseTensor::size_prefixed_root_as_sparse_tensor_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.size_prefixed_root_as_sparse_tensor_unchecked.json).

<a id="op-920a49bedc6db64cbaf775ee"></a>
## size_prefixed_root_as_sparse_tensor_unchecked

`function` · `arrow_ipc::gen::SparseTensor::size_prefixed_root_as_sparse_tensor_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn size_prefixed_root_as_sparse_tensor_unchecked(buf: &[u8]) -> SparseTensor<'_>
```

Source: `src/gen/SparseTensor.rs:2276`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a size prefixed SparseTensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `SparseTensor`.
