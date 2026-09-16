# `buoyant_kernel::metrics::metered_json`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.metrics.metered_json.json`](../model/buoyant_kernel.metrics.metered_json.json)

## MeteredJsonHandler

`struct` · `buoyant_kernel::metrics::metered_json::MeteredJsonHandler`

Also reachable as `buoyant_kernel::metrics::MeteredJsonHandler`, `delta_kernel::metrics::metered_json::MeteredJsonHandler`

```rust
struct MeteredJsonHandler
```

**Implements**: `buoyant_kernel::JsonHandler`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn JsonHandler>) -> Self
```

**via `buoyant_kernel::JsonHandler`**

```rust
fn parse_json(&self, json_strings: Box<dyn EngineData>, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
fn read_json_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn write_json_file(&self, path: &url::Url, data: DeltaResultIterator<'_, FilteredEngineData>, overwrite: bool) -> DeltaResult<()>
```

Decorator over an engine-provided `Arc<dyn JsonHandler>` that emits a
`JsonReadCompleted` span on every `read_json_files` call. `parse_json` and
`write_json_file` are pass-through and emit nothing.

---
