# `datafusion_datasource_arrow::source`

Crate `datafusion-datasource-arrow` · 2 public items · structured records in [`model/datafusion_datasource_arrow.source.json`](../model/datafusion_datasource_arrow.source.json)

## ArrowOpener

`struct` · `datafusion_datasource_arrow::source::ArrowOpener`

Also reachable as `datafusion::datasource::physical_plan::ArrowOpener`, `datafusion::datasource::physical_plan::arrow::ArrowOpener`

```rust
struct ArrowOpener
```

**Fields**: `inner`

**Implements**: `datafusion_datasource::file_stream::FileOpener`

**Methods** (3)

```rust
fn new(inner: Arc<dyn FileOpener>) -> Self
fn new_file_opener(object_store: Arc<dyn ObjectStore>, projection: Option<Vec<usize>>) -> Self
fn new_stream_file_opener(object_store: Arc<dyn ObjectStore>, projection: Option<Vec<usize>>) -> Self
```

**via `datafusion_datasource::file_stream::FileOpener`**

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_arrow.source.ArrowOpener.md).


`FileOpener` wrapper for both Arrow IPC file and stream formats

---

## ArrowSource

`struct` · `datafusion_datasource_arrow::source::ArrowSource`

Also reachable as `datafusion::datasource::physical_plan::ArrowSource`, `datafusion::datasource::physical_plan::arrow::ArrowSource`

```rust
struct ArrowSource
```

**Implements**: `datafusion_datasource::file::FileSource`

**Derives**: Clone

**Methods** (3)

```rust
fn new_file_source(table_schema: impl Into<TableSchema>) -> Self
fn new_stream_file_source(table_schema: impl Into<TableSchema>) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
```

**via `datafusion_datasource::file::FileSource`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, _base_config: &FileScanConfig, _partition: usize) -> Result<Arc<dyn FileOpener>>
fn file_type(&self) -> &str
fn metrics(&self) -> &ExecutionPlanMetricsSet
fn projection(&self) -> Option<&ProjectionExprs>
fn repartitioned(&self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>, config: &FileScanConfig) -> Result<Option<FileScanConfig>>
fn table_schema(&self) -> &TableSchema
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_batch_size(&self, _batch_size: usize) -> Arc<dyn FileSource>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_arrow.source.ArrowSource.md).


`FileSource` for both Arrow IPC file and stream formats

---
