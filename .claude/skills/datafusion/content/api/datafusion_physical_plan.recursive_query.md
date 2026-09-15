# `datafusion_physical_plan::recursive_query`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.recursive_query.json`](../model/datafusion_physical_plan.recursive_query.json)

## RecursiveQueryExec

`struct` · `datafusion_physical_plan::recursive_query::RecursiveQueryExec`

```rust
struct RecursiveQueryExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn is_distinct(&self) -> bool
fn name(&self) -> &str
fn recursive_term(&self) -> &Arc<dyn ExecutionPlan>
fn static_term(&self) -> &Arc<dyn ExecutionPlan>
fn try_new(name: String, output_schema: SchemaRef, static_term: Arc<dyn ExecutionPlan>, recursive_term: Arc<dyn ExecutionPlan>, is_distinct: bool) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
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
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Recursive query execution plan.

This plan has two components: a base part (the static term) and
a dynamic part (the recursive term). The execution will start from
the base, and as long as the previous iteration produced at least
a single new row (taking care of the distinction) the recursive
part will be continuously executed.

Before each execution of the dynamic part, the rows from the previous
iteration will be available in a "working table" (not a real table,
can be only accessed using a continuance operation).

Note that there won't be any limit or checks applied to detect
an infinite recursion, so it is up to the planner to ensure that
it won't happen.

---
