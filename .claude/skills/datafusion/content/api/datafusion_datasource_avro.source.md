# `datafusion_datasource_avro::source`

Crate `datafusion-datasource-avro` · 1 public items · structured records in [`model/datafusion_datasource_avro.source.json`](../model/datafusion_datasource_avro.source.json)

## AvroSource

`struct` · `datafusion_datasource_avro::source::AvroSource`

Also reachable as `datafusion::datasource::physical_plan::AvroSource`, `datafusion::datasource::physical_plan::avro::AvroSource`

```rust
struct AvroSource
```

**Implements**: `datafusion_datasource::file::FileSource`

**Derives**: Clone

**Methods** (2)

```rust
fn new(table_schema: impl Into<TableSchema>) -> Self
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
```

**via `datafusion_datasource::file::FileSource`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, _base_config: &FileScanConfig, _partition: usize) -> Result<Arc<dyn FileOpener>>
fn file_type(&self) -> &str
fn metrics(&self) -> &ExecutionPlanMetricsSet
fn projection(&self) -> Option<&ProjectionExprs>
fn supports_repartitioning(&self) -> bool
fn table_schema(&self) -> &TableSchema
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

AvroSource holds the extra configuration that is necessary for opening avro files

---
