# ObjectStore

`object_store::ObjectStore`

```rust
trait ObjectStore: std::fmt::Display + Send + Sync + Debug + 'static
```

Also reachable as `datafusion::object_store::ObjectStore`

Prose: [`api/object_store.md`](../api/object_store.md#objectstore) · records: [`model/object_store.json`](../model/object_store.json)

## Required

Every implementation must supply these.

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

## Implementors (12)

Read one before writing your own.

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

## Documentation

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
