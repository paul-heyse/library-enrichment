# StorageHandler

`buoyant_kernel::StorageHandler`

```rust
trait StorageHandler: AsAny
```

Also reachable as `delta_kernel::StorageHandler`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#storagehandler) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn copy_atomic(&self, src: &Url, dest: &Url) -> DeltaResult<()>
fn delete(&self, path: &Url) -> DeltaResult<()>
fn head(&self, path: &Url) -> DeltaResult<FileMeta>
fn list_from(&self, path: &Url) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<FileMeta>>>>
fn put(&self, path: &Url, data: Bytes, overwrite: bool) -> DeltaResult<()>
fn read_files(&self, files: Vec<FileSlice>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<Bytes>>>>
```

## Implementors (3)

Read one before writing your own.

- `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler`
- `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler`
- `deltalake_core::delta_datafusion::engine::storage::DataFusionStorageHandler`

## Documentation

Provides file system related functionalities to Delta Kernel.

Delta Kernel uses this handler whenever it needs to access the underlying
file system where the Delta table is present. Connector implementation of
this trait can hide filesystem specific details from Delta Kernel.
