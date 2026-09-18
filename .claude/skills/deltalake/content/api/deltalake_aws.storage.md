# `deltalake_aws::storage`

Crate `deltalake-aws` · 4 public items · structured records in [`model/deltalake_aws.storage.json`](../model/deltalake_aws.storage.json)

## S3ObjectStoreFactory

`struct` · `deltalake_aws::storage::S3ObjectStoreFactory`
[Full member contracts, output types and access classification](../operations/deltalake_aws.storage.S3ObjectStoreFactory.md)

```rust
struct S3ObjectStoreFactory
```

**Implements**: `deltalake_aws::storage::S3StorageOptionsConversion`, `deltalake_core::logstore::factories::ObjectStoreFactory`

**Derives**: Clone, Debug, Default

**via `deltalake_core::logstore::factories::ObjectStoreFactory`**

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

---

## S3StorageBackend

`struct` · `deltalake_aws::storage::S3StorageBackend`
[Full member contracts, output types and access classification](../operations/deltalake_aws.storage.S3StorageBackend.md)

```rust
struct S3StorageBackend
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(storage: ObjectStoreRef, allow_unsafe_rename: bool) -> ObjectStoreResult<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
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

An S3 implementation of the [ObjectStore] trait

---

## S3StorageOptions

`struct` · `deltalake_aws::storage::S3StorageOptions`
[Full member contracts, output types and access classification](../operations/deltalake_aws.storage.S3StorageOptions.md)

```rust
struct S3StorageOptions
```

**Fields**: `locking_provider`, `allow_unsafe_rename`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn builder() -> S3StorageOptionsBuilder<((), ())>
fn from_map(options: &HashMap<String, String>) -> DeltaResult<S3StorageOptions>
fn try_default() -> DeltaResult<Self>
```

Options used to configure the [S3StorageBackend].

Available options are described in [constants].

---

## S3StorageOptionsBuilder

`struct` · `deltalake_aws::storage::S3StorageOptionsBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_aws.storage.S3StorageOptionsBuilder.md)

```rust
struct S3StorageOptionsBuilder<TypedBuilderFields = ((), ())>
```

**Derives**: Clone

**Methods** (3)

```rust
fn allow_unsafe_rename(self, allow_unsafe_rename: bool) -> S3StorageOptionsBuilder<(__locking_provider, (bool,))>
fn build(self) -> S3StorageOptions
fn locking_provider(self, locking_provider: impl ::core::convert::Into<String>) -> S3StorageOptionsBuilder<((Option<String>,), __allow_unsafe_rename)>
```

Builder for [`S3StorageOptions`] instances.

See [`S3StorageOptions::builder()`] for more info.

---
