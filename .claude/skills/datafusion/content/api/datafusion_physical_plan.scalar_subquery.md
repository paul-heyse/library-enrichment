# `datafusion_physical_plan::scalar_subquery`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.scalar_subquery.json`](../model/datafusion_physical_plan.scalar_subquery.json)

## ScalarSubqueryExec

`struct` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec`

```rust
struct ScalarSubqueryExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug

**Methods** (5)

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(input: Arc<dyn ExecutionPlan>, subqueries: Vec<ScalarSubqueryLink>, results: ScalarSubqueryResults) -> Self
fn results(&self) -> &ScalarSubqueryResults
fn subqueries(&self) -> &[ScalarSubqueryLink]
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn maintains_input_order(&self) -> Vec<bool>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Manages execution of uncorrelated scalar subqueries for a single plan
level.

From a query-results perspective, this node is a pass-through: it yields
the same batches as its main input and exists only to populate scalar
subquery results as a side effect before those batches are produced.

The first child node is the **main input plan**, whose batches are passed
through unchanged. The remaining children are **subquery plans**, each of
which must produce exactly zero or one row. Before any batches from the main
input are yielded, all subquery plans are executed and their scalar results
are stored in a shared [`ScalarSubqueryResults`] container owned by this
node. [`ScalarSubqueryExpr`] nodes embedded in the main input's expressions
hold the same container and read from it by index.

All subqueries are evaluated eagerly when the first output partition is
requested, before any rows from the main input are produced.

TODO: Consider overlapping computation of the subqueries with evaluating the
main query.

[`ScalarSubqueryExpr`]: datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr

---

## ScalarSubqueryLink

`struct` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink`

```rust
struct ScalarSubqueryLink
```

**Fields**: `plan`, `index`

**Derives**: Clone, Debug

Links a scalar subquery's execution plan to its index in the shared results
container. The [`ScalarSubqueryExec`] that owns these links populates
`results[index]` at execution time, and [`ScalarSubqueryExpr`] instances
with the same index read from it.

[`ScalarSubqueryExpr`]: datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr

---
