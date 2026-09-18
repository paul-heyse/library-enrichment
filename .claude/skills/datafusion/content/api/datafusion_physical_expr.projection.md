# `datafusion_physical_expr::projection`

Crate `datafusion-physical-expr` · 10 public items · structured records in [`model/datafusion_physical_expr.projection.json`](../model/datafusion_physical_expr.projection.json)

## combine_projections

`function` · `datafusion_physical_expr::projection::combine_projections`

```rust
fn combine_projections(p1: Option<&ProjectionRef>, p2: Option<&ProjectionRef>) -> datafusion_common::Result<Option<ProjectionRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.combine_projections.md).


Combine two projections.

If `p1` is [`None`] then there are no changes.
Otherwise, if passed `p2` is not [`None`] then it is remapped
according to the `p1`. Otherwise, there are no changes.

# Example

If stored projection is [0, 2] and we call `apply_projection([0, 2, 3])`,
then the resulting projection will be [0, 3].

# Error

Returns an internal error if `p1` contains index that is greater than `p2` len.

---

## project_ordering

`function` · `datafusion_physical_expr::projection::project_ordering`

Also reachable as `datafusion_physical_expr::equivalence::project_ordering`

```rust
fn project_ordering(ordering: &datafusion_physical_expr_common::sort_expr::LexOrdering, schema: &arrow::datatypes::SchemaRef) -> Option<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.project_ordering.md).


Projects a single [LexOrdering] onto the given schema.

This function attempts to rewrite every [PhysicalSortExpr] in the provided
[LexOrdering] so that any [Column] expressions point at the correct field
indices in `schema`.

Key details:
- Columns are matched by name, not by index. The index of each matched
  column is looked up with [Schema::column_with_name](arrow::datatypes::Schema::column_with_name) and a new
  [Column] with the correct [index](Column::index) is substituted.
- If an expression references a column name that does not exist in
  `schema`, projection of the current ordering stops and only the already
  rewritten prefix is kept. This models the fact that a lexicographical
  ordering remains valid for any leading prefix whose expressions are
  present in the projected schema.
- If no expressions can be projected (i.e. the first one is missing), the
  function returns `None`.

Return value:
- `Some(LexOrdering)` if at least one sort expression could be projected.
  The returned ordering may be a strict prefix of the input ordering.
- `None` if no part of the ordering can be projected onto `schema`.

Example

Suppose we have an input ordering `[col("a@0"), col("b@1")]` but the projected
schema only contains b and not a. The result will be `Some([col("a@0")])`. In other
words, the column reference is reindexed to match the projected schema.
If neither a nor b is present, the result will be None.

---

## project_orderings

`function` · `datafusion_physical_expr::projection::project_orderings`

Also reachable as `datafusion_physical_expr::equivalence::project_orderings`

```rust
fn project_orderings(orderings: &[datafusion_physical_expr_common::sort_expr::LexOrdering], schema: &arrow::datatypes::SchemaRef) -> Vec<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.project_orderings.md).


Projects a slice of [LexOrdering]s onto the given schema.

This is a convenience wrapper that applies [project_ordering] to each
input ordering and collects the successful projections:
- For each input ordering, the result of [project_ordering] is appended to
  the output if it is `Some(...)`.
- Order is preserved and no deduplication is attempted.
- If none of the input orderings can be projected, an empty `Vec` is
  returned.

See [project_ordering] for the semantics of projecting a single
[LexOrdering].

---

## update_expr

`function` · `datafusion_physical_expr::projection::update_expr`

Also reachable as `datafusion_physical_plan::projection::update_expr`

```rust
fn update_expr(expr: &std::sync::Arc<dyn PhysicalExpr>, projected_exprs: &[ProjectionExpr], unproject: bool) -> datafusion_common::Result<Option<std::sync::Arc<dyn PhysicalExpr>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.update_expr.md).


The function projects / unprojects an expression with respect to set of
projection expressions.

See also [`ProjectionExprs::unproject_expr`] and [`ProjectionExprs::project_expr`]

1) When `unproject` is `true`:

   Rewrites an expression with respect to the projection expressions,
   effectively "unprojecting" it to reference the original input columns.

   For example, given
   * the expressions `a@1 + b@2` and `c@0`
   * and projection expressions `c@2, a@0, b@1`

   Then
   * `a@1 + b@2` becomes `a@0 + b@1`
   * `c@0` becomes `c@2`

2) When `unproject` is `false`:

   Rewrites the expression to reference the projected expressions,
   effectively "projecting" it. The resulting expression will reference the
   indices as they appear in the projection.

   If the expression cannot be rewritten after the projection, it returns
   `None`.

   For example, given
   * the expressions `c@0`, `a@1` and `b@2`
   * the projection `a@1 as a, c@0 as c_new`,

   Then
   * `c@0` becomes `c_new@1`
   * `a@1` becomes `a@0`
   * `b@2` results in `None` since the projection does not include `b`.

# Errors
This function returns an error if `unproject` is `true` and if any expression references
an index that is out of bounds for `projected_exprs`.
For example:

- `expr` is `a@3`
- `projected_exprs` is \[`a@0`, `b@1`\]

In this case, `a@3` references index 3, which is out of bounds for `projected_exprs` (which has length 2).

---

## ProjectionExpr

`struct` · `datafusion_physical_expr::projection::ProjectionExpr`

Also reachable as `datafusion_physical_plan::projection::ProjectionExpr`

```rust
struct ProjectionExpr
```

**Fields**: `expr`, `alias`

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `datafusion_physical_plan::execution_plan::AsPhysicalExprRef`

**Derives**: Clone, Debug, Eq, PartialEq

**Methods** (2)

```rust
fn new(expr: Arc<dyn PhysicalExpr>, alias: impl Into<String>) -> Self
fn new_from_expression(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Arc<dyn PhysicalExpr>
```

**via `core::convert::From`**

```rust
fn from(value: &(Arc<dyn PhysicalExpr>, String)) -> Self
fn from(value: (Arc<dyn PhysicalExpr>, String)) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.ProjectionExpr.md).


An expression used by projection operations.

The expression is evaluated and the result is stored in a column
with the name specified by `alias`.

For example, the SQL expression `a + b AS sum_ab` would be represented
as a `ProjectionExpr` where `expr` is the expression `a + b`
and `alias` is the string `sum_ab`.

See [`ProjectionExprs`] for a collection of projection expressions.

---

## ProjectionExprs

`struct` · `datafusion_physical_expr::projection::ProjectionExprs`

Also reachable as `datafusion_physical_plan::projection::ProjectionExprs`

```rust
struct ProjectionExprs
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (18)

```rust
fn column_indices(&self) -> Vec<usize>
fn create_expression_metrics(&self, metrics: &ExecutionPlanMetricsSet, partition: usize) -> ExpressionEvaluatorMetrics
fn expr_iter(&self) -> impl Iterator<Item = Arc<dyn PhysicalExpr>> + '_
fn from_expressions(exprs: impl Into<Arc<[ProjectionExpr]>>) -> Self
fn from_indices(indices: &[usize], schema: &Schema) -> Self
fn iter(&self) -> impl Iterator<Item = &ProjectionExpr>
fn make_projector(&self, input_schema: &Schema) -> Result<Projector>
fn make_projector_with_schema_metadata(&self, input_schema: &Schema, projected_schema: &Schema) -> Result<Projector>
fn new(exprs: impl IntoIterator<Item = ProjectionExpr>) -> Self
fn ordered_column_indices(&self) -> Vec<usize>
fn project_expr(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
fn project_schema(&self, input_schema: &Schema) -> Result<Schema>
fn project_statistics(&self, stats: Statistics, output_schema: &Schema) -> Result<Statistics>
fn projected_column_position(&self, column: &Column) -> Option<usize>
fn projection_mapping(&self, input_schema: &SchemaRef) -> Result<ProjectionMapping>
fn try_map_exprs<F>(self, f: F) -> Result<Self> where F: FnMut(Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
fn try_merge(&self, other: &ProjectionExprs) -> Result<ProjectionExprs>
fn unproject_expr(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[ProjectionExpr]
```

**via `core::convert::From`**

```rust
fn from(value: &[ProjectionExpr]) -> Self
fn from(value: Vec<ProjectionExpr>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = ProjectionExpr>>(exprs: T) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.ProjectionExprs.md).


A collection of  [`ProjectionExpr`] instances, representing a complete
projection operation.

Projection operations are used in query plans to select specific columns or
compute new columns based on existing ones.

See [`ProjectionExprs::from_indices`] to select a subset of columns by
indices.

---

## ProjectionMapping

`struct` · `datafusion_physical_expr::projection::ProjectionMapping`

Also reachable as `datafusion_physical_expr::equivalence::ProjectionMapping`

```rust
struct ProjectionMapping
```

**Implements**: `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn from_indices(indices: &[usize], schema: &SchemaRef) -> Result<Self>
fn try_new(expr: impl IntoIterator<Item = (Arc<dyn PhysicalExpr>, String)>, input_schema: &SchemaRef) -> Result<Self>
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = (Arc<dyn PhysicalExpr>, ProjectionTargets)>>(iter: T) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.ProjectionMapping.md).


Stores the mapping between source expressions and target expressions for a
projection.

---

## ProjectionTargets

`struct` · `datafusion_physical_expr::projection::ProjectionTargets`

```rust
struct ProjectionTargets
```

**Implements**: `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn first(&self) -> &(Arc<dyn PhysicalExpr>, usize)
fn push(&mut self, target: (Arc<dyn PhysicalExpr>, usize))
```

**via `core::convert::From`**

```rust
fn from(exprs_indices: Vec<(Arc<dyn PhysicalExpr>, usize)>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.ProjectionTargets.md).


Stores target expressions, along with their indices, that associate with a
source expression in a projection mapping.

---

## Projector

`struct` · `datafusion_physical_expr::projection::Projector`

```rust
struct Projector
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn output_schema(&self) -> &SchemaRef
fn project_batch(&self, batch: &RecordBatch) -> Result<RecordBatch>
fn projection(&self) -> &ProjectionExprs
fn with_metrics(&self, metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.Projector.md).


Applies a projection to record batches.

A [`Projector`] uses a set of projection expressions to transform
and a pre-computed output schema to project record batches accordingly.

The main reason to use a `Projector` is to avoid repeatedly computing
the output schema for each batch, which can be costly if the projection
expressions are complex.

---

## ProjectionRef

`type_alias` · `datafusion_physical_expr::projection::ProjectionRef`

```rust
type ProjectionRef = std::sync::Arc<[usize]>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.projection.ProjectionRef.md).


Describes an immutable reference counted projection.

This structure represents projecting a set of columns by index.
[`Arc`] is used to make it cheap to clone.

---
