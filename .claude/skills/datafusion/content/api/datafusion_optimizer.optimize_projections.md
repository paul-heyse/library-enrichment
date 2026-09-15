# `datafusion_optimizer::optimize_projections`

Crate `datafusion-optimizer` · 2 public items · structured records in [`model/datafusion_optimizer.optimize_projections.json`](../model/datafusion_optimizer.optimize_projections.json)

## is_projection_unnecessary

`function` · `datafusion_optimizer::optimize_projections::is_projection_unnecessary`

```rust
fn is_projection_unnecessary(input: &datafusion_expr::logical_plan::LogicalPlan, proj_exprs: &[datafusion_expr::Expr]) -> datafusion_common::Result<bool>
```

Projection is unnecessary, when
- input schema of the projection, output schema of the projection are same, and
- all projection expressions are either Column or Literal

---

## OptimizeProjections

`struct` · `datafusion_optimizer::optimize_projections::OptimizeProjections`

```rust
struct OptimizeProjections
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

Optimizer rule to prune unnecessary columns from intermediate schemas
inside the [`LogicalPlan`]. This rule:
- Removes unnecessary columns that do not appear at the output and/or are
  not used during any computation step.
- Adds projections to decrease table column size before operators that
  benefit from a smaller memory footprint at its input.
- Removes unnecessary [`LogicalPlan::Projection`]s from the [`LogicalPlan`].

`OptimizeProjections` is an optimizer rule that identifies and eliminates
columns from a logical plan that are not used by downstream operations.
This can improve query performance and reduce unnecessary data processing.

The rule analyzes the input logical plan, determines the necessary column
indices, and then removes any unnecessary columns. It also removes any
unnecessary projections from the plan tree.

## Schema, Field Properties, and Metadata Handling

The `OptimizeProjections` rule preserves schema and field metadata in most optimization scenarios:

**Schema-level metadata preservation by plan type**:
- **Window and Aggregate plans**: Schema metadata is preserved
- **Projection plans**: Schema metadata is preserved per [`projection_schema`](datafusion_expr::logical_plan::projection_schema).
- **Other logical plans**: Schema metadata is preserved unless [`LogicalPlan::recompute_schema`]
  is called on plan types that drop metadata

**Field-level properties and metadata**: Individual field properties are preserved when fields
are retained in the optimized plan, determined by [`exprlist_to_fields`](datafusion_expr::utils::exprlist_to_fields)
and [`ExprSchemable::to_field`](datafusion_expr::expr_schema::ExprSchemable::to_field).

**Field precedence**: When the same field appears multiple times, the optimizer
maintains one occurrence and removes duplicates (refer to `RequiredIndices::compact()`),
preserving the properties and metadata of that occurrence.

---
