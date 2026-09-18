# `datafusion_physical_plan::streaming`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.streaming.json`](../model/datafusion_physical_plan.streaming.json)

## StreamingTableExec

`struct` · `datafusion_physical_plan::streaming::StreamingTableExec`

```rust
struct StreamingTableExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn is_infinite(&self) -> bool
fn limit(&self) -> Option<usize>
fn partition_schema(&self) -> &SchemaRef
fn partitions(&self) -> &Vec<Arc<dyn PartitionStream>>
fn projected_output_ordering(&self) -> impl IntoIterator<Item = LexOrdering>
fn projected_schema(&self) -> &Schema
fn projection(&self) -> &Option<Arc<[usize]>>
fn try_new(schema: SchemaRef, partitions: Vec<Arc<dyn PartitionStream>>, projection: Option<&Vec<usize>>, projected_output_ordering: impl IntoIterator<Item = LexOrdering>, infinite: bool, limit: Option<usize>) -> Result<Self>
fn with_output_partitioning(self, output_partitioning: Partitioning) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, ctx: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.streaming.StreamingTableExec.md).


An [`ExecutionPlan`] for one or more [`PartitionStream`]s.

If your source can be represented as one or more [`PartitionStream`]s, you can
use this struct to implement [`ExecutionPlan`].

---

## PartitionStream

`trait` · `datafusion_physical_plan::streaming::PartitionStream`

```rust
trait PartitionStream: Debug + Send + Sync
```

**Implementors** (1)

- `datafusion_physical_plan::test::TestPartitionStream`

**Methods** (2)

```rust
fn execute(&self, ctx: Arc<TaskContext>) -> SendableRecordBatchStream
fn schema(&self) -> &SchemaRef
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.streaming.PartitionStream.md).


A partition that can be converted into a [`SendableRecordBatchStream`]

Combined with [`StreamingTableExec`], you can use this trait to implement
[`ExecutionPlan`] for a custom source with less boiler plate than
implementing `ExecutionPlan` directly for many use cases.

---
