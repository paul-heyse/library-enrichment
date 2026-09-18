# `datafusion_datasource::memory`

Crate `datafusion-datasource` · 3 public items · structured records in [`model/datafusion_datasource.memory.json`](../model/datafusion_datasource.memory.json)

## MemSink

`struct` · `datafusion_datasource::memory::MemSink`

```rust
struct MemSink
```

**Implements**: `datafusion_datasource::sink::DataSink`, `datafusion_physical_plan::display::DisplayAs`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(batches: Vec<PartitionData>, schema: SchemaRef) -> Result<Self>
```

**via `datafusion_datasource::sink::DataSink`**

```rust
fn schema(&self) -> &SchemaRef
async fn write_all(&self, data: SendableRecordBatchStream, _context: &Arc<TaskContext>) -> Result<u64>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.memory.MemSink.md).


Implements for writing to a [`MemTable`]

[`MemTable`]: <https://docs.rs/datafusion/latest/datafusion/datasource/memory/struct.MemTable.html>

---

## MemorySourceConfig

`struct` · `datafusion_datasource::memory::MemorySourceConfig`

Also reachable as `datafusion::datasource::memory::MemorySourceConfig`, `datafusion_catalog::memory::MemorySourceConfig`

```rust
struct MemorySourceConfig
```

**Implements**: `datafusion_datasource::source::DataSource`

**Derives**: Clone, Debug

**Methods** (13)

```rust
fn original_schema(&self) -> SchemaRef
fn partitions(&self) -> &[Vec<RecordBatch>]
fn projection(&self) -> &Option<Vec<usize>>
fn show_sizes(&self) -> bool
fn sort_information(&self) -> &[LexOrdering]
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
fn try_new(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
fn try_new_as_values(schema: SchemaRef, data: Vec<Vec<Arc<dyn PhysicalExpr>>>) -> Result<Arc<DataSourceExec>>
fn try_new_exec(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Arc<DataSourceExec>>
fn try_new_from_batches(schema: SchemaRef, batches: Vec<RecordBatch>) -> Result<Arc<DataSourceExec>>
fn try_with_sort_information(self, sort_information: Vec<LexOrdering>) -> Result<Self>
fn with_limit(self, limit: Option<usize>) -> Self
fn with_show_sizes(self, show_sizes: bool) -> Self
```

**via `datafusion_datasource::source::DataSource`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn eq_properties(&self) -> EquivalenceProperties
fn fetch(&self) -> Option<usize>
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn open(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn output_partitioning(&self) -> Partitioning
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
fn repartitioned(&self, target_partitions: usize, _repartition_file_min_size: usize, output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>>
fn scheduling_type(&self) -> SchedulingType
fn try_swapping_with_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn DataSource>>>
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn DataSource>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.memory.MemorySourceConfig.md).


Data source configuration for reading in-memory batches of data

---

## PartitionData

`type_alias` · `datafusion_datasource::memory::PartitionData`

```rust
type PartitionData = std::sync::Arc<tokio::sync::RwLock<Vec<arrow::array::RecordBatch>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.memory.PartitionData.md).


Type alias for partition data

---
