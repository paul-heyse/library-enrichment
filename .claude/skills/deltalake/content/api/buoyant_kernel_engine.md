# `buoyant_kernel_engine`

Crate `buoyant_kernel_engine` · 4 public items · structured records in [`model/buoyant_kernel_engine.json`](../model/buoyant_kernel_engine.json)

## build_add_file_metadata

`function` · `buoyant_kernel_engine::build_add_file_metadata`

Also reachable as `delta_kernel_default_engine::build_add_file_metadata`

```rust
fn build_add_file_metadata(file_metadata: parquet::DataFileMetadata, write_context: &delta_kernel::transaction::WriteContext) -> delta_kernel::DeltaResult<Box<dyn EngineData>>
```

Converts [`DataFileMetadata`] into Add action [`EngineData`] using the partition values and
table root from the provided [`WriteContext`].

Paths in the returned Add action metadata are stored relative to the table root.

This is the public API for building Add action metadata from file write results. Custom
Arrow-based engines that write parquet files themselves (bypassing
[`DefaultEngine::write_parquet`]) should call this to produce the Add action metadata for
[`Transaction::add_files`].

[`DataFileMetadata`]: parquet::DataFileMetadata
[`Transaction::add_files`]: delta_kernel::transaction::Transaction::add_files

---

## DefaultEngine

`struct` · `buoyant_kernel_engine::DefaultEngine`

Also reachable as `delta_kernel_default_engine::DefaultEngine`

```rust
struct DefaultEngine<E: TaskExecutor>
```

**Implements**: `buoyant_kernel::Engine`

**Derives**: Debug

**Methods** (5)

```rust
fn builder(object_store: Arc<DynObjectStore>) -> DefaultEngineBuilder<DefaultTaskExecutor>
fn default_parquet_handler(&self) -> Arc<DefaultParquetHandler<E>>
fn enter(&self) -> <E as TaskExecutor>::Guard<'_>
fn get_object_store_for_url(&self, _url: &Url) -> Option<Arc<DynObjectStore>>
async fn write_parquet(&self, data: &ArrowEngineData, write_context: &WriteContext) -> DeltaResult<Box<dyn EngineData>>
```

**via `buoyant_kernel::Engine`**

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
fn json_handler(&self) -> Arc<dyn JsonHandler>
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

---

## DefaultEngineBuilder

`struct` · `buoyant_kernel_engine::DefaultEngineBuilder`

Also reachable as `delta_kernel_default_engine::DefaultEngineBuilder`

```rust
struct DefaultEngineBuilder<E>
```

**Derives**: Debug

**Methods** (6)

```rust
fn build(self) -> DefaultEngine<E>
fn build(self) -> DefaultEngine<executor::tokio::TokioBackgroundExecutor>
fn new(object_store: Arc<DynObjectStore>) -> Self
fn with_batch_size(self, batch_size: NonZero<usize>) -> Self
fn with_buffer_size(self, buffer_size: NonZero<usize>) -> Self
fn with_task_executor<F: TaskExecutor>(self, task_executor: Arc<F>) -> DefaultEngineBuilder<Arc<F>>
```

Builder for creating [`DefaultEngine`] instances.

# Example

```no_run
# use std::sync::Arc;
# use buoyant_kernel_engine as delta_kernel_default_engine;
# use delta_kernel_default_engine::DefaultEngineBuilder;
# use delta_kernel_default_engine::executor::tokio::TokioBackgroundExecutor;
# use delta_kernel::object_store::local::LocalFileSystem;
// Build a DefaultEngine with default executor
let engine = DefaultEngineBuilder::new(Arc::new(LocalFileSystem::new()))
    .build();

// Build with a custom executor
let engine = DefaultEngineBuilder::new(Arc::new(LocalFileSystem::new()))
    .with_task_executor(Arc::new(TokioBackgroundExecutor::new()))
    .build();
```

---

## DefaultTaskExecutor

`struct` · `buoyant_kernel_engine::DefaultTaskExecutor`

Also reachable as `delta_kernel_default_engine::DefaultTaskExecutor`

```rust
struct DefaultTaskExecutor
```

Represents the default [`TaskExecutor`]. The executor is created lazily to avoid unnecessary
instantiations.

---
