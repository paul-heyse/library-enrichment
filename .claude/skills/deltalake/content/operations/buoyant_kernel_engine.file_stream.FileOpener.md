# `buoyant_kernel_engine::file_stream::FileOpener`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.file_stream.FileOpener.json).

<a id="op-b0fa26a30ee2f87e0744bff4"></a>
## FileOpener

`trait` · `buoyant_kernel_engine::file_stream::FileOpener` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait FileOpener: Send + Unpin
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Generic API for opening a file using an [`ObjectStore`] and resolving to a
stream of [`RecordBatch`]

[`ObjectStore`]: delta_kernel::object_store::ObjectStore

Unresolved upstream links (retained, not inferred): `delta_kernel::object_store::ObjectStore`, ``RecordBatch``.

<a id="op-fae16ea8e7ee8bc06ed0069c"></a>
## open

`function` · `buoyant_kernel_engine::file_stream::FileOpener::open` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn open(&self, file_meta: FileMeta, range: Option<Range<i64>>) -> DeltaResult<FileOpenFuture>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Asynchronously open the specified file and return a stream
of [`RecordBatch`]

Unresolved upstream links (retained, not inferred): ``RecordBatch``.
