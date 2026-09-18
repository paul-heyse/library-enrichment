# `object_store::ObjectStore`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.ObjectStore.json).

<a id="op-94894eaf9e5f6b785baca8ca"></a>
## ObjectStore

`trait` · `object_store::ObjectStore` · object_store 0.13.2

```rust
trait ObjectStore: std::fmt::Display + Send + Sync + Debug + 'static
```

Source: `src/lib.rs:746`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Universal API for object store services.

See the [module-level documentation](crate) for a high level overview and
examples. See [`ObjectStoreExt`](../operations/object_store.ObjectStoreExt.md#op-20c376c84f546ecb167a6a88) for additional convenience methods.

# Contract
This trait is a contract between object store _implementations_
(e.g. providers, wrappers) and the `object_store` crate itself. It is
intended to be the minimum API required for an object store.

The [`ObjectStoreExt`](../operations/object_store.ObjectStoreExt.md#op-20c376c84f546ecb167a6a88) acts as an API/contract between `object_store` and
the store _users_ and provides additional methods that may be simpler to use
but overlap in functionality with [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca).

# Clone
If a store implements [`Clone`], that will only clone the handle to the underlying data. It will NOT clone/fork the
actual key-value data. Hence, the cloned instance and the original instance share the same state.

# Minimal Default Implementations
There are only a few default implementations for methods in this trait by
design. This was different from versions prior to `0.13.0`, which had many
more default implementations. Default implementations are convenient for
users, but error-prone for implementors as they require keeping the
convenience APIs correctly in sync.

As of version 0.13.0, most methods on [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) must be implemented, and
the convenience methods have been moved to the [`ObjectStoreExt`](../operations/object_store.ObjectStoreExt.md#op-20c376c84f546ecb167a6a88) trait as
described above. See [#385] for more details.

[#385]: https://github.com/apache/arrow-rs-object-store/issues/385

# Wrappers
If you wrap an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) -- e.g. to add observability -- you SHOULD
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

1. Add a `use` for [`ObjectStoreExt`](../operations/object_store.ObjectStoreExt.md#op-20c376c84f546ecb167a6a88) to solve the error

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

3. Convert `ObjectStore::delete` to [`ObjectStore::delete_stream`](../operations/object_store.ObjectStore.md#op-f0edb1771fce4df7c490c451) (see documentation
   on that method for details and examples)

4. Combine `ObjectStore::copy` and `ObjectStore::copy_if_not_exists` implementations into
   [`ObjectStore::copy_opts`](../operations/object_store.ObjectStore.md#op-bf8918868a8df49bc2d03102) (see documentation on that method for details and examples)

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


Unresolved upstream links (retained, not inferred): ``Clone``.

<a id="op-bf8918868a8df49bc2d03102"></a>
## copy_opts

`function` · `object_store::ObjectStore::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Source: `src/lib.rs:1111`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Copy an object from one path to another in the same object store.

<a id="op-f0edb1771fce4df7c490c451"></a>
## delete_stream

`function` · `object_store::ObjectStore::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Source: `src/lib.rs:1066`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Delete all the objects at the specified locations

When supported, this method will use bulk operations that delete more
than one object per a request. Otherwise, the implementation may call
the single object delete method for each location.

# Bulk Delete Support

The following backends support native bulk delete operations:

- **AWS (S3)**: Uses the native [DeleteObjects] API with batches of up to 1000 objects
- **Azure**: Uses the native [Blob Batch] API with batches of up to 256 objects

The following backends use concurrent individual delete operations:

- **GCP**: Performs individual delete requests with up to 10 concurrent operations
- **HTTP**: Performs individual delete requests with up to 10 concurrent operations
- **Local**: Performs individual file deletions with up to 10 concurrent operations
- **Memory**: Performs individual in-memory deletions sequentially

[DeleteObjects]: https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteObjects.html
[Blob Batch]: https://learn.microsoft.com/en-us/rest/api/storageservices/blob-batch

The returned stream yields the results of the delete operations in the
same order as the input locations. However, some errors will be from
an overall call to a bulk delete operation, and not from a specific
location.

If the object did not exist, the result may be an error or a success,
depending on the behavior of the underlying store. For example, local
filesystems, GCP, and Azure return an error, while S3 and in-memory will
return Ok. If it is an error, it will be [`Error::NotFound`](../operations/object_store.Error.md#op-05c916c94bd0100476adb324).

```ignore-wasm32
# use futures_util::{StreamExt, TryStreamExt};
# use object_store::local::LocalFileSystem;
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
# let root = tempfile::TempDir::new().unwrap();
# let store = LocalFileSystem::new_with_prefix(root.path()).unwrap();
# use object_store::{ObjectStore, ObjectStoreExt, ObjectMeta};
# use object_store::path::Path;
# use futures_util::{StreamExt, TryStreamExt};
#
// Create two objects
store.put(&Path::from("foo"), "foo".into()).await?;
store.put(&Path::from("bar"), "bar".into()).await?;

// List object
let locations = store.list(None).map_ok(|m| m.location).boxed();

// Delete them
store.delete_stream(locations).try_collect::<Vec<Path>>().await?;
# Ok(())
# }
# let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
# rt.block_on(example()).unwrap();
```

Note: Before version 0.13, `delete_stream` has a default implementation
that deletes each object with up to 10 concurrent requests. This default
behavior has been removed, and each implementation must now provide its
own `delete_stream` implementation explicitly. The following example
shows how to implement `delete_stream` to get the previous default
behavior.

```
# use async_trait::async_trait;
# use futures_util::stream::{BoxStream, StreamExt};
# use object_store::path::Path;
# use object_store::{
#     CopyOptions, GetOptions, GetResult, ListResult, MultipartUpload, ObjectMeta, ObjectStore,
#     PutMultipartOptions, PutOptions, PutPayload, PutResult, Result,
# };
# use std::fmt;
# use std::fmt::Debug;
# use std::sync::Arc;
#
# struct ExampleClient;
#
# impl ExampleClient {
#     async fn delete(&self, _path: &Path) -> Result<()> {
#         Ok(())
#     }
# }
#
# struct ExampleStore {
#     client: Arc<ExampleClient>,
# }
#
# impl Debug for ExampleStore {
#     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
#         write!(f, "ExampleStore")
#     }
# }
#
# impl fmt::Display for ExampleStore {
#     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
#         write!(f, "ExampleStore")
#     }
# }
#
# #[async_trait]
# impl ObjectStore for ExampleStore {
#     async fn put_opts(&self, _: &Path, _: PutPayload, _: PutOptions) -> Result<PutResult> {
#         todo!()
#     }
#
#     async fn put_multipart_opts(
#         &self,
#         _: &Path,
#         _: PutMultipartOptions,
#     ) -> Result<Box<dyn MultipartUpload>> {
#         todo!()
#     }
#
#     async fn get_opts(&self, _: &Path, _: GetOptions) -> Result<GetResult> {
#         todo!()
#     }
#
fn delete_stream(
    &self,
    locations: BoxStream<'static, Result<Path>>,
) -> BoxStream<'static, Result<Path>> {
    let client = Arc::clone(&self.client);
    locations
        .map(move |location| {
            let client = Arc::clone(&client);
            async move {
                let location = location?;
                client.delete(&location).await?;
                Ok(location)
            }
        })
        .buffered(10)
        .boxed()
}
#
#     fn list(&self, _: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>> {
#         todo!()
#     }
#
#     async fn list_with_delimiter(&self, _: Option<&Path>) -> Result<ListResult> {
#         todo!()
#     }
#
#     async fn copy_opts(&self, _: &Path, _: &Path, _: CopyOptions) -> Result<()> {
#         todo!()
#     }
# }
#
# async fn example() {
#     let store = ExampleStore { client: Arc::new(ExampleClient) };
#     let paths = futures_util::stream::iter(vec![Ok(Path::from("foo")), Ok(Path::from("bar"))]).boxed();
#     let results = store.delete_stream(paths).collect::<Vec<_>>().await;
#     assert_eq!(results.len(), 2);
#     assert_eq!(results[0].as_ref().unwrap(), &Path::from("foo"));
#     assert_eq!(results[1].as_ref().unwrap(), &Path::from("bar"));
# }
#
# let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
# rt.block_on(example());
```

<a id="op-0eb8121eee6ac22c92ee9da0"></a>
## get_opts

`function` · `object_store::ObjectStore::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Source: `src/lib.rs:891`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform a get request with options

## Example

This example uses a basic local filesystem object store to get an object with a specific etag.
On the local filesystem, supplying an invalid etag will error.
Versioned object stores will return the specified object version, if it exists.

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use tempfile::tempdir;
# use object_store::{path::Path, ObjectStore, ObjectStoreExt, GetOptions};
async fn get_opts_example() {
    let tmp = tempdir().unwrap();
    let store = LocalFileSystem::new_with_prefix(tmp.path()).unwrap();
    let location = Path::from("example.txt");
    let content = b"Hello, Object Store!";

    // Put the object into the store
    store
        .put(&location, content.as_ref().into())
        .await
        .expect("Failed to put object");

    // Get the object from the store to figure out the right etag
    let result: object_store::GetResult = store.get(&location).await.expect("Failed to get object");

    let etag = result.meta.e_tag.expect("ETag should be present");

    // Get the object from the store with range and etag
    let bytes = store
        .get_opts(
            &location,
            GetOptions::new()
                .with_if_match(Some(etag.clone())),
        )
        .await
        .expect("Failed to get object with range and etag")
        .bytes()
        .await
        .expect("Failed to read bytes");

    println!(
        "Retrieved with ETag {}: {}",
        etag,
        String::from_utf8_lossy(&bytes)
    );

    // Show that if the etag does not match, we get an error
    let wrong_etag = "wrong-etag".to_string();
    match store
        .get_opts(
            &location,
            GetOptions::new().with_if_match(Some(wrong_etag))
        )
        .await
    {
        Ok(_) => println!("Unexpectedly succeeded with wrong ETag"),
        Err(e) => println!("On a non-versioned object store, getting an invalid ETag ('wrong-etag') results in an error as expected: {}", e),
    }
}
```

To retrieve a range of bytes from a versioned object, specify the range in the [`GetOptions`](../operations/object_store.GetOptions.md#op-eb3de57203b85138a693ae44) supplied to this method.

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use tempfile::tempdir;
# use object_store::{path::Path, ObjectStore, ObjectStoreExt, GetOptions};
async fn get_opts_range_example() {
    let tmp = tempdir().unwrap();
    let store = LocalFileSystem::new_with_prefix(tmp.path()).unwrap();
    let location = Path::from("example.txt");
    let content = b"Hello, Object Store!";

    // Put the object into the store
    store
        .put(&location, content.as_ref().into())
        .await
        .expect("Failed to put object");

    // Get the object from the store to figure out the right etag
    let result: object_store::GetResult = store.get(&location).await.expect("Failed to get object");

    let etag = result.meta.e_tag.expect("ETag should be present");

    // Get the object from the store with range and etag
    let bytes = store
        .get_opts(
            &location,
            GetOptions::new()
                .with_range(Some(0..5))
                .with_if_match(Some(etag.clone())),
        )
        .await
        .expect("Failed to get object with range and etag")
        .bytes()
        .await
        .expect("Failed to read bytes");

    println!(
        "Retrieved range [0-5] with ETag {}: {}",
        etag,
        String::from_utf8_lossy(&bytes)
    );

    // Show that if the etag does not match, we get an error
    let wrong_etag = "wrong-etag".to_string();
    match store
        .get_opts(
            &location,
            GetOptions::new().with_range(Some(0..5)).with_if_match(Some(wrong_etag))
        )
        .await
    {
        Ok(_) => println!("Unexpectedly succeeded with wrong ETag"),
        Err(e) => println!("On a non-versioned object store, getting an invalid ETag ('wrong-etag') results in an error as expected: {}", e),
    }
}
```

<a id="op-89edf38f0aecae998b9e9357"></a>
## get_ranges

`function` · `object_store::ObjectStore::get_ranges` · object_store 0.13.2

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
```

Source: `src/lib.rs:895`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return the bytes that are stored at the specified location
in the given byte ranges

<a id="op-eaf50d65c0fa0bf3019d428b"></a>
## list

`function` · `object_store::ObjectStore::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Source: `src/lib.rs:1079`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

List all the objects with the given prefix.

Prefixes are evaluated on a path segment basis, i.e. `foo/bar` is a prefix of `foo/bar/x` but not of
`foo/bar_baz/x`. List is recursive, i.e. `foo/bar/more/x` will be included.

Note: the order of returned [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518) is not guaranteed

For more advanced listing see [`PaginatedListStore`](list::PaginatedListStore)

<a id="op-5534cc89a3fb17e5a0e15d20"></a>
## list_with_delimiter

`function` · `object_store::ObjectStore::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Source: `src/lib.rs:1108`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

List objects with the given prefix and an implementation specific
delimiter. Returns common prefixes (directories) in addition to object
metadata.

Prefixes are evaluated on a path segment basis, i.e. `foo/bar` is a prefix of `foo/bar/x` but not of
`foo/bar_baz/x`. List is not recursive, i.e. `foo/bar/more/x` will not be included.

<a id="op-6c9f6dac06dc9bcb986fc957"></a>
## list_with_offset

`function` · `object_store::ObjectStore::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Source: `src/lib.rs:1091`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

List all the objects with the given prefix and a location greater than `offset`

Some stores, such as S3 and GCS, may be able to push `offset` down to reduce
the number of network requests required.

This returns an exclusive offset, i.e. objects at exactly `offset` will not be included.

Note: the order of returned [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518) is not guaranteed

For more advanced listing see [`PaginatedListStore`](list::PaginatedListStore)

<a id="op-27dfce2ef1fd51c4e2938336"></a>
## put_multipart_opts

`function` · `object_store::ObjectStore::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Source: `src/lib.rs:765`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform a multipart upload with options

Client should prefer [`ObjectStore::put_opts`](../operations/object_store.ObjectStore.md#op-4938238191db15fe350c71b9) for small payloads, as streaming uploads
typically require multiple separate requests. See [`MultipartUpload`](../operations/object_store.upload.MultipartUpload.md#op-d1bf78ac32fe2ad950a9d5d0) for more information

For more advanced multipart uploads see [`MultipartStore`](multipart::MultipartStore)

<a id="op-4938238191db15fe350c71b9"></a>
## put_opts

`function` · `object_store::ObjectStore::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Source: `src/lib.rs:752`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Save the provided `payload` to `location` with the given options

The operation is guaranteed to be atomic, it will either successfully
write the entirety of `payload` to `location`, or fail. No clients
should be able to observe a partially written object

<a id="op-ed893287b7ec1c1637a55e09"></a>
## rename_opts

`function` · `object_store::ObjectStore::rename_opts` · object_store 0.13.2

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

Source: `src/lib.rs:1117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Move an object from one path to another in the same object store.

By default, this is implemented as a copy and then delete source. It may not
check when deleting source that it was the same object that was originally copied.
