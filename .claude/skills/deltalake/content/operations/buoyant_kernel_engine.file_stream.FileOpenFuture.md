# `buoyant_kernel_engine::file_stream::FileOpenFuture`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.file_stream.FileOpenFuture.json).

<a id="op-12b747d929c6b0f172c6b87e"></a>
## FileOpenFuture

`type_alias` · `buoyant_kernel_engine::file_stream::FileOpenFuture` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type FileOpenFuture = futures::future::BoxFuture<'static, delta_kernel::DeltaResult<futures::stream::BoxStream<'static, delta_kernel::DeltaResult<delta_kernel::arrow::array::RecordBatch>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L16).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A fallible future that resolves to a stream of [`RecordBatch`]
cbindgen:ignore

Unresolved upstream links (retained, not inferred): ``RecordBatch``.
