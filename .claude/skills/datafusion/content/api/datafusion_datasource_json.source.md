# `datafusion_datasource_json::source`

Crate `datafusion-datasource-json` · 3 public items · structured records in [`model/datafusion_datasource_json.source.json`](../model/datafusion_datasource_json.source.json)

## plan_to_json

`function` · `datafusion_datasource_json::source::plan_to_json`

Also reachable as `datafusion::datasource::physical_plan::json::plan_to_json`

```rust
async fn plan_to_json(task_ctx: std::sync::Arc<datafusion_execution::TaskContext>, plan: std::sync::Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> datafusion_common::error::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_json.source.plan_to_json.md).


---

## JsonOpener

`struct` · `datafusion_datasource_json::source::JsonOpener`

Also reachable as `datafusion::datasource::physical_plan::JsonOpener`, `datafusion::datasource::physical_plan::json::JsonOpener`

```rust
struct JsonOpener
```

**Implements**: `datafusion_datasource::file_stream::FileOpener`

**Methods** (1)

```rust
fn new(batch_size: usize, projected_schema: SchemaRef, file_compression_type: FileCompressionType, object_store: Arc<dyn ObjectStore>, newline_delimited: bool) -> Self
```

**via `datafusion_datasource::file_stream::FileOpener`**

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_json.source.JsonOpener.md).


A [`FileOpener`] that opens a JSON file and yields a [`FileOpenFuture`]

---

## JsonSource

`struct` · `datafusion_datasource_json::source::JsonSource`

Also reachable as `datafusion::datasource::physical_plan::JsonSource`, `datafusion::datasource::physical_plan::json::JsonSource`

```rust
struct JsonSource
```

**Implements**: `datafusion_datasource::file::FileSource`

**Derives**: Clone

**Methods** (3)

```rust
fn new(table_schema: impl Into<datafusion_datasource::TableSchema>) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn with_newline_delimited(self, newline_delimited: bool) -> Self
```

**via `datafusion_datasource::file::FileSource`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, _partition: usize) -> Result<Arc<dyn FileOpener>>
fn file_type(&self) -> &str
fn metrics(&self) -> &ExecutionPlanMetricsSet
fn projection(&self) -> Option<&ProjectionExprs>
fn table_schema(&self) -> &datafusion_datasource::TableSchema
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_json.source.JsonSource.md).


JsonSource holds the extra configuration that is necessary for [`JsonOpener`]

---
