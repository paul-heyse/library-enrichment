# The kernel boundary

This library does not implement the Delta protocol itself any more. Schema, actions, log replay, scans and checkpointing come from the kernel crate, which is generic over an `Engine` supplying IO, JSON, Parquet and expression evaluation. That is why kernel types appear throughout the public API: `Schema` is the kernel's `StructType`, and the types are re-exported, not wrapped.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `buoyant_kernel::snapshot::Snapshot` | struct | 36 | [prose](../api/buoyant_kernel.snapshot.md#snapshot) | [records](../model/buoyant_kernel.snapshot.json) |
| `buoyant_kernel::scan::Scan` | struct | 10 | [prose](../api/buoyant_kernel.scan.md#scan) | [records](../model/buoyant_kernel.scan.json) |
| `buoyant_kernel::scan::ScanBuilder` | struct | 10 | [prose](../api/buoyant_kernel.scan.md#scanbuilder) | [records](../model/buoyant_kernel.scan.json) |
| `buoyant_kernel::engine::arrow_data::ArrowEngineData` | struct | 10 | [prose](../api/buoyant_kernel.engine.arrow_data.md#arrowenginedata) | [records](../model/buoyant_kernel.engine.arrow_data.json) |
| `buoyant_kernel_engine::DefaultEngine` | struct | 10 | [prose](../api/buoyant_kernel_engine.md#defaultengine) | [records](../model/buoyant_kernel_engine.json) |
| `buoyant_kernel::table_configuration::TableConfiguration` | struct | 18 | [prose](../api/buoyant_kernel.table_configuration.md#tableconfiguration) | [records](../model/buoyant_kernel.table_configuration.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `buoyant_kernel::Engine` | 4 | 0 | 3 | [Engine](../traits/Engine.md) |
| `buoyant_kernel::engine_data::EngineData` | 5 | 1 | 1 | [EngineData](../traits/EngineData.md) |
| `buoyant_kernel::JsonHandler` | 3 | 0 | 3 | [JsonHandler](../traits/JsonHandler.md) |
| `buoyant_kernel::ParquetHandler` | 3 | 0 | 3 | [ParquetHandler](../traits/ParquetHandler.md) |
| `buoyant_kernel::StorageHandler` | 6 | 0 | 3 | [StorageHandler](../traits/StorageHandler.md) |
| `buoyant_kernel::EvaluationHandler` | 4 | 0 | 1 | [EvaluationHandler](../traits/EvaluationHandler.md) |
| `buoyant_kernel::ExpressionEvaluator` | 1 | 0 | 1 | [ExpressionEvaluator](../traits/ExpressionEvaluator.md) |
| `buoyant_kernel::PredicateEvaluator` | 1 | 0 | 1 | [PredicateEvaluator](../traits/PredicateEvaluator.md) |
| `buoyant_kernel::engine_data::RowVisitor` | 2 | 1 | 22 | [RowVisitor](../traits/RowVisitor.md) |
| `buoyant_kernel::IntoEngineData` | 1 | 0 | 5 | [IntoEngineData](../traits/IntoEngineData.md) |
| `buoyant_kernel_engine::executor::TaskExecutor` | 4 | 0 | 2 | [TaskExecutor](../traits/TaskExecutor.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Implement `Engine` only to run Delta on a runtime the default engine does not cover; it is a large surface.
- Read kernel types under their canonical crate, and remember the Cargo rename: the crate is published as `buoyant_kernel` and used as `delta_kernel`.

## Anti-patterns

- Reimplementing log replay or checkpoint writing outside the kernel.
- Assuming a kernel type is a delta-rs type because it is re-exported through this crate.

## Agent checklist

- Is the work already done by the kernel?
- Are kernel types resolved to their canonical crate before being attributed?
