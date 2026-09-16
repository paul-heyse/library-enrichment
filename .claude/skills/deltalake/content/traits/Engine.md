# Engine

`buoyant_kernel::Engine`

```rust
trait Engine: AsAny
```

Also reachable as `delta_kernel::Engine`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#engine) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
fn json_handler(&self) -> Arc<dyn JsonHandler>
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

## Implementors (3)

Read one before writing your own.

- `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine`
- `buoyant_kernel_engine::DefaultEngine`
- `deltalake_core::delta_datafusion::engine::DataFusionEngine`

## Documentation

The `Engine` trait encapsulates all the functionality an engine or connector needs to provide
to the Delta Kernel in order to read the Delta table.

Engines/Connectors are expected to pass an implementation of this trait when reading a Delta
table.
