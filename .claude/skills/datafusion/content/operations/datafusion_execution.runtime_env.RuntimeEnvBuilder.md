# `datafusion_execution::runtime_env::RuntimeEnvBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.runtime_env.RuntimeEnvBuilder.json).

<a id="op-428a1f27f49cdacb242a4132"></a>
## RuntimeEnvBuilder

`struct` · `datafusion_execution::runtime_env::RuntimeEnvBuilder` · datafusion-execution 55.1.0

```rust
struct RuntimeEnvBuilder
```

Source: `src/runtime_env.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Execution runtime configuration builder.

See example on [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67)

<a id="op-29828f8e1136ce639b8b1e0c"></a>
## build

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::build` · datafusion-execution 55.1.0

```rust
fn build(self) -> Result<RuntimeEnv>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Build a RuntimeEnv

<a id="op-2dec76d25f15e2a33b03c06d"></a>
## build_arc

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::build_arc` · datafusion-execution 55.1.0

```rust
fn build_arc(self) -> Result<Arc<RuntimeEnv>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:514`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Convenience method to create a new `Arc<RuntimeEnv>`

<a id="op-8469c2f3dfe553e954b8dcb6"></a>
## cache_manager

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::cache_manager` · datafusion-execution 55.1.0

```rust
cache_manager: cache::cache_manager::CacheManagerConfig
```

Source: `src/runtime_env.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

CacheManager to manage cache data

<a id="op-b0be2dc1c1d577ff003699c3"></a>
## clone

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> RuntimeEnvBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 10], "end": [357, 15], "filename": "src/runtime_env.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/runtime_env.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df42830433f02ae97bf3cfb7"></a>
## default

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::default` · datafusion-execution 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [380, 2], "filename": "src/runtime_env.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/runtime_env.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7f1658d9158c78f78ffc664"></a>
## disk_manager

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::disk_manager` · datafusion-execution 55.1.0

```rust
disk_manager: Option<std::sync::Arc<disk_manager::DiskManager>>
```

Source: `src/runtime_env.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

DiskManager to manage temporary disk file usage

<a id="op-92bdd73f916e50a797d8a9cc"></a>
## disk_manager_builder

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::disk_manager_builder` · datafusion-execution 55.1.0

```rust
disk_manager_builder: Option<disk_manager::DiskManagerBuilder>
```

Source: `src/runtime_env.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

DiskManager builder to manager temporary disk file usage

<a id="op-b42f30d34a1e67e3e21bf273"></a>
## entries

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::entries` · datafusion-execution 55.1.0

```rust
fn entries(&self) -> Vec<ConfigEntry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a list of all available runtime configurations with their current values and descriptions

<a id="op-641700fde01dc372bb672b04"></a>
## from_runtime_env

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::from_runtime_env` · datafusion-execution 55.1.0

```rust
fn from_runtime_env(runtime_env: &RuntimeEnv) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new RuntimeEnvBuilder from an existing RuntimeEnv

<a id="op-4a7c7a83f3efd9c79a6ae757"></a>
## generate_config_markdown

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::generate_config_markdown` · datafusion-execution 55.1.0

```rust
fn generate_config_markdown() -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Generate documentation that can be included in the user guide

<a id="op-69ea324b6ecb00f4cc11945c"></a>
## memory_pool

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::memory_pool` · datafusion-execution 55.1.0

```rust
memory_pool: Option<std::sync::Arc<dyn MemoryPool>>
```

Source: `src/runtime_env.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

[`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) from which to allocate memory

Defaults to using an [`UnboundedMemoryPool`](../operations/datafusion_execution.memory_pool.pool.UnboundedMemoryPool.md#op-bc210b8fe79074b67e0fb694) if `None`

<a id="op-d3fcd07d15c30a19f951af92"></a>
## new

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::new` · datafusion-execution 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

New with default values

<a id="op-70d76df7010ee45ef19d34bd"></a>
## object_store_registry

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::object_store_registry` · datafusion-execution 55.1.0

```rust
object_store_registry: std::sync::Arc<dyn ObjectStoreRegistry>
```

Source: `src/runtime_env.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

ObjectStoreRegistry to get object store based on url

<a id="op-69659820c60434e262f22288"></a>
## parquet_encryption_factory_registry

`struct_field` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::parquet_encryption_factory_registry` · datafusion-execution 55.1.0

```rust
parquet_encryption_factory_registry: std::sync::Arc<parquet_encryption::EncryptionFactoryRegistry>
```

Source: `src/runtime_env.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Parquet encryption factory registry

<a id="op-425e5f81f67fff7861a57f9c"></a>
## with_cache_manager

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_cache_manager` · datafusion-execution 55.1.0

```rust
fn with_cache_manager(self, cache_manager: CacheManagerConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Customize cache policy

<a id="op-accca36081c033f916b43347"></a>
## with_disk_manager_builder

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_disk_manager_builder` · datafusion-execution 55.1.0

```rust
fn with_disk_manager_builder(self, disk_manager: DiskManagerBuilder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Customize the disk manager builder

<a id="op-ca87cd78a0d2a7de408b0c0a"></a>
## with_file_statistics_cache_limit

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_file_statistics_cache_limit` · datafusion-execution 55.1.0

```rust
fn with_file_statistics_cache_limit(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf370ac9fad949e089937af7"></a>
## with_max_spill_merge_fan_in

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_max_spill_merge_fan_in` · datafusion-execution 55.1.0

```rust
fn with_max_spill_merge_fan_in(self, fan_in: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Limit the number of spill files opened by one external merge pass.

A value of 0 means unlimited.

<a id="op-4cace5fc7e0f4c22fc54fd67"></a>
## with_max_temp_directory_size

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_max_temp_directory_size` · datafusion-execution 55.1.0

```rust
fn with_max_temp_directory_size(self, size: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Specify a limit on the size of the temporary file directory in bytes

<a id="op-67b941dfc4573dd8aea8cf06"></a>
## with_memory_limit

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_memory_limit` · datafusion-execution 55.1.0

```rust
fn with_memory_limit(self, max_memory: usize, memory_fraction: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Specify the total memory to use while running the DataFusion
plan to `max_memory * memory_fraction` in bytes.

This defaults to using [`GreedyMemoryPool`](../operations/datafusion_execution.memory_pool.pool.GreedyMemoryPool.md#op-2f1ce34e98ac037b2abc0c26) wrapped in the
[`TrackConsumersPool`](../operations/datafusion_execution.memory_pool.pool.TrackConsumersPool.md#op-68ccf7cd2608b07636f02ae9) with a maximum of 5 consumers.

Note DataFusion does not yet respect this limit in all cases.

<a id="op-f022794b3809a36b5f28527c"></a>
## with_memory_pool

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_memory_pool` · datafusion-execution 55.1.0

```rust
fn with_memory_pool(self, memory_pool: Arc<dyn MemoryPool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Customize memory policy

<a id="op-f192219cfc00293ae9f0c6ab"></a>
## with_metadata_cache_limit

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_metadata_cache_limit` · datafusion-execution 55.1.0

```rust
fn with_metadata_cache_limit(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:461`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Specify the limit of the file-embedded metadata cache, in bytes.

<a id="op-afc2ea0857c1b7b17593b11d"></a>
## with_object_list_cache_limit

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_object_list_cache_limit` · datafusion-execution 55.1.0

```rust
fn with_object_list_cache_limit(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Specifies the memory limit for the object list cache, in bytes.

<a id="op-43b6bc290dd1434dda21c87b"></a>
## with_object_list_cache_ttl

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_object_list_cache_ttl` · datafusion-execution 55.1.0

```rust
fn with_object_list_cache_ttl(self, ttl: Option<Duration>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Specifies the duration entries in the object list cache will be considered valid.

<a id="op-eb30f3263ceb83b98e6be23e"></a>
## with_object_store_registry

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_object_store_registry` · datafusion-execution 55.1.0

```rust
fn with_object_store_registry(self, object_store_registry: Arc<dyn ObjectStoreRegistry>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Customize object store registry

<a id="op-b2c9d78aaaec99882390aa89"></a>
## with_temp_file_path

`function` · `datafusion_execution::runtime_env::RuntimeEnvBuilder::with_temp_file_path` · datafusion-execution 55.1.0

```rust
fn with_temp_file_path(self, path: impl Into<PathBuf>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::runtime_env::RuntimeEnvBuilder", "path": "RuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [586, 2], "filename": "src/runtime_env.rs"}, "trait": null, "trait_path": null}`

Source: `src/runtime_env.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Use the specified path to create any needed temporary files
