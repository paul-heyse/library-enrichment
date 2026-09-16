# `buoyant_kernel::metrics::metered_storage`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.metrics.metered_storage.json`](../model/buoyant_kernel.metrics.metered_storage.json)

## MeteredStorageHandler

`struct` · `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler`

Also reachable as `buoyant_kernel::metrics::MeteredStorageHandler`, `delta_kernel::metrics::metered_storage::MeteredStorageHandler`

```rust
struct MeteredStorageHandler
```

**Implements**: `buoyant_kernel::StorageHandler`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn StorageHandler>) -> Self
```

**via `buoyant_kernel::StorageHandler`**

```rust
fn copy_atomic(&self, src: &Url, dest: &Url) -> DeltaResult<()>
fn delete(&self, path: &Url) -> DeltaResult<()>
fn head(&self, path: &Url) -> DeltaResult<FileMeta>
fn list_from(&self, path: &Url) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<FileMeta>>>>
fn put(&self, path: &Url, data: Bytes, overwrite: bool) -> DeltaResult<()>
fn read_files(&self, files: Vec<FileSlice>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<Bytes>>>>
```

Decorator over an engine-provided `Arc<dyn StorageHandler>` that emits the kernel's
standard `"storage"` spans on operations that produce metrics. `put`, `head`, and `delete`
are pass-through and emit nothing.

---
