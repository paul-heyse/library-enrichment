# `datafusion_execution::runtime_env`

Crate `datafusion-execution` · 2 public items · structured records in [`model/datafusion_execution.runtime_env.json`](../model/datafusion_execution.runtime_env.json)

## RuntimeEnv

`struct` · `datafusion_execution::runtime_env::RuntimeEnv`

```rust
struct RuntimeEnv
```

**Fields**: `memory_pool`, `disk_manager`, `cache_manager`, `object_store_registry`, `parquet_encryption_factory_registry`

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn config_entries(&self) -> Vec<ConfigEntry>
fn deregister_object_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
fn object_store(&self, url: impl AsRef<Url>) -> Result<Arc<dyn ObjectStore>>
fn parquet_encryption_factory(&self, id: &str) -> Result<Arc<dyn EncryptionFactory>>
fn register_object_store(&self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
fn register_parquet_encryption_factory(&self, id: &str, encryption_factory: Arc<dyn EncryptionFactory>) -> Option<Arc<dyn EncryptionFactory>>
fn spilling_progress(&self) -> SpillingProgress
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.runtime_env.RuntimeEnv.md).


Execution runtime environment that manages system resources such
as memory, disk, cache and storage.

A [`RuntimeEnv`] can be created using [`RuntimeEnvBuilder`] and has the
following resource management functionality:

* [`MemoryPool`]: Manage memory
* [`DiskManager`]: Manage temporary files on local disk
* [`CacheManager`]: Manage temporary cache data during the session lifetime
* [`ObjectStoreRegistry`]: Manage mapping URLs to object store instances

# Example: Create default `RuntimeEnv`
```
# use datafusion_execution::runtime_env::RuntimeEnv;
let runtime_env = RuntimeEnv::default();
```

# Example: Create a `RuntimeEnv` from [`RuntimeEnvBuilder`] with a new memory pool
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

---

## RuntimeEnvBuilder

`struct` · `datafusion_execution::runtime_env::RuntimeEnvBuilder`

```rust
struct RuntimeEnvBuilder
```

**Fields**: `disk_manager`, `disk_manager_builder`, `memory_pool`, `cache_manager`, `object_store_registry`, `parquet_encryption_factory_registry`

**Derives**: Clone, Default

**Methods** (18)

```rust
fn build(self) -> Result<RuntimeEnv>
fn build_arc(self) -> Result<Arc<RuntimeEnv>>
fn entries(&self) -> Vec<ConfigEntry>
fn from_runtime_env(runtime_env: &RuntimeEnv) -> Self
fn generate_config_markdown() -> String
fn new() -> Self
fn with_cache_manager(self, cache_manager: CacheManagerConfig) -> Self
fn with_disk_manager_builder(self, disk_manager: DiskManagerBuilder) -> Self
fn with_file_statistics_cache_limit(self, limit: usize) -> Self
fn with_max_spill_merge_fan_in(self, fan_in: usize) -> Self
fn with_max_temp_directory_size(self, size: u64) -> Self
fn with_memory_limit(self, max_memory: usize, memory_fraction: f64) -> Self
fn with_memory_pool(self, memory_pool: Arc<dyn MemoryPool>) -> Self
fn with_metadata_cache_limit(self, limit: usize) -> Self
fn with_object_list_cache_limit(self, limit: usize) -> Self
fn with_object_list_cache_ttl(self, ttl: Option<Duration>) -> Self
fn with_object_store_registry(self, object_store_registry: Arc<dyn ObjectStoreRegistry>) -> Self
fn with_temp_file_path(self, path: impl Into<PathBuf>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.runtime_env.RuntimeEnvBuilder.md).


Execution runtime configuration builder.

See example on [`RuntimeEnv`]

---
