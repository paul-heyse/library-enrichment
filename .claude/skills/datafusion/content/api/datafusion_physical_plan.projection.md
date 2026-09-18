# `datafusion_physical_plan::projection`

Crate `datafusion-physical-plan` · 19 public items · structured records in [`model/datafusion_physical_plan.projection.json`](../model/datafusion_physical_plan.projection.json)

## all_alias_free_columns

`function` · `datafusion_physical_plan::projection::all_alias_free_columns`

```rust
fn all_alias_free_columns(exprs: &[ProjectionExpr]) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.all_alias_free_columns.md).


Given the expression set of a projection, checks if the projection causes
any renaming or constructs a non-`Column` physical expression.

---

## all_columns

`function` · `datafusion_physical_plan::projection::all_columns`

```rust
fn all_columns(exprs: &[ProjectionExpr]) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.all_columns.md).


Returns `true` if all the expressions in the argument are `Column`s.

---

## join_allows_pushdown

`function` · `datafusion_physical_plan::projection::join_allows_pushdown`

```rust
fn join_allows_pushdown(projection_as_columns: &[(super::expressions::Column, String)], join_schema: &arrow::datatypes::SchemaRef, far_right_left_col_ind: i32, far_left_right_col_ind: i32) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.join_allows_pushdown.md).


Checks three conditions for pushing a projection down through a join:
- Projection must narrow the join output schema.
- Columns coming from left/right tables must be collected at the left/right
  sides of the output table.
- Left or right table is not lost after the projection.

---

## join_table_borders

`function` · `datafusion_physical_plan::projection::join_table_borders`

```rust
fn join_table_borders(left_table_column_count: usize, projection_as_columns: &[(super::expressions::Column, String)]) -> (i32, i32)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.join_table_borders.md).


Returns the last index before encountering a column coming from the right table when traveling
through the projection from left to right, and the last index before encountering a column
coming from the left table when traveling through the projection from right to left.
If there is no column in the projection coming from the left side, it returns (-1, ...),
if there is no column in the projection coming from the right side, it returns (..., projection length).

---

## make_with_child

`function` · `datafusion_physical_plan::projection::make_with_child`

```rust
fn make_with_child(projection: &ProjectionExec, child: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.make_with_child.md).


Creates a new [`ProjectionExec`] instance with the given child plan and
projected expressions, preserving the original output metadata.

---

## new_join_children

`function` · `datafusion_physical_plan::projection::new_join_children`

```rust
fn new_join_children(projection_as_columns: &[(super::expressions::Column, String)], far_right_left_col_ind: i32, far_left_right_col_ind: i32, left_child: &std::sync::Arc<dyn ExecutionPlan>, right_child: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<(ProjectionExec, ProjectionExec)>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.new_join_children.md).


If pushing down the projection over this join's children seems possible,
this function constructs the new [`ProjectionExec`]s that will come on top
of the original children of the join.

---

## new_projections_for_columns

`function` · `datafusion_physical_plan::projection::new_projections_for_columns`

```rust
fn new_projections_for_columns(projection: &[ProjectionExpr], source: &[usize]) -> Vec<usize>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.new_projections_for_columns.md).


Updates a source provider's projected columns according to the given
projection operator's expressions. To use this function safely, one must
ensure that all expressions are `Column` expressions without aliases.

---

## physical_to_column_exprs

`function` · `datafusion_physical_plan::projection::physical_to_column_exprs`

```rust
fn physical_to_column_exprs(exprs: &[ProjectionExpr]) -> Option<Vec<(super::expressions::Column, String)>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.physical_to_column_exprs.md).


Downcasts all the expressions in `exprs` to `Column`s. If any of the given
expressions is not a `Column`, returns `None`.

---

## remove_unnecessary_projections

`function` · `datafusion_physical_plan::projection::remove_unnecessary_projections`

```rust
fn remove_unnecessary_projections(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.remove_unnecessary_projections.md).


This function checks if `plan` is a [`ProjectionExec`], and inspects its
input(s) to test whether it can push `plan` under its input(s). This function
will operate on the entire tree and may ultimately remove `plan` entirely
by leveraging source providers with built-in projection capabilities.

---

## try_embed_projection

`function` · `datafusion_physical_plan::projection::try_embed_projection`

```rust
fn try_embed_projection<Exec: EmbeddedProjection + 'static>(projection: &ProjectionExec, execution_plan: &Exec) -> datafusion_common::Result<Option<std::sync::Arc<dyn ExecutionPlan>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.try_embed_projection.md).


Some projection can't be pushed down left input or right input of hash join because filter or on need may need some columns that won't be used in later.
By embed those projection to hash join, we can reduce the cost of build_batch_from_indices in hash join (build_batch_from_indices need to can compute::take() for each column) and avoid unnecessary output creation.

---

## try_pushdown_through_join

`function` · `datafusion_physical_plan::projection::try_pushdown_through_join`

> **Deprecated** — since 55.0.0: Use try_pushdown_through_join_with_column_indices instead

```rust
fn try_pushdown_through_join(projection: &ProjectionExec, join_left: &std::sync::Arc<dyn ExecutionPlan>, join_right: &std::sync::Arc<dyn ExecutionPlan>, join_on: joins::utils::JoinOnRef<'_>, schema: &arrow::datatypes::SchemaRef, filter: Option<&joins::utils::JoinFilter>) -> datafusion_common::Result<Option<JoinData>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.try_pushdown_through_join.md).


---

## try_pushdown_through_join_with_column_indices

`function` · `datafusion_physical_plan::projection::try_pushdown_through_join_with_column_indices`

```rust
fn try_pushdown_through_join_with_column_indices(projection: &ProjectionExec, join_left: &std::sync::Arc<dyn ExecutionPlan>, join_right: &std::sync::Arc<dyn ExecutionPlan>, join_on: joins::utils::JoinOnRef<'_>, schema: &arrow::datatypes::SchemaRef, filter: Option<&joins::utils::JoinFilter>, column_indices: &[joins::utils::ColumnIndex]) -> datafusion_common::Result<Option<JoinData>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.try_pushdown_through_join_with_column_indices.md).


Attempts to move a projection below a join by mapping each join output
column to the child column that produced it.

`schema` is the complete output schema of the join, not either child's
schema. `column_indices` must contain one entry for each field in `schema`.
Each [`JoinSide::Left`] or [`JoinSide::Right`] entry identifies the source
child and uses an index relative to that child's schema.

[`JoinSide::None`] identifies a column produced by the join itself, such as
a mark column. If `projection` references such a column, this function
returns `Ok(None)` because neither child can produce it.

Returns `Ok(None)` when the projection cannot be pushed down safely.

# Errors

Returns an error if `column_indices` does not match `schema` or contains an
index outside the corresponding child schema.

---

## update_join_filter

`function` · `datafusion_physical_plan::projection::update_join_filter`

```rust
fn update_join_filter(projection_left_exprs: &[(super::expressions::Column, String)], projection_right_exprs: &[(super::expressions::Column, String)], join_filter: &joins::utils::JoinFilter, left_field_size: usize) -> Option<joins::utils::JoinFilter>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.update_join_filter.md).


Tries to update the column indices of a [`JoinFilter`] as if the input of
the join was replaced by a projection.

---

## update_join_on

`function` · `datafusion_physical_plan::projection::update_join_on`

```rust
fn update_join_on(proj_left_exprs: &[(super::expressions::Column, String)], proj_right_exprs: &[(super::expressions::Column, String)], hash_join_on: &[(datafusion_physical_expr_common::physical_expr::PhysicalExprRef, datafusion_physical_expr_common::physical_expr::PhysicalExprRef)], left_field_size: usize) -> Option<Vec<(datafusion_physical_expr_common::physical_expr::PhysicalExprRef, datafusion_physical_expr_common::physical_expr::PhysicalExprRef)>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.update_join_on.md).


Tries to update the equi-join `Column`'s of a join as if the input of
the join was replaced by a projection.

---

## update_ordering

`function` · `datafusion_physical_plan::projection::update_ordering`

```rust
fn update_ordering(ordering: datafusion_physical_expr_common::sort_expr::LexOrdering, projected_exprs: &[ProjectionExpr]) -> datafusion_common::Result<Option<datafusion_physical_expr_common::sort_expr::LexOrdering>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.update_ordering.md).


Updates the given lexicographic ordering according to given projected
expressions using the [`update_expr`] function.

---

## update_ordering_requirement

`function` · `datafusion_physical_plan::projection::update_ordering_requirement`

```rust
fn update_ordering_requirement(reqs: datafusion_physical_expr_common::sort_expr::LexRequirement, projected_exprs: &[ProjectionExpr]) -> datafusion_common::Result<Option<datafusion_physical_expr_common::sort_expr::LexRequirement>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.update_ordering_requirement.md).


Updates the given lexicographic requirement according to given projected
expressions using the [`update_expr`] function.

---

## JoinData

`struct` · `datafusion_physical_plan::projection::JoinData`

```rust
struct JoinData
```

**Fields**: `projected_left_child`, `projected_right_child`, `join_filter`, `join_on`

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.JoinData.md).


---

## ProjectionExec

`struct` · `datafusion_physical_plan::projection::ProjectionExec`

```rust
struct ProjectionExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn expr(&self) -> &[ProjectionExpr]
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn projection_expr(&self) -> &ProjectionExprs
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new<I, E>(expr: I, input: Arc<dyn ExecutionPlan>) -> Result<Self> where I: IntoIterator<Item = E>, E: Into<ProjectionExpr>
fn try_new_with_schema_metadata<I, E>(expr: I, input: Arc<dyn ExecutionPlan>, projected_schema: &Schema) -> Result<Self> where I: IntoIterator<Item = E>, E: Into<ProjectionExpr>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.ProjectionExec.md).


[`ExecutionPlan`] for a projection

Computes a set of scalar value expressions for each input row, producing one
output row for each input row.

---

## EmbeddedProjection

`trait` · `datafusion_physical_plan::projection::EmbeddedProjection`

```rust
trait EmbeddedProjection: ExecutionPlan + Sized
```

**Implementors** (3)

- `datafusion_physical_plan::filter::FilterExec`
- `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec`
- `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec`

**Methods** (1)

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.projection.EmbeddedProjection.md).


Trait for execution plans that can embed a projection, avoiding a separate
[`ProjectionExec`] wrapper.

# Empty projections

`Some(vec![])` is a valid projection that produces zero output columns while
preserving the correct row count. Implementors must ensure that runtime batch
construction still returns batches with the right number of rows even when no
columns are selected (e.g. for `SELECT count(1) … JOIN …`).

---
