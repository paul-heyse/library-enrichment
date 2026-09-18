# Representation routes

| Representation | Reviewed decisions |
|---|---|
| Array | [Choose conversion failure and output-schema policies explicitly](../capabilities/arrow.cast.md), [Reuse selection preparation when the workload warrants it](../capabilities/arrow.filter-reuse.md), [Choose a row or column selection representation](../capabilities/arrow.select.md) |
| ArrayRef | [Choose conversion failure and output-schema policies explicitly](../capabilities/arrow.cast.md), [Reuse selection preparation when the workload warrants it](../capabilities/arrow.filter-reuse.md), [Encode composite keys for local comparisons](../capabilities/arrow.row-keys.md), [Distinguish batch validity, relational schema and metadata propagation](../capabilities/arrow.schema.md), [Choose a row or column selection representation](../capabilities/arrow.select.md), [Reuse aggregate/window functions and characterize state before extensions](../capabilities/df.aggregate-window.md) |
| ArrowError | [Choose conversion failure and output-schema policies explicitly](../capabilities/arrow.cast.md) |
| BooleanArray | [Reuse selection preparation when the workload warrants it](../capabilities/arrow.filter-reuse.md), [Choose a row or column selection representation](../capabilities/arrow.select.md) |
| CastOptions | [Choose conversion failure and output-schema policies explicitly](../capabilities/arrow.cast.md) |
| ColumnarValue | [Use coalesce through its planning contract](../capabilities/df.coalesce.md), [Separate expression construction, coercion and evaluation](../capabilities/df.expressions.md), [Discover built-ins in the actual session before writing a UDF](../capabilities/df.functions.md) |
| DFSchema | [Distinguish batch validity, relational schema and metadata propagation](../capabilities/arrow.schema.md), [Separate expression construction, coercion and evaluation](../capabilities/df.expressions.md) |
| DataFrame | [Choose stream, materialization, cache and resource boundaries](../capabilities/df.consume.md), [Choose relational cardinality, null-key and ordering semantics](../capabilities/df.relations.md), [Reuse a provider before implementing a new source](../capabilities/df.source.md), [Separate source reuse, plan reuse, materialization and writes](../capabilities/df.storage-reuse.md) |
| DataType | [Choose conversion failure and output-schema policies explicitly](../capabilities/arrow.cast.md), [Use coalesce through its planning contract](../capabilities/df.coalesce.md) |
| ExecutionPlan | [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md), [Advertise only sound filter, projection and limit pushdown](../capabilities/df.pushdown.md), [Reuse a provider before implementing a new source](../capabilities/df.source.md), [Separate source reuse, plan reuse, materialization and writes](../capabilities/df.storage-reuse.md) |
| Expr | [Use coalesce through its planning contract](../capabilities/df.coalesce.md), [Separate expression construction, coercion and evaluation](../capabilities/df.expressions.md), [Discover built-ins in the actual session before writing a UDF](../capabilities/df.functions.md), [Advertise only sound filter, projection and limit pushdown](../capabilities/df.pushdown.md), [Choose relational cardinality, null-key and ordering semantics](../capabilities/df.relations.md) |
| FFI handles | [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md) |
| FFI provider | [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md) |
| Field | [Distinguish batch validity, relational schema and metadata propagation](../capabilities/arrow.schema.md), [Use coalesce through its planning contract](../capabilities/df.coalesce.md) |
| FilterPredicate | [Reuse selection preparation when the workload warrants it](../capabilities/arrow.filter-reuse.md) |
| LogicalPlan | [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md), [Choose relational cardinality, null-key and ordering semantics](../capabilities/df.relations.md), [Reuse a provider before implementing a new source](../capabilities/df.source.md) |
| ObjectStore | [Reuse a provider before implementing a new source](../capabilities/df.source.md), [Separate source reuse, plan reuse, materialization and writes](../capabilities/df.storage-reuse.md) |
| OwnedRow | [Encode composite keys for local comparisons](../capabilities/arrow.row-keys.md) |
| Parquet bytes | [Compose row-group pruning, row selection and decode filters](../capabilities/parquet.selection.md) |
| PhysicalExpr | [Distinguish batch validity, relational schema and metadata propagation](../capabilities/arrow.schema.md), [Separate expression construction, coercion and evaluation](../capabilities/df.expressions.md) |
| RecordBatch | [Reuse selection preparation when the workload warrants it](../capabilities/arrow.filter-reuse.md), [Distinguish batch validity, relational schema and metadata propagation](../capabilities/arrow.schema.md), [Choose a row or column selection representation](../capabilities/arrow.select.md), [Choose stream, materialization, cache and resource boundaries](../capabilities/df.consume.md), [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md), [Reuse a provider before implementing a new source](../capabilities/df.source.md), [Compose row-group pruning, row selection and decode filters](../capabilities/parquet.selection.md) |
| RowFilter | [Compose row-group pruning, row selection and decode filters](../capabilities/parquet.selection.md) |
| RowSelection | [Compose row-group pruning, row selection and decode filters](../capabilities/parquet.selection.md) |
| Rows | [Encode composite keys for local comparisons](../capabilities/arrow.row-keys.md) |
| RuntimeEnv | [Choose stream, materialization, cache and resource boundaries](../capabilities/df.consume.md) |
| ScalarUDF | [Separate expression construction, coercion and evaluation](../capabilities/df.expressions.md) |
| ScalarValue | [Reuse aggregate/window functions and characterize state before extensions](../capabilities/df.aggregate-window.md) |
| ScanResult | [Advertise only sound filter, projection and limit pushdown](../capabilities/df.pushdown.md) |
| Schema | [Distinguish batch validity, relational schema and metadata propagation](../capabilities/arrow.schema.md) |
| SendableRecordBatchStream | [Choose stream, materialization, cache and resource boundaries](../capabilities/df.consume.md) |
| Session | [Advertise only sound filter, projection and limit pushdown](../capabilities/df.pushdown.md) |
| SessionState | [Discover built-ins in the actual session before writing a UDF](../capabilities/df.functions.md) |
| SortField | [Encode composite keys for local comparisons](../capabilities/arrow.row-keys.md) |
| SortOptions | [Encode composite keys for local comparisons](../capabilities/arrow.row-keys.md) |
| TableProvider | [Discover built-ins in the actual session before writing a UDF](../capabilities/df.functions.md), [Reuse a provider before implementing a new source](../capabilities/df.source.md), [Separate source reuse, plan reuse, materialization and writes](../capabilities/df.storage-reuse.md) |
| bytes | [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md) |
| files | [Reuse a provider before implementing a new source](../capabilities/df.source.md), [Separate source reuse, plan reuse, materialization and writes](../capabilities/df.storage-reuse.md) |
| group indices | [Reuse aggregate/window functions and characterize state before extensions](../capabilities/df.aggregate-window.md) |
| integer indices | [Choose a row or column selection representation](../capabilities/arrow.select.md) |
| limit | [Advertise only sound filter, projection and limit pushdown](../capabilities/df.pushdown.md) |
| plan | [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md) |
| projection indices | [Advertise only sound filter, projection and limit pushdown](../capabilities/df.pushdown.md) |
| reader | [Compose row-group pruning, row selection and decode filters](../capabilities/parquet.selection.md) |
| row groups | [Compose row-group pruning, row selection and decode filters](../capabilities/parquet.selection.md) |
| state arrays | [Reuse aggregate/window functions and characterize state before extensions](../capabilities/df.aggregate-window.md) |
| stream | [Match the boundary to data, plan or ABI interchange](../capabilities/df.interchange.md) |
| window partition | [Reuse aggregate/window functions and characterize state before extensions](../capabilities/df.aggregate-window.md) |
