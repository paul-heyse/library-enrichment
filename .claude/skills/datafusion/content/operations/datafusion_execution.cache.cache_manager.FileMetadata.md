# `datafusion_execution::cache::cache_manager::FileMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.FileMetadata.json).

<a id="op-2f003a3649d7b4b496dab583"></a>
## FileMetadata

`trait` · `datafusion_execution::cache::cache_manager::FileMetadata` · datafusion-execution 55.1.0

```rust
trait FileMetadata: Any + Send + Sync
```

Source: `src/cache/cache_manager.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Generic file-embedded metadata used with [`FileMetadataCache`](../operations/datafusion_execution.cache.cache_manager.FileMetadataCache.md#op-eae1fe1b1bd732ee9949f01d).

For example, Parquet footers and page metadata can be represented
using this trait.

See [`crate::runtime_env::RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) for more details

<a id="op-55e0145768d5e607ba9f688d"></a>
## as_any

`function` · `datafusion_execution::cache::cache_manager::FileMetadata::as_any` · datafusion-execution 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/cache/cache_manager.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the file metadata as [`Any`] so that it can be downcast to a specific
implementation.

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-7dee6b100cd2e110e4fcd1cf"></a>
## extra_info

`function` · `datafusion_execution::cache::cache_manager::FileMetadata::extra_info` · datafusion-execution 55.1.0

```rust
fn extra_info(&self) -> HashMap<String, String>
```

Source: `src/cache/cache_manager.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns extra information about this entry

<a id="op-7835e61410cb556d7968e07b"></a>
## memory_size

`function` · `datafusion_execution::cache::cache_manager::FileMetadata::memory_size` · datafusion-execution 55.1.0

```rust
fn memory_size(&self) -> usize
```

Source: `src/cache/cache_manager.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the size of the metadata in bytes.
