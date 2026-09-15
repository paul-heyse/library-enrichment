# `datafusion_execution::object_store`

Crate `datafusion-execution` · 3 public items · structured records in [`model/datafusion_execution.object_store.json`](../model/datafusion_execution.object_store.json)

## DefaultObjectStoreRegistry

`struct` · `datafusion_execution::object_store::DefaultObjectStoreRegistry`

Also reachable as `datafusion::datasource::object_store::DefaultObjectStoreRegistry`

```rust
struct DefaultObjectStoreRegistry
```

**Implements**: `datafusion_execution::object_store::ObjectStoreRegistry`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_execution::object_store::ObjectStoreRegistry`**

```rust
fn deregister_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
fn get_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

The default [`ObjectStoreRegistry`]

---

## ObjectStoreUrl

`struct` · `datafusion_execution::object_store::ObjectStoreUrl`

Also reachable as `datafusion::datasource::object_store::ObjectStoreUrl`

```rust
struct ObjectStoreUrl
```

**Implements**: `core::convert::AsRef`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn as_str(&self) -> &str
fn local_filesystem() -> Self
fn parse(s: impl AsRef<str>) -> Result<Self>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
fn as_ref(&self) -> &Url
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A parsed URL identifying a particular [`ObjectStore`] instance

For example:
* `file://` for local file system
* `s3://bucket` for AWS S3 bucket
* `oss://bucket` for Aliyun OSS bucket

---

## ObjectStoreRegistry

`trait` · `datafusion_execution::object_store::ObjectStoreRegistry`

Also reachable as `datafusion::datasource::object_store::ObjectStoreRegistry`

```rust
trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + 'static
```

**Implementors** (1)

- `datafusion_execution::object_store::DefaultObjectStoreRegistry`

**Methods** (3)

```rust
fn deregister_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
fn get_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

[`ObjectStoreRegistry`] maps a URL to an [`ObjectStore`] instance,
and allows DataFusion to read from different [`ObjectStore`]
instances. For example DataFusion might be configured so that

1. `s3://my_bucket/lineitem/` mapped to the `/lineitem` path on an
   AWS S3 object store bound to `my_bucket`

2. `s3://my_other_bucket/lineitem/` mapped to the (same)
   `/lineitem` path on a *different* AWS S3 object store bound to
   `my_other_bucket`

When given a [`ListingTableUrl`], DataFusion tries to find an
appropriate [`ObjectStore`]. For example

```sql
create external table unicorns stored as parquet location 's3://my_bucket/lineitem/';
```

In this particular case, the url `s3://my_bucket/lineitem/` will be provided to
[`ObjectStoreRegistry::get_store`] and one of three things will happen:

- If an [`ObjectStore`] has been registered with [`ObjectStoreRegistry::register_store`] with
  `s3://my_bucket`, that [`ObjectStore`] will be returned

- If an AWS S3 object store can be ad-hoc discovered by the url `s3://my_bucket/lineitem/`, this
  object store will be registered with key `s3://my_bucket` and returned.

- Otherwise an error will be returned, indicating that no suitable [`ObjectStore`] could
  be found

This allows for two different use-cases:

1. Systems where object store buckets are explicitly created using DDL, can register these
   buckets using [`ObjectStoreRegistry::register_store`]

2. Systems relying on ad-hoc discovery, without corresponding DDL, can create [`ObjectStore`]
   lazily by providing a custom implementation of [`ObjectStoreRegistry`]

<!-- is in a different crate so normal rustdoc links don't work -->
[`ListingTableUrl`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTableUrl.html
[`ObjectStore`]: object_store::ObjectStore

---
