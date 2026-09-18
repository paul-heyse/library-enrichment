# `datafusion_execution::runtime_env::RuntimeEnv`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.runtime_env.RuntimeEnv.json).

<a id="op-c598f4df4cf51824ae4b9a67"></a>
## RuntimeEnv

`struct` · `datafusion_execution::runtime_env::RuntimeEnv` · datafusion-execution 55.1.0

```rust
struct RuntimeEnv
```

Source: `src/runtime_env.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Execution runtime environment that manages system resources such
as memory, disk, cache and storage.

A [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) can be created using [`RuntimeEnvBuilder`](../operations/datafusion_execution.runtime_env.RuntimeEnvBuilder.md#op-428a1f27f49cdacb242a4132) and has the
following resource management functionality:

* [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af): Manage memory
* [`DiskManager`](../operations/datafusion_execution.disk_manager.DiskManager.md#op-e778066e9d4969684552adc1): Manage temporary files on local disk
* [`CacheManager`](../operations/datafusion_execution.cache.cache_manager.CacheManager.md#op-b6af2b372497895e325851ad): Manage temporary cache data during the session lifetime
* [`ObjectStoreRegistry`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-90df51667c296b43700e395a): Manage mapping URLs to object store instances

# Example: Create default `RuntimeEnv`
```
# use datafusion_execution::runtime_env::RuntimeEnv;
let runtime_env = RuntimeEnv::default();
```

# Example: Create a `RuntimeEnv` from [`RuntimeEnvBuilder`](../operations/datafusion_execution.runtime_env.RuntimeEnvBuilder.md#op-428a1f27f49cdacb242a4132) with a new memory pool
```
# use std::sync::Arc;
# use datafusion_execution::memory_pool::GreedyMemoryPool;
# use datafusion_execution::runtime_env::{RuntimeEnv, RuntimeEnvBuilder};
// restrict to using at most 100MB of memory
let pool_size = 100 * 1024 * 1024;
let runtime_env = RuntimeEnvBuilder::new()
    .with_memory_pool(Arc::new(GreedyMemoryPool::new(pool_size)))
    .build()
    .unwrap();
```

<a id="op-34c39ccd2bd0531705ce9a30"></a>
## cache_manager

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnv::cache_manager` · datafusion-execution 55.1.0

```rust
cache_manager: std::sync::Arc<cache::cache_manager::CacheManager>
```

Source: `src/runtime_env.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Manage temporary cache during query execution

<a id="op-c79ca701e9c2c26e27f450ae"></a>
## clone

`function` · `datafusion_execution::runtime_env::RuntimeEnv::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> RuntimeEnv
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/runtime_env.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/runtime_env.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-975ab82ff7964e24cc0a67fd"></a>
## config_entries

`function` · `datafusion_execution::runtime_env::RuntimeEnv::config_entries` · datafusion-execution 55.1.0

```rust
fn config_entries(&self) -> Vec<ConfigEntry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [346, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the current runtime configuration entries

<a id="op-3fbd07424ce903759fdc14c5"></a>
## default

`function` · `datafusion_execution::runtime_env::RuntimeEnv::default` · datafusion-execution 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [352, 2], "filename": "src/runtime_env.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/runtime_env.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c612e2c74847558db08b912"></a>
## deregister_object_store

`function` · `datafusion_execution::runtime_env::RuntimeEnv::deregister_object_store` · datafusion-execution 55.1.0

```rust
fn deregister_object_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [346, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Deregisters a custom `ObjectStore` previously registered for a specific url.
See [`ObjectStoreRegistry::deregister_store`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-274852ecda8a78653c2ac533) for more details.

<a id="op-103b30a49eb9641dad45e285"></a>
## disk_manager

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnv::disk_manager` · datafusion-execution 55.1.0

```rust
disk_manager: std::sync::Arc<disk_manager::DiskManager>
```

Source: `src/runtime_env.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Manage temporary files during query execution

<a id="op-a4eb8809007ac58ab7e4cf1f"></a>
## fmt

`function` · `datafusion_execution::runtime_env::RuntimeEnv::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [91, 2], "filename": "src/runtime_env.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/runtime_env.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc6346f53c820f55e9f60fe"></a>
## memory_pool

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnv::memory_pool` · datafusion-execution 55.1.0

```rust
memory_pool: std::sync::Arc<dyn MemoryPool>
```

Source: `src/runtime_env.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Runtime memory management

<a id="op-f0aff33afa21c8a4a773ce13"></a>
## object_store

`function` · `datafusion_execution::runtime_env::RuntimeEnv::object_store` · datafusion-execution 55.1.0

```rust
fn object_store(&self, url: impl AsRef<Url>) -> Result<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [346, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Retrieves a `ObjectStore` instance for a url by consulting the
registry. See [`ObjectStoreRegistry::get_store`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-3ec38485c8c1b7466b6ea670) for more
details.

<a id="op-79338d534a5ec7b59da489a5"></a>
## object_store_registry

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnv::object_store_registry` · datafusion-execution 55.1.0

```rust
object_store_registry: std::sync::Arc<dyn ObjectStoreRegistry>
```

Source: `src/runtime_env.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Object Store Registry

<a id="op-e0c9a9daafcfab66447f5bed"></a>
## parquet_encryption_factory

`function` · `datafusion_execution::runtime_env::RuntimeEnv::parquet_encryption_factory` · datafusion-execution 55.1.0

```rust
fn parquet_encryption_factory(&self, id: &str) -> Result<Arc<dyn EncryptionFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [346, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Retrieve an [`EncryptionFactory`](../operations/datafusion_execution.parquet_encryption.EncryptionFactory.md#op-7306e44c25717f033e9116e5) by its identifier

<a id="op-76864656f1a6d6f4fe231d49"></a>
## parquet_encryption_factory_registry

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnv::parquet_encryption_factory_registry` · datafusion-execution 55.1.0

```rust
parquet_encryption_factory_registry: std::sync::Arc<parquet_encryption::EncryptionFactoryRegistry>
```

Source: `src/runtime_env.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Parquet encryption factory registry

<a id="op-e3bcab5c9d369a54456bd1a7"></a>
## register_object_store

`function` · `datafusion_execution::runtime_env::RuntimeEnv::register_object_store` · datafusion-execution 55.1.0

```rust
fn register_object_store(&self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [346, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Registers a custom `ObjectStore` to be used with a specific url.
This allows DataFusion to create external tables from urls that do not have
built in support such as `hdfs://namenode:port/...`.

Returns the [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) previously registered for this
scheme, if any.

See [`ObjectStoreRegistry`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-90df51667c296b43700e395a) for more details

# Example: Register local file system object store
```
# use std::sync::Arc;
# use url::Url;
# use datafusion_execution::runtime_env::RuntimeEnv;
# let runtime_env = RuntimeEnv::default();
let url = Url::try_from("file://").unwrap();
let object_store = object_store::local::LocalFileSystem::new();
// register the object store with the runtime environment
runtime_env.register_object_store(&url, Arc::new(object_store));
```

# Example: Register remote URL object store like [Github](https://github.com)
```
# use std::sync::Arc;
# use url::Url;
# use datafusion_execution::runtime_env::RuntimeEnv;
# let runtime_env = RuntimeEnv::default();
# // use local store for example as http feature is not enabled
# let http_store = object_store::local::LocalFileSystem::new();
// create a new object store via object_store::http::HttpBuilder;
let base_url = Url::parse("https://github.com").unwrap();
// (note this example can't depend on the http feature)
// let http_store = HttpBuilder::new()
//    .with_url(base_url.clone())
//    .build()
//    .unwrap();
// register the object store with the runtime environment
runtime_env.register_object_store(&base_url, Arc::new(http_store));
```

<a id="op-9ba3340a715facdc9efbad69"></a>
## register_parquet_encryption_factory

`function` · `datafusion_execution::runtime_env::RuntimeEnv::register_parquet_encryption_factory` · datafusion-execution 55.1.0

```rust
fn register_parquet_encryption_factory(&self, id: &str, encryption_factory: Arc<dyn EncryptionFactory>) -> Option<Arc<dyn EncryptionFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [346, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Register an [`EncryptionFactory`](../operations/datafusion_execution.parquet_encryption.EncryptionFactory.md#op-7306e44c25717f033e9116e5) with an associated identifier that can be later
used to configure encryption when reading or writing Parquet.
If an encryption factory with the same identifier was already registered, it is replaced and returned.

<a id="op-4873c4cb69ae16c7068e50fc"></a>
## spilling_progress

`function` · `datafusion_execution::runtime_env::RuntimeEnv::spilling_progress` · datafusion-execution 55.1.0

```rust
fn spilling_progress(&self) -> SpillingProgress
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnv", "path": "RuntimeEnv"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [346, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the current spilling progress
