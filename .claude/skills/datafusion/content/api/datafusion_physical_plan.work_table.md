# `datafusion_physical_plan::work_table`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.work_table.json`](../model/datafusion_physical_plan.work_table.json)

## WorkTable

`struct` · `datafusion_physical_plan::work_table::WorkTable`

Also reachable as `datafusion::physical_plan::WorkTable`, `datafusion_physical_plan::WorkTable`

```rust
struct WorkTable
```

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.work_table.WorkTable.md).


The name is from PostgreSQL's terminology.
See <https://wiki.postgresql.org/wiki/CTEReadme#How_Recursion_Works>
This table serves as a mirror or buffer between each iteration of a recursive query.

---

## WorkTableExec

`struct` · `datafusion_physical_plan::work_table::WorkTableExec`

```rust
struct WorkTableExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn name(&self) -> &str
fn new(name: String, schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
fn schema(&self) -> SchemaRef
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_state(&self, state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.work_table.WorkTableExec.md).


A temporary "working table" operation where the input data will be
taken from the named handle during the execution and will be re-published
as is (kind of like a mirror).

Most notably used in the implementation of recursive queries where the
underlying relation does not exist yet but the data will come as the previous
term is evaluated. This table will be used such that the recursive plan
will register a receiver in the task context and this plan will use that
receiver to get the data and stream it back up so that the batches are available
in the next iteration.

---
