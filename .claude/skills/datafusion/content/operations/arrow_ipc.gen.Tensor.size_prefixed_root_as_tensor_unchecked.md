# `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Tensor.size_prefixed_root_as_tensor_unchecked.json).

<a id="op-a434c4169460ad54e75dbdb9"></a>
## size_prefixed_root_as_tensor_unchecked

`function` · `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn size_prefixed_root_as_tensor_unchecked(buf: &[u8]) -> Tensor<'_>
```

Source: `src/gen/Tensor.rs:1191`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a size prefixed Tensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Tensor`.
