# JsonHandler

`buoyant_kernel::JsonHandler`

```rust
trait JsonHandler: AsAny
```

Also reachable as `delta_kernel::JsonHandler`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#jsonhandler) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn parse_json(&self, json_strings: Box<dyn EngineData>, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
fn read_json_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn write_json_file(&self, path: &Url, data: DeltaResultIterator<'_, FilteredEngineData>, overwrite: bool) -> DeltaResult<()>
```

## Implementors (3)

Read one before writing your own.

- `buoyant_kernel::metrics::metered_json::MeteredJsonHandler`
- `buoyant_kernel_engine::json::DefaultJsonHandler`
- `deltalake_core::delta_datafusion::engine::file_formats::DataFusionFileFormatHandler`

## Documentation

Provides JSON handling functionality to Delta Kernel.

Delta Kernel can use this handler to parse JSON strings into Row or read content from JSON
files. Connectors can leverage this trait to provide their best implementation of the JSON
parsing capability to Delta Kernel.
