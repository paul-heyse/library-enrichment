# `buoyant_kernel_engine::filesystem`

Crate `buoyant_kernel_engine` · 1 public items · structured records in [`model/buoyant_kernel_engine.filesystem.json`](../model/buoyant_kernel_engine.filesystem.json)

## ObjectStoreStorageHandler

`struct` · `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel_engine.filesystem.ObjectStoreStorageHandler.md)

Also reachable as `delta_kernel_default_engine::filesystem::ObjectStoreStorageHandler`

```rust
struct ObjectStoreStorageHandler<E: TaskExecutor>
```

**Implements**: `buoyant_kernel::StorageHandler`

**Derives**: Debug

**Methods** (2)

```rust
fn new(store: Arc<DynObjectStore>, task_executor: Arc<E>) -> Self
fn with_readahead(self, readahead: usize) -> Self
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

---
