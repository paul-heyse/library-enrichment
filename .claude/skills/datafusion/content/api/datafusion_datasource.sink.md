# `datafusion_datasource::sink`

Crate `datafusion-datasource` · 2 public items · structured records in [`model/datafusion_datasource.sink.json`](../model/datafusion_datasource.sink.json)

## DataSinkExec

`struct` · `datafusion_datasource::sink::DataSinkExec`

Also reachable as `datafusion::datasource::sink::DataSinkExec`

```rust
struct DataSinkExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn decode_sort_order(collection: Option<&datafusion_proto_models::protobuf::PhysicalSortExprNodeCollection>, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>, schema: &Schema) -> Result<Option<LexRequirement>>
fn encode_sort_order(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalSortExprNodeCollection>>
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(input: Arc<dyn ExecutionPlan>, sink: Arc<dyn DataSink>, sort_order: Option<LexRequirement>) -> Self
fn sink(&self) -> &dyn DataSink
fn sort_order(&self) -> &Option<LexRequirement>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.sink.DataSinkExec.md).


Execution plan for writing record batches to a [`DataSink`]

Returns a single row with the number of values written

---

## DataSink

`trait` · `datafusion_datasource::sink::DataSink`

Also reachable as `datafusion::datasource::sink::DataSink`

```rust
trait DataSink: Any + DisplayAs + Debug + Send + Sync
```

**Implementors** (4)

- `datafusion_datasource::memory::MemSink`
- `datafusion_datasource_csv::file_format::CsvSink`
- `datafusion_datasource_json::file_format::JsonSink`
- `datafusion_datasource_parquet::sink::ParquetSink`

**Methods** (4)

```rust
fn metrics(&self) -> Option<MetricsSet>
fn schema(&self) -> &SchemaRef
fn try_to_proto(&self, _exec: &DataSinkExec, _ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.sink.DataSink.md).


`DataSink` implements writing streams of [`RecordBatch`]es to
user defined destinations.

The `Display` impl is used to format the sink for explain plan
output.

---
