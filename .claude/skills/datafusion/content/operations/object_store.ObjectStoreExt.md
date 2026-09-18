# `object_store::ObjectStoreExt`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.ObjectStoreExt.json).

<a id="op-20c376c84f546ecb167a6a88"></a>
## ObjectStoreExt

`trait` · `object_store::ObjectStoreExt` · object_store 0.13.2

```rust
trait ObjectStoreExt: ObjectStore
```

Source: `src/lib.rs:1220`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Extension trait for [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) with convenience functions.

See the [module-level documentation](crate) for a high level overview and
examples. See "contract" section within the [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) documentation
for more reasoning.

# Implementation
You MUST NOT implement this trait yourself. It is automatically implemented for all [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations.

<a id="op-3fbfa40d12641304f23c9222"></a>
## copy

`function` · `object_store::ObjectStoreExt::copy` · object_store 0.13.2

```rust
fn copy(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
```

Source: `src/lib.rs:1315`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Copy an object from one path to another in the same object store.

If there exists an object at the destination, it will be overwritten.

<a id="op-a42f4998b2530859df7a79df"></a>
## copy_if_not_exists

`function` · `object_store::ObjectStoreExt::copy_if_not_exists` · object_store 0.13.2

```rust
fn copy_if_not_exists(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
```

Source: `src/lib.rs:1324`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Copy an object from one path to another, only if destination is empty.

Will return an error if the destination already has an object.

Performs an atomic operation if the underlying object storage supports it.
If atomic operations are not supported by the underlying object storage (like S3)
it will return an error.

<a id="op-e6dfde49a3805cf7f5261abc"></a>
## delete

`function` · `object_store::ObjectStoreExt::delete` · object_store 0.13.2

```rust
fn delete(&self, location: &Path) -> impl Future<Output = Result<()>>
```

Source: `src/lib.rs:1310`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Delete the object at the specified location.

<a id="op-547adec24b48cc0bdb8cd456"></a>
## get

`function` · `object_store::ObjectStoreExt::get` · object_store 0.13.2

```rust
fn get(&self, location: &Path) -> impl Future<Output = Result<GetResult>>
```

Source: `src/lib.rs:1267`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return the bytes that are stored at the specified location.

## Example

This example uses a basic local filesystem object store to get an object.

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use tempfile::tempdir;
# use object_store::{path::Path, ObjectStore, ObjectStoreExt};
async fn get_example() {
    let tmp = tempdir().unwrap();
    let store = LocalFileSystem::new_with_prefix(tmp.path()).unwrap();
    let location = Path::from("example.txt");
    let content = b"Hello, Object Store!";

    // Put the object into the store
    store
        .put(&location, content.as_ref().into())
        .await
        .expect("Failed to put object");

    // Get the object from the store
    let get_result = store.get(&location).await.expect("Failed to get object");
    let bytes = get_result.bytes().await.expect("Failed to read bytes");
    println!("Retrieved content: {}", String::from_utf8_lossy(&bytes));
}
```

<a id="op-c46044b5384a5c5c106c5137"></a>
## get_range

`function` · `object_store::ObjectStoreExt::get_range` · object_store 0.13.2

```rust
fn get_range(&self, location: &Path, range: Range<u64>) -> impl Future<Output = Result<Bytes>>
```

Source: `src/lib.rs:1304`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return the bytes that are stored at the specified location
in the given byte range.

See [`GetRange::Bounded`](../operations/object_store.util.GetRange.md#op-65a32023cea31c5abe6ac3e6) for more details on how `range` gets interpreted.

To retrieve a range of bytes from a versioned object, use [`ObjectStore::get_opts`](../operations/object_store.ObjectStore.md#op-0eb8121eee6ac22c92ee9da0) by specifying the range in the [`GetOptions`](../operations/object_store.GetOptions.md#op-eb3de57203b85138a693ae44).

## Examples

This example uses a basic local filesystem object store to get a byte range from an object.

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use tempfile::tempdir;
# use object_store::{path::Path, ObjectStore, ObjectStoreExt};
async fn get_range_example() {
    let tmp = tempdir().unwrap();
    let store = LocalFileSystem::new_with_prefix(tmp.path()).unwrap();
    let location = Path::from("example.txt");
    let content = b"Hello, Object Store!";

    // Put the object into the store
    store
        .put(&location, content.as_ref().into())
        .await
        .expect("Failed to put object");

    // Get the object from the store
    let bytes = store
        .get_range(&location, 0..5)
        .await
        .expect("Failed to get object");
    println!("Retrieved range [0-5]: {}", String::from_utf8_lossy(&bytes));
}
```

<a id="op-e14b1b9ae2393005fc5a5a50"></a>
## head

`function` · `object_store::ObjectStoreExt::head` · object_store 0.13.2

```rust
fn head(&self, location: &Path) -> impl Future<Output = Result<ObjectMeta>>
```

Source: `src/lib.rs:1307`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return the metadata for the specified location

<a id="op-a1072dd9112ad9b932a9c13e"></a>
## put

`function` · `object_store::ObjectStoreExt::put` · object_store 0.13.2

```rust
fn put(&self, location: &Path, payload: PutPayload) -> impl Future<Output = Result<PutResult>>
```

Source: `src/lib.rs:1226`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Save the provided bytes to the specified location

The operation is guaranteed to be atomic, it will either successfully
write the entirety of `payload` to `location`, or fail. No clients
should be able to observe a partially written object

<a id="op-1b08e290f936482b5d2e4bb0"></a>
## put_multipart

`function` · `object_store::ObjectStoreExt::put_multipart` · object_store 0.13.2

```rust
fn put_multipart(&self, location: &Path) -> impl Future<Output = Result<Box<dyn MultipartUpload>>>
```

Source: `src/lib.rs:1234`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform a multipart upload

Client should prefer [`ObjectStoreExt::put`](../operations/object_store.ObjectStoreExt.md#op-a1072dd9112ad9b932a9c13e) for small payloads, as streaming uploads
typically require multiple separate requests. See [`MultipartUpload`](../operations/object_store.upload.MultipartUpload.md#op-d1bf78ac32fe2ad950a9d5d0) for more information

For more advanced multipart uploads see [`MultipartStore`](multipart::MultipartStore)

<a id="op-cadb3e51a61bf63f9c119590"></a>
## rename

`function` · `object_store::ObjectStoreExt::rename` · object_store 0.13.2

```rust
fn rename(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
```

Source: `src/lib.rs:1332`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Move an object from one path to another in the same object store.

By default, this is implemented as a copy and then delete source. It may not
check when deleting source that it was the same object that was originally copied.

If there exists an object at the destination, it will be overwritten.

<a id="op-64c0edb6820401a78d6079f6"></a>
## rename_if_not_exists

`function` · `object_store::ObjectStoreExt::rename_if_not_exists` · object_store 0.13.2

```rust
fn rename_if_not_exists(&self, from: &Path, to: &Path) -> impl Future<Output = Result<()>>
```

Source: `src/lib.rs:1337`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Move an object from one path to another in the same object store.

Will return an error if the destination already has an object.
