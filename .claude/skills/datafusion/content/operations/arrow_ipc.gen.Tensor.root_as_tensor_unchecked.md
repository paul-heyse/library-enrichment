# `arrow_ipc::gen::Tensor::root_as_tensor_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Tensor.root_as_tensor_unchecked.json).

<a id="op-c4966fe23bdf0c5db47572d4"></a>
## root_as_tensor_unchecked

`function` · `arrow_ipc::gen::Tensor::root_as_tensor_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn root_as_tensor_unchecked(buf: &[u8]) -> Tensor<'_>
```

Source: `src/gen/Tensor.rs:1184`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a Tensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `Tensor`.
