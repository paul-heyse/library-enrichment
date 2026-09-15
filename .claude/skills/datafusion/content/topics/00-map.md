# Capability map

Fifteen capability axes. Each page maps a capability to the types that provide it, the traits that extend it, the settings that tune it, and the upstream examples that demonstrate it. Everything below the mental model is generated from the pinned model, so the mapping cannot drift from the API.

| Topic | Covers |
|---|---|
| [Sessions and runtime](sessions-and-runtime.md) | SessionContext, SessionConfig, RuntimeEnv, memory pools, spilling, task context |
| [Reading data](reading-data.md) | File formats, listing tables, object stores, schema inference, read options |
| [Writing data](writing-data.md) | COPY, INSERT, DML on table providers, sinks, write options |
| [DataFrame API](dataframe-api.md) | Building and executing plans programmatically without SQL |
| [Expressions](expressions.md) | Expr, expression construction, simplification, schema typing |
| [SQL surface](sql.md) | Dialect, parsing, prepared statements, DDL, information_schema |
| [Logical planning](logical-planning.md) | LogicalPlan, LogicalPlanBuilder, analysis, plan inspection |
| [Optimizer](optimizer.md) | Logical and physical optimizer rules, pushdown, statistics-driven planning |
| [Physical execution](physical-execution.md) | ExecutionPlan, partitioning, streaming, ordering, metrics |
| [User-defined functions](user-defined-functions.md) | Scalar, aggregate and window UDFs, async UDFs, table functions |
| [Custom table providers](custom-table-providers.md) | Implementing TableProvider, pushdown, statistics, DML |
| [Catalogs and schemas](catalogs.md) | CatalogProvider, SchemaProvider, async catalogs, information_schema |
| [Arrow interop](arrow-interop.md) | Arrays, schemas, extension types, compute kernels, type conversion |
| [Parquet](parquet.md) | Reading, writing, pruning, metadata, encryption, statistics |
| [Cross-process interop](interop.md) | Substrait, protobuf serialization, FFI, Flight, Spark compatibility |
