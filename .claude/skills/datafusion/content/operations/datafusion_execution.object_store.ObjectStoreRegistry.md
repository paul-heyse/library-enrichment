# `datafusion_execution::object_store::ObjectStoreRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.object_store.ObjectStoreRegistry.json).

<a id="op-90df51667c296b43700e395a"></a>
## ObjectStoreRegistry

`trait` · `datafusion_execution::object_store::ObjectStoreRegistry` · datafusion-execution 55.1.0

```rust
trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + 'static
```

Source: `src/object_store.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

[`ObjectStoreRegistry`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-90df51667c296b43700e395a) maps a URL to an [`ObjectStore`] instance,
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
[`ObjectStoreRegistry::get_store`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-3ec38485c8c1b7466b6ea670) and one of three things will happen:

- If an [`ObjectStore`] has been registered with [`ObjectStoreRegistry::register_store`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-8b8aab3f96842173349c4b16) with
  `s3://my_bucket`, that [`ObjectStore`] will be returned

- If an AWS S3 object store can be ad-hoc discovered by the url `s3://my_bucket/lineitem/`, this
  object store will be registered with key `s3://my_bucket` and returned.

- Otherwise an error will be returned, indicating that no suitable [`ObjectStore`] could
  be found

This allows for two different use-cases:

1. Systems where object store buckets are explicitly created using DDL, can register these
   buckets using [`ObjectStoreRegistry::register_store`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-8b8aab3f96842173349c4b16)

2. Systems relying on ad-hoc discovery, without corresponding DDL, can create [`ObjectStore`]
   lazily by providing a custom implementation of [`ObjectStoreRegistry`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-90df51667c296b43700e395a)

<!-- is in a different crate so normal rustdoc links don't work -->
[`ListingTableUrl`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTableUrl.html
[`ObjectStore`]: object_store::ObjectStore

<a id="op-274852ecda8a78653c2ac533"></a>
## deregister_store

`function` · `datafusion_execution::object_store::ObjectStoreRegistry::deregister_store` · datafusion-execution 55.1.0

```rust
fn deregister_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
```

Source: `src/object_store.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Deregister the store previously registered with the same key. Returns the
deregistered store if it existed.

<a id="op-3ec38485c8c1b7466b6ea670"></a>
## get_store

`function` · `datafusion_execution::object_store::ObjectStoreRegistry::get_store` · datafusion-execution 55.1.0

```rust
fn get_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
```

Source: `src/object_store.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Get a suitable store for the provided URL. For example:

- URL with scheme `file:///` or no scheme will return the default LocalFS store
- URL with scheme `s3://bucket/` will return the S3 store
- URL with scheme `hdfs://hostname:port/` will return the hdfs store

If no [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) found for the `url`, ad-hoc discovery may be executed depending on
the `url` and [`ObjectStoreRegistry`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-90df51667c296b43700e395a) implementation. An [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) may be lazily
created and registered.

<a id="op-8b8aab3f96842173349c4b16"></a>
## register_store

`function` · `datafusion_execution::object_store::ObjectStoreRegistry::register_store` · datafusion-execution 55.1.0

```rust
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

Source: `src/object_store.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

If a store with the same key existed before, it is replaced and returned
