# `object_store`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.json).

<a id="op-61af97d6b5407781ed1810a8"></a>
## object_store

`module` · `object_store` · object_store 0.13.2

```rust
mod object_store
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

# object_store

This crate provides a uniform API for interacting with object
storage services and local files via the [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca)
trait.

Using this crate, the same binary and code can run in multiple
clouds and local test environments, via a simple runtime
configuration change.

# Highlights

1. A high-performance async API focused on providing a consistent interface
   mirroring that of object stores such as [S3]

2. Production quality, leading this crate to be used in large
   scale production systems, such as [crates.io] and [InfluxDB IOx]

3. Support for advanced functionality, including atomic, conditional reads
   and writes, vectored IO, bulk deletion, and more...

4. Stable and predictable governance via the [Apache Arrow] project

5. Small dependency footprint, depending on only a small number of common crates

Originally developed by [InfluxData] and subsequently donated
to [Apache Arrow].

[Apache Arrow]: https://arrow.apache.org/
[InfluxData]: https://www.influxdata.com/
[crates.io]: https://github.com/rust-lang/crates.io
[ACID]: https://en.wikipedia.org/wiki/ACID
[S3]: https://aws.amazon.com/s3/

# APIs

* [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca): Core object store API
* [`ObjectStoreExt`](../operations/object_store.ObjectStoreExt.md#op-20c376c84f546ecb167a6a88): (*New in 0.13.0*) Extension trait with additional convenience methods

# Available [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) Implementations

By default, this crate provides the following implementations:

* Memory: [`InMemory`](memory::InMemory)

Feature flags are used to enable support for other implementations:

* Local filesystem: [`LocalFileSystem`](local::LocalFileSystem)
* [`gcp`](../modules/object_store.gcp.md#op-397cb506fa80f7a4f9953b4d): [Google Cloud Storage](https://cloud.google.com/storage/) support. See [`GoogleCloudStorageBuilder`](gcp::GoogleCloudStorageBuilder)
* [`aws`](../modules/object_store.aws.md#op-91f16db41cdd768042f8c13f): [Amazon S3](https://aws.amazon.com/s3/). See [`AmazonS3Builder`](aws::AmazonS3Builder)
* [`azure`](../modules/object_store.azure.md#op-e8cabc3351f612dca436727f): [Azure Blob Storage](https://azure.microsoft.com/en-gb/services/storage/blobs/). See [`MicrosoftAzureBuilder`](azure::MicrosoftAzureBuilder)
* [`http`](../modules/object_store.http.md#op-864ea065fc3b64d53597d706): [HTTP/WebDAV Storage](https://datatracker.ietf.org/doc/html/rfc2518). See [`HttpBuilder`](http::HttpBuilder)

# Why not a Filesystem Interface?

The [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) interface is designed to mirror the APIs
of object stores and *not* filesystems, and thus has stateless APIs instead
of cursor based interfaces such as [`Read`] or [`Seek`] available in filesystems.

This design provides the following advantages:

* All operations are atomic, and readers cannot observe partial and/or failed writes
* Methods map directly to object store APIs, providing both efficiency and predictability
* Abstracts away filesystem and operating system specific quirks, ensuring portability
* Allows for functionality not native to filesystems, such as operation preconditions
  and atomic multipart uploads

This crate does provide [`BufReader`] and [`BufWriter`] adapters
which provide a more filesystem-like API for working with the
[`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) trait, however, they should be used with care

[`BufReader`]: buffered::BufReader
[`BufWriter`]: buffered::BufWriter
[`Read`]: std::io::Read
[`Seek`]: std::io::Seek

# Adapters

[`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) instances can be composed with various adapters
which add additional functionality:

* Rate Throttling: [`ThrottleConfig`](throttle::ThrottleConfig)
* Concurrent Request Limit: [`LimitStore`](limit::LimitStore)

# Configuration System

This crate provides a configuration system inspired by the APIs exposed by [fsspec],
[PyArrow FileSystem], and [Hadoop FileSystem], allowing creating a [`DynObjectStore`](../operations/object_store.DynObjectStore.md#op-defcd6fd5cb1e6bb62bb8a3f)
from a URL and an optional list of key value pairs. This provides a flexible interface
to support a wide variety of user-defined store configurations, with minimal additional
application complexity.

```no_run,ignore-wasm32
# #[cfg(feature = "aws")] {
# use url::Url;
# use object_store::{parse_url, parse_url_opts};
# use object_store::aws::{AmazonS3, AmazonS3Builder};
#
#
// Can manually create a specific store variant using the appropriate builder
let store: AmazonS3 = AmazonS3Builder::from_env()
    .with_bucket_name("my-bucket").build().unwrap();

// Alternatively can create an ObjectStore from an S3 URL
let url = Url::parse("s3://bucket/path").unwrap();
let (store, path) = parse_url(&url).unwrap();
assert_eq!(path.as_ref(), "path");

// Potentially with additional options
let (store, path) = parse_url_opts(&url, vec![("aws_access_key_id", "...")]).unwrap();

// Or with URLs that encode the bucket name in the URL path
let url = Url::parse("https://ACCOUNT_ID.r2.cloudflarestorage.com/bucket/path").unwrap();
let (store, path) = parse_url(&url).unwrap();
assert_eq!(path.as_ref(), "path");
# }
```

[PyArrow FileSystem]: https://arrow.apache.org/docs/python/generated/pyarrow.fs.FileSystem.html#pyarrow.fs.FileSystem.from_uri
[fsspec]: https://filesystem-spec.readthedocs.io/en/latest/api.html#fsspec.filesystem
[Hadoop FileSystem]: https://hadoop.apache.org/docs/r3.0.0/api/org/apache/hadoop/fs/FileSystem.html#get-java.net.URI-org.apache.hadoop.conf.Configuration-

# List objects

Use the [`ObjectStore::list`](../operations/object_store.ObjectStore.md#op-eaf50d65c0fa0bf3019d428b) method to iterate over objects in
remote storage or files in the local filesystem:

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use std::sync::Arc;
# use object_store::{path::Path, ObjectStore};
# use futures_util::stream::StreamExt;
# // use LocalFileSystem for example
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(LocalFileSystem::new())
# }
#
# async fn example() {
#
// create an ObjectStore
let object_store: Arc<dyn ObjectStore> = get_object_store();

// Recursively list all files below the 'data' path.
// 1. On AWS S3 this would be the 'data/' prefix
// 2. On a local filesystem, this would be the 'data' directory
let prefix = Path::from("data");

// Get an `async` stream of Metadata objects:
let mut list_stream = object_store.list(Some(&prefix));

// Print a line about each object
while let Some(meta) = list_stream.next().await.transpose().unwrap() {
    println!("Name: {}, size: {}", meta.location, meta.size);
}
# }
```

Which will print out something like the following:

```text
Name: data/file01.parquet, size: 112832
Name: data/file02.parquet, size: 143119
Name: data/child/file03.parquet, size: 100
...
```

# Fetch objects

Use the [`ObjectStoreExt::get`](../operations/object_store.ObjectStoreExt.md#op-547adec24b48cc0bdb8cd456) / [`ObjectStore::get_opts`](../operations/object_store.ObjectStore.md#op-0eb8121eee6ac22c92ee9da0) method to fetch the data bytes
from remote storage or files in the local filesystem as a stream.

```ignore-wasm32
# use futures_util::TryStreamExt;
# use object_store::local::LocalFileSystem;
# use std::sync::Arc;
#  use bytes::Bytes;
# use object_store::{path::Path, ObjectStore, ObjectStoreExt, GetResult};
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(LocalFileSystem::new())
# }
#
# async fn example() {
#
// Create an ObjectStore
let object_store: Arc<dyn ObjectStore> = get_object_store();

// Retrieve a specific file
let path = Path::from("data/file01.parquet");

// Fetch just the file metadata
let meta = object_store.head(&path).await.unwrap();
println!("{meta:?}");

// Fetch the object including metadata
let result: GetResult = object_store.get(&path).await.unwrap();
assert_eq!(result.meta, meta);

// Buffer the entire object in memory
let object: Bytes = result.bytes().await.unwrap();
assert_eq!(object.len() as u64, meta.size);

// Alternatively stream the bytes from object storage
let stream = object_store.get(&path).await.unwrap().into_stream();

// Count the '0's using `try_fold` from `TryStreamExt` trait
let num_zeros = stream
    .try_fold(0, |acc, bytes| async move {
        Ok(acc + bytes.iter().filter(|b| **b == 0).count())
    }).await.unwrap();

println!("Num zeros in {} is {}", path, num_zeros);
# }
```

# Put Object

Use the [`ObjectStoreExt::put`](../operations/object_store.ObjectStoreExt.md#op-a1072dd9112ad9b932a9c13e) method to atomically write data.

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use object_store::{ObjectStore, ObjectStoreExt, PutPayload};
# use std::sync::Arc;
# use object_store::path::Path;
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(LocalFileSystem::new())
# }
# async fn put() {
#
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/file1");
let payload = PutPayload::from_static(b"hello");
object_store.put(&path, payload).await.unwrap();
# }
```

# Multipart Upload

Use the [`ObjectStoreExt::put_multipart`](../operations/object_store.ObjectStoreExt.md#op-1b08e290f936482b5d2e4bb0) / [`ObjectStore::put_multipart_opts`](../operations/object_store.ObjectStore.md#op-27dfce2ef1fd51c4e2938336) method to atomically write a large
amount of data

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use object_store::{ObjectStore, ObjectStoreExt, WriteMultipart};
# use std::sync::Arc;
# use bytes::Bytes;
# use tokio::io::AsyncWriteExt;
# use object_store::path::Path;
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(LocalFileSystem::new())
# }
# async fn multi_upload() {
#
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/large_file");
let upload =  object_store.put_multipart(&path).await.unwrap();
let mut write = WriteMultipart::new(upload);
write.write(b"hello");
write.finish().await.unwrap();
# }
```

# Vectored Read

A common pattern, especially when reading structured datasets, is to need to fetch
multiple, potentially non-contiguous, ranges of a particular object.

[`ObjectStore::get_ranges`](../operations/object_store.ObjectStore.md#op-89edf38f0aecae998b9e9357) provides an efficient way to perform such vectored IO, and will
automatically coalesce adjacent ranges into an appropriate number of parallel requests.

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use object_store::ObjectStore;
# use std::sync::Arc;
# use bytes::Bytes;
# use tokio::io::AsyncWriteExt;
# use object_store::path::Path;
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(LocalFileSystem::new())
# }
# async fn multi_upload() {
#
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/large_file");
let ranges = object_store.get_ranges(&path, &[90..100, 400..600, 0..10]).await.unwrap();
assert_eq!(ranges.len(), 3);
assert_eq!(ranges[0].len(), 10);
# }
```

To retrieve ranges from a versioned object, use [`ObjectStore::get_opts`](../operations/object_store.ObjectStore.md#op-0eb8121eee6ac22c92ee9da0) by specifying the range in the [`GetOptions`](../operations/object_store.GetOptions.md#op-eb3de57203b85138a693ae44).

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use object_store::ObjectStore;
# use object_store::GetOptions;
# use std::sync::Arc;
# use bytes::Bytes;
# use tokio::io::AsyncWriteExt;
# use object_store::path::Path;
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(LocalFileSystem::new())
# }
# async fn get_range_with_options() {
#
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/large_file");
let ranges = vec![90..100, 400..600, 0..10];
for range in ranges {
    let opts = GetOptions::default().with_range(Some(range));
    let data = object_store.get_opts(&path, opts).await.unwrap();
    // Do something with the data
}
# }
``````

# Vectored Write

When writing data it is often the case that the size of the output is not known ahead of time.

A common approach to handling this is to bump-allocate a `Vec`, whereby the underlying
allocation is repeatedly reallocated, each time doubling the capacity. The performance of
this is suboptimal as reallocating memory will often involve copying it to a new location.

Fortunately, as [`PutPayload`] does not require memory regions to be contiguous, it is
possible to instead allocate memory in chunks and avoid bump allocating. [`PutPayloadMut`]
encapsulates this approach

```ignore-wasm32
# use object_store::local::LocalFileSystem;
# use object_store::{ObjectStore, ObjectStoreExt, PutPayloadMut};
# use std::sync::Arc;
# use bytes::Bytes;
# use tokio::io::AsyncWriteExt;
# use object_store::path::Path;
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(LocalFileSystem::new())
# }
# async fn multi_upload() {
#
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/large_file");
let mut buffer = PutPayloadMut::new().with_block_size(8192);
for _ in 0..22 {
    buffer.extend_from_slice(&[0; 1024]);
}
let payload = buffer.freeze();

// Payload consists of 3 separate 8KB allocations
assert_eq!(payload.as_ref().len(), 3);
assert_eq!(payload.as_ref()[0].len(), 8192);
assert_eq!(payload.as_ref()[1].len(), 8192);
assert_eq!(payload.as_ref()[2].len(), 6144);

object_store.put(&path, payload).await.unwrap();
# }
```

# Conditional Fetch

More complex object retrieval can be supported by [`ObjectStore::get_opts`].

For example, efficiently refreshing a cache without re-fetching the entire object
data if the object hasn't been modified.

```
# use std::collections::btree_map::Entry;
# use std::collections::HashMap;
# use object_store::{GetOptions, GetResult, ObjectStore, ObjectStoreExt, Result, Error};
# use std::sync::Arc;
# use std::time::{Duration, Instant};
# use bytes::Bytes;
# use tokio::io::AsyncWriteExt;
# use object_store::path::Path;
struct CacheEntry {
    /// Data returned by last request
    data: Bytes,
    /// ETag identifying the object returned by the server
    e_tag: String,
    /// Instant of last refresh
    refreshed_at: Instant,
}

/// Example cache that checks entries after 10 seconds for a new version
struct Cache {
    entries: HashMap<Path, CacheEntry>,
    store: Arc<dyn ObjectStore>,
}

impl Cache {
    pub async fn get(&mut self, path: &Path) -> Result<Bytes> {
        Ok(match self.entries.get_mut(path) {
            Some(e) => match e.refreshed_at.elapsed() < Duration::from_secs(10) {
                true => e.data.clone(), // Return cached data
                false => { // Check if remote version has changed
                    let opts = GetOptions::new().with_if_none_match(Some(e.e_tag.clone()));
                    match self.store.get_opts(&path, opts).await {
                        Ok(d) => e.data = d.bytes().await?,
                        Err(Error::NotModified { .. }) => {} // Data has not changed
                        Err(e) => return Err(e),
                    };
                    e.refreshed_at = Instant::now();
                    e.data.clone()
                }
            },
            None => { // Not cached, fetch data
                let get = self.store.get(&path).await?;
                let e_tag = get.meta.e_tag.clone();
                let data = get.bytes().await?;
                if let Some(e_tag) = e_tag {
                    let entry = CacheEntry {
                        e_tag,
                        data: data.clone(),
                        refreshed_at: Instant::now(),
                    };
                    self.entries.insert(path.clone(), entry);
                }
                data
            }
        })
    }
}
```

# Conditional Put

The default behaviour when writing data is to upsert any existing object at the given path,
overwriting any previous value. More complex behaviours can be achieved using [`PutMode`], and
can be used to build [Optimistic Concurrency Control] based transactions. This facilitates
building metadata catalogs, such as [Apache Iceberg] or [Delta Lake], directly on top of object
storage, without relying on a separate DBMS.

```
# use object_store::{Error, ObjectStore, ObjectStoreExt, PutMode, UpdateVersion};
# use std::sync::Arc;
# use bytes::Bytes;
# use tokio::io::AsyncWriteExt;
# use object_store::memory::InMemory;
# use object_store::path::Path;
# fn get_object_store() -> Arc<dyn ObjectStore> {
#   Arc::new(InMemory::new())
# }
# fn do_update(b: Bytes) -> Bytes {b}
# async fn conditional_put() {
let store = get_object_store();
let path = Path::from("test");

// Perform a conditional update on path
loop {
    // Perform get request
    let r = store.get(&path).await.unwrap();

    // Save version information fetched
    let version = UpdateVersion {
        e_tag: r.meta.e_tag.clone(),
        version: r.meta.version.clone(),
    };

    // Compute new version of object contents
    let new = do_update(r.bytes().await.unwrap());

    // Attempt to commit transaction
    match store.put_opts(&path, new.into(), PutMode::Update(version).into()).await {
        Ok(_) => break, // Successfully committed
        Err(Error::Precondition { .. }) => continue, // Object has changed, try again
        Err(e) => panic!("{e}")
    }
}
# }
```

[Optimistic Concurrency Control]: https://en.wikipedia.org/wiki/Optimistic_concurrency_control
[Apache Iceberg]: https://iceberg.apache.org/
[Delta Lake]: https://delta.io/

# TLS Certificates

Stores that use HTTPS/TLS (this is true for most cloud stores) can choose the source of their [CA]
certificates. By default the system-bundled certificates are used (see
[`rustls-native-certs`]). The `tls-webpki-roots` feature switch can be used to also bundle Mozilla's
root certificates with the library/application (see [`webpki-roots`]).

[CA]: https://en.wikipedia.org/wiki/Certificate_authority
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs/
[`webpki-roots`]: https://crates.io/crates/webpki-roots

# Customizing HTTP Clients

Many [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations permit customization of the HTTP client via
the [`HttpConnector`] trait and utilities in the [`client`](../modules/object_store.client.md#op-988ade6019fb87cb45c4e7ff) module.
Examples include injecting custom HTTP headers or using an alternate
tokio Runtime I/O requests.

[`HttpConnector`]: client::HttpConnector

Unresolved upstream links (retained, not inferred): `std::io::Seek`, `std::io::Read`.
