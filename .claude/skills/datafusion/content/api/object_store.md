# `object_store`

Crate `object_store` · 20 public items · structured records in [`model/object_store.json`](../model/object_store.json)

## CopyMode

`enum` · `object_store::CopyMode`

Also reachable as `datafusion::object_store::CopyMode`

```rust
enum CopyMode
```

**Variants**: `Overwrite`, `Create`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/object_store.CopyMode.md).


Configure preconditions for the copy operation

---

## Error

`enum` · `object_store::Error`

Also reachable as `datafusion::object_store::Error`

```rust
enum Error
```

**Variants**: `Generic`, `NotFound`, `InvalidPath`, `JoinError`, `NotSupported`, `AlreadyExists`, `Precondition`, `NotModified`, `NotImplemented`, `PermissionDenied`, `Unauthenticated`, `UnknownConfigurationKey`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: path::Error) -> Self
fn from(source: tokio::task::JoinError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/object_store.Error.md).


A specialized `Error` for object store-related errors

---

## GetResultPayload

`enum` · `object_store::GetResultPayload`

Also reachable as `datafusion::object_store::GetResultPayload`

```rust
enum GetResultPayload
```

**Variants**: `File`, `Stream`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/object_store.GetResultPayload.md).


The kind of a [`GetResult`]

This special cases the case of a local file, as some systems may
be able to optimise the case of a file already present on local disk

---

## PutMode

`enum` · `object_store::PutMode`

Also reachable as `datafusion::object_store::PutMode`

```rust
enum PutMode
```

**Variants**: `Overwrite`, `Create`, `Update`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/object_store.PutMode.md).


Configure preconditions for the put operation

---

## RenameTargetMode

`enum` · `object_store::RenameTargetMode`

Also reachable as `datafusion::object_store::RenameTargetMode`

```rust
enum RenameTargetMode
```

**Variants**: `Overwrite`, `Create`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/object_store.RenameTargetMode.md).


Configure preconditions for the target of rename operation.

Note though that the source location may or not be deleted at the same time in an atomic operation. There is
currently NO flag to control the atomicity of "delete source at the same time as creating the target".

---

## CopyOptions

`struct` · `object_store::CopyOptions`

Also reachable as `datafusion::object_store::CopyOptions`

```rust
struct CopyOptions
```

**Fields**: `mode`, `extensions`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**Methods** (3)

```rust
fn new() -> Self
fn with_extensions(self, extensions: Extensions) -> Self
fn with_mode(self, mode: CopyMode) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.CopyOptions.md).


Options for a copy request

---

## GetOptions

`struct` · `object_store::GetOptions`

Also reachable as `datafusion::object_store::GetOptions`

```rust
struct GetOptions
```

**Fields**: `if_match`, `if_none_match`, `if_modified_since`, `if_unmodified_since`, `range`, `version`, `head`, `extensions`

**Derives**: Clone, Debug, Default

**Methods** (10)

```rust
fn check_preconditions(&self, meta: &ObjectMeta) -> Result<()>
fn new() -> Self
fn with_extensions(self, extensions: Extensions) -> Self
fn with_head(self, head: impl Into<bool>) -> Self
fn with_if_match(self, etag: Option<impl Into<String>>) -> Self
fn with_if_modified_since(self, dt: Option<impl Into<DateTime<Utc>>>) -> Self
fn with_if_none_match(self, etag: Option<impl Into<String>>) -> Self
fn with_if_unmodified_since(self, dt: Option<impl Into<DateTime<Utc>>>) -> Self
fn with_range(self, range: Option<impl Into<GetRange>>) -> Self
fn with_version(self, version: Option<impl Into<String>>) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.GetOptions.md).


Options for a get request, such as range

---

## GetResult

`struct` · `object_store::GetResult`

Also reachable as `datafusion::object_store::GetResult`

```rust
struct GetResult
```

**Fields**: `payload`, `meta`, `range`, `attributes`

**Derives**: Debug

**Methods** (2)

```rust
async fn bytes(self) -> Result<Bytes>
fn into_stream(self) -> BoxStream<'static, Result<Bytes>>
```

[Full member, field, variant and typed contracts](../operations/object_store.GetResult.md).


Result for a get request

---

## ListResult

`struct` · `object_store::ListResult`

Also reachable as `datafusion::object_store::ListResult`

```rust
struct ListResult
```

**Fields**: `common_prefixes`, `objects`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/object_store.ListResult.md).


Result of a list call that includes objects, prefixes (directories) and a
token for the next set of results. Individual result sets may be limited to
1,000 objects based on the underlying object storage's limitations.

---

## ObjectMeta

`struct` · `object_store::ObjectMeta`

Also reachable as `datafusion::object_store::ObjectMeta`

```rust
struct ObjectMeta
```

**Fields**: `location`, `last_modified`, `size`, `e_tag`, `version`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/object_store.ObjectMeta.md).


The metadata that describes an object.

---

## PutMultipartOptions

`struct` · `object_store::PutMultipartOptions`

Also reachable as `datafusion::object_store::PutMultipartOptions`

```rust
struct PutMultipartOptions
```

**Fields**: `tags`, `attributes`, `extensions`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**via `core::convert::From`**

```rust
fn from(attributes: Attributes) -> Self
fn from(tags: TagSet) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.PutMultipartOptions.md).


Options for [`ObjectStore::put_multipart_opts`]

---

## PutOptions

`struct` · `object_store::PutOptions`

Also reachable as `datafusion::object_store::PutOptions`

```rust
struct PutOptions
```

**Fields**: `mode`, `tags`, `attributes`, `extensions`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**via `core::convert::From`**

```rust
fn from(mode: PutMode) -> Self
fn from(attributes: Attributes) -> Self
fn from(tags: TagSet) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.PutOptions.md).


Options for a put request

---

## PutResult

`struct` · `object_store::PutResult`

Also reachable as `datafusion::object_store::PutResult`

```rust
struct PutResult
```

**Fields**: `e_tag`, `version`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/object_store.PutResult.md).


Result for a put request

---

## RenameOptions

`struct` · `object_store::RenameOptions`

Also reachable as `datafusion::object_store::RenameOptions`

```rust
struct RenameOptions
```

**Fields**: `target_mode`, `extensions`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**Methods** (3)

```rust
fn new() -> Self
fn with_extensions(self, extensions: Extensions) -> Self
fn with_target_mode(self, target_mode: RenameTargetMode) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.RenameOptions.md).


Options for a rename request

---

## UpdateVersion

`struct` · `object_store::UpdateVersion`

Also reachable as `datafusion::object_store::UpdateVersion`

```rust
struct UpdateVersion
```

**Fields**: `e_tag`, `version`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: PutResult) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.UpdateVersion.md).


Uniquely identifies a version of an object to update

Stores will use differing combinations of `e_tag` and `version` to provide conditional
updates, and it is therefore recommended applications preserve both

---

## ObjectStore

`trait` · `object_store::ObjectStore`

Also reachable as `datafusion::object_store::ObjectStore`

```rust
trait ObjectStore: std::fmt::Display + Send + Sync + Debug + 'static
```

**Implementors** (12)

- `alloc::boxed::Box`
- `alloc::sync::Arc`
- `object_store::aws::AmazonS3`
- `object_store::azure::MicrosoftAzure`
- `object_store::chunked::ChunkedStore`
- `object_store::gcp::GoogleCloudStorage`
- `object_store::http::HttpStore`
- `object_store::limit::LimitStore`
- `object_store::local::LocalFileSystem`
- `object_store::memory::InMemory`
- `object_store::prefix::PrefixStore`
- `object_store::throttle::ThrottledStore`

**Methods** (10)

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/object_store.ObjectStore.md).


Universal API for object store services.

See the [module-level documentation](crate) for a high level overview and
examples. See [`ObjectStoreExt`] for additional convenience methods.

# Contract
This trait is a contract between object store _implementations_
(e.g. providers, wrappers) and the `object_store` crate itself. It is
intended to be the minimum API required for an object store.

The [`ObjectStoreExt`] acts as an API/contract between `object_store` and
the store _users_ and provides additional methods that may be simpler to use
but overlap in functionality with [`ObjectStore`].

# Clone
If a store implements [`Clone`], that will only clone the handle to the underlying data. It will NOT clone/fork the
actual key-value data. Hence, the cloned instance and the original instance share the same state.

# Minimal Default Implementations
There are only a few default implementations for methods in this trait by
design. This was different from versions prior to `0.13.0`, which had many
more default implementations. Default implementations are convenient for
users, but error-prone for implementors as they require keeping the
convenience APIs correctly in sync.

As of version 0.13.0, most methods on [`ObjectStore`] must be implemented, and
the convenience methods have been moved to the [`ObjectStoreExt`] trait as
described above. See [#385] for more details.

[#385]: https://github.com/apache/arrow-rs-object-store/issues/385

# Wrappers
If you wrap an [`ObjectStore`] -- e.g. to add observability -- you SHOULD
implement all trait methods. This ensures that defaults implementations
that are overwritten by the wrapped store are also used by the wrapper.
For example:

```ignore
struct MyStore {
    ...
}

#[async_trait]
impl ObjectStore for MyStore {
    // implement custom ranges handling
    async fn get_ranges(
        &self,
        location: &Path,
        ranges: &[Range<u64>],
    ) -> Result<Vec<Bytes>> {
        ...
    }

    ...
}

struct Wrapper {
    inner: Arc<dyn ObjectStore>,
}

#[async_trait]
#[deny(clippy::missing_trait_methods)]
impl ObjectStore for Wrapper {
    // If we would not implement this method,
    // we would get the trait default and not
    // use the actual implementation of `inner`.
    async fn get_ranges(
        &self,
        location: &Path,
        ranges: &[Range<u64>],
    ) -> Result<Vec<Bytes>> {
        ...
    }

    ...
}
```

To automatically detect this issue, use
[`#[deny(clippy::missing_trait_methods)]`](https://rust-lang.github.io/rust-clippy/master/index.html#missing_trait_methods).

# Upgrade Guide for 0.13.0

Upgrading to object_store 0.13.0 from an earlier version typically involves:

1. Add a `use` for [`ObjectStoreExt`] to solve the error

```text
error[E0599]: no method named `put` found for reference `&dyn object_store::ObjectStore` in the current scope
   --> datafusion/datasource/src/url.rs:993:14
```

2. Remove any (now) redundant implementations (such as `ObjectStore::put`) from any
  `ObjectStore` implementations to resolve the error

```text
error[E0407]: method `put` is not a member of trait `ObjectStore`
    --> datafusion/datasource/src/url.rs:1103:9
     |
```

3. Convert `ObjectStore::delete` to [`ObjectStore::delete_stream`] (see documentation
   on that method for details and examples)

4. Combine `ObjectStore::copy` and `ObjectStore::copy_if_not_exists` implementations into
   [`ObjectStore::copy_opts`] (see documentation on that method for details and examples)

5. Update `object_store::Error::NotImplemented` to include the name of the missing method

For example, change instances of
```text
object_store::Error::NotImplemented
```
to
```
object_store::Error::NotImplemented {
   operation: "put".to_string(),
   implementer: "RequestCountingObjectStore".to_string(),
 };
```

---

## ObjectStoreExt

`trait` · `object_store::ObjectStoreExt`

Also reachable as `datafusion::object_store::ObjectStoreExt`

```rust
trait ObjectStoreExt: ObjectStore
```

**Methods** (10)

```rust
fn copy(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
fn copy_if_not_exists(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
fn delete(&self, location: &Path) -> impl Future<Output = Result<()>>
fn get(&self, location: &Path) -> impl Future<Output = Result<GetResult>>
fn get_range(&self, location: &Path, range: Range<u64>) -> impl Future<Output = Result<Bytes>>
fn head(&self, location: &Path) -> impl Future<Output = Result<ObjectMeta>>
fn put(&self, location: &Path, payload: PutPayload) -> impl Future<Output = Result<PutResult>>
fn put_multipart(&self, location: &Path) -> impl Future<Output = Result<Box<dyn MultipartUpload>>>
fn rename(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
fn rename_if_not_exists(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
```

[Full member, field, variant and typed contracts](../operations/object_store.ObjectStoreExt.md).


Extension trait for [`ObjectStore`] with convenience functions.

See the [module-level documentation](crate) for a high level overview and
examples. See "contract" section within the [`ObjectStore`] documentation
for more reasoning.

# Implementation
You MUST NOT implement this trait yourself. It is automatically implemented for all [`ObjectStore`] implementations.

---

## DynObjectStore

`type_alias` · `object_store::DynObjectStore`

Also reachable as `datafusion::object_store::DynObjectStore`

```rust
type DynObjectStore = dyn ObjectStore
```

[Full member, field, variant and typed contracts](../operations/object_store.DynObjectStore.md).


An alias for a dynamically dispatched object store implementation.

---

## MultipartId

`type_alias` · `object_store::MultipartId`

Also reachable as `datafusion::object_store::MultipartId`

```rust
type MultipartId = String
```

[Full member, field, variant and typed contracts](../operations/object_store.MultipartId.md).


Id type for multipart uploads.

---

## Result

`type_alias` · `object_store::Result`

Also reachable as `datafusion::object_store::Result`

```rust
type Result<T, E = Error> = std::result::Result<T, E>
```

[Full member, field, variant and typed contracts](../operations/object_store.Result.md).


A specialized `Result` for object store-related errors

---
