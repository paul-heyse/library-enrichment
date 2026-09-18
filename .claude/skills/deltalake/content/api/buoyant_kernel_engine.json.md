# `buoyant_kernel_engine::json`

Crate `buoyant_kernel_engine` · 1 public items · structured records in [`model/buoyant_kernel_engine.json.json`](../model/buoyant_kernel_engine.json.json)

## DefaultJsonHandler

`struct` · `buoyant_kernel_engine::json::DefaultJsonHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel_engine.json.DefaultJsonHandler.md)

Also reachable as `delta_kernel_default_engine::json::DefaultJsonHandler`

```rust
struct DefaultJsonHandler<E: TaskExecutor>
```

**Implements**: `buoyant_kernel::JsonHandler`

**Derives**: Debug

**Methods** (3)

```rust
fn new(store: Arc<DynObjectStore>, task_executor: Arc<E>) -> Self
fn with_batch_size(self, batch_size: NonZero<usize>) -> Self
fn with_buffer_size(self, buffer_size: NonZero<usize>) -> Self
```

**via `buoyant_kernel::JsonHandler`**

```rust
fn parse_json(&self, json_strings: Box<dyn EngineData>, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
fn read_json_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn write_json_file(&self, path: &Url, data: DeltaResultIterator<'_, FilteredEngineData>, overwrite: bool) -> DeltaResult<()>
```

---
