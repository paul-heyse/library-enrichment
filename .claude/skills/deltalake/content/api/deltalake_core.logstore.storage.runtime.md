# `deltalake_core::logstore::storage::runtime`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.logstore.storage.runtime.json`](../model/deltalake_core.logstore.storage.runtime.json)

## IORuntime

`enum` · `deltalake_core::logstore::storage::runtime::IORuntime`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.runtime.IORuntime.md)

Also reachable as `deltalake::logstore::IORuntime`, `deltalake_core::logstore::IORuntime`

```rust
enum IORuntime
```

**Variants**: `RT`, `Config`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn get_handle(&self) -> Handle
```

Provide custom Tokio RT or a runtime config

---

## DeltaIOStorageBackend

`struct` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.runtime.DeltaIOStorageBackend.md)

Also reachable as `deltalake::logstore::DeltaIOStorageBackend`, `deltalake_core::logstore::DeltaIOStorageBackend`

```rust
struct DeltaIOStorageBackend<T: ObjectStore + Clone>
```

**Fields**: `inner`

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn new(store: T, rt: IORuntime) -> Self
fn spawn_io_rt<F, O>(&self, f: F, store: &T, path: Path) -> BoxFuture<'_, ObjectStoreResult<O>> where F: for<'a> FnOnce(&'a T, &'a Path) -> BoxFuture<'a, ObjectStoreResult<O>> + Send + 'static, O: Send + 'static
fn spawn_io_rt_from_to<F, O>(&self, f: F, store: &T, from: Path, to: Path) -> BoxFuture<'_, ObjectStoreResult<O>> where F: for<'a> FnOnce(&'a T, &'a Path, &'a Path) -> BoxFuture<'a, ObjectStoreResult<O>> + Send + 'static, O: Send + 'static
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

**via `object_store::ObjectStore`**

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> ObjectStoreResult<()>
fn delete_stream(&self, locations: BoxStream<'static, ObjectStoreResult<Path>>) -> BoxStream<'static, ObjectStoreResult<Path>>
async fn get_opts(&self, location: &Path, options: GetOptions) -> ObjectStoreResult<GetResult>
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> ObjectStoreResult<Vec<Bytes>>
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> ObjectStoreResult<ListResult>
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
async fn put_multipart_opts(&self, location: &Path, options: PutMultipartOptions) -> ObjectStoreResult<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, bytes: PutPayload, options: PutOptions) -> ObjectStoreResult<PutResult>
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> ObjectStoreResult<()>
```

Wraps any object store and runs IO in it's own runtime [EXPERIMENTAL]

---

## RuntimeConfig

`struct` · `deltalake_core::logstore::storage::runtime::RuntimeConfig`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.storage.runtime.RuntimeConfig.md)

```rust
struct RuntimeConfig
```

**Implements**: `core::iter::traits::collect::FromIterator`, `deltalake_core::logstore::config::TryUpdateKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

**via `deltalake_core::logstore::config::TryUpdateKey`**

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Configuration for Tokio runtime

---
