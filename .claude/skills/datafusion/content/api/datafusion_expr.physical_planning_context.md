# `datafusion_expr::physical_planning_context`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.physical_planning_context.json`](../model/datafusion_expr.physical_planning_context.json)

## PhysicalPlanningContext

`struct` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext`

```rust
struct PhysicalPlanningContext
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn index_of(&self, subquery: &logical_plan::Subquery) -> Option<SubqueryIndex>
fn lambda_variable_qualifier(&self, name: &str) -> Option<&TableReference>
fn new(indexes: HashMap<logical_plan::Subquery, SubqueryIndex>, results: ScalarSubqueryResults) -> Self
fn results(&self) -> &ScalarSubqueryResults
fn with_qualified_lambda_variables(self, qualifier: &TableReference, variables: &[String]) -> Self
```

Context used while converting a logical plan subtree into a physical plan.

Unlike [`ExecutionProps`](crate::execution_props::ExecutionProps), which
applies to the overall planning and execution of a query, this context can
differ between recursively planned subtrees. It currently carries:

* the state needed to create physical expressions for
  [`Expr::ScalarSubquery`] nodes that read from a shared
  [`ScalarSubqueryResults`] container, and
* the qualifiers assigned to the [`Expr::LambdaVariable`]s that are in scope.

The physical planner builds this context from the set of uncorrelated scalar
subqueries it has scheduled for a subtree. It is then passed explicitly
through `create_physical_expr` so that function can find the slot index for
each [`Subquery`]. While planning the body of a lambda,
`create_physical_expr` extends the context with the lambda's parameters via
[`Self::with_qualified_lambda_variables`].

An empty [`PhysicalPlanningContext`] (the [`Default`]) is what every
non-physical-planner caller passes; if such a caller encounters a scalar
subquery, `create_physical_expr` returns a `not_impl_err`.

[`Expr::ScalarSubquery`]: crate::Expr::ScalarSubquery
[`Expr::LambdaVariable`]: crate::Expr::LambdaVariable
[`Subquery`]: crate::logical_plan::Subquery

---

## ScalarSubqueryResults

`struct` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults`

```rust
struct ScalarSubqueryResults
```

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn clear(&self)
fn get(&self, index: SubqueryIndex) -> Option<ScalarValue>
fn new(n: usize) -> Self
fn ptr_eq(this: &Self, other: &Self) -> bool
fn set(&self, index: SubqueryIndex, value: ScalarValue) -> Result<()>
```

Shared results container for uncorrelated scalar subqueries.

Each entry corresponds to one scalar subquery, identified by its index.
Each slot is populated at execution time by `ScalarSubqueryExec`, read by
`ScalarSubqueryExpr` instances that share this container, and cleared when
the plan is reset for re-execution.

---

## SubqueryIndex

`struct` · `datafusion_expr::physical_planning_context::SubqueryIndex`

```rust
struct SubqueryIndex
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn as_usize(self) -> usize
const fn new(index: usize) -> Self
```

Index of a scalar subquery within a [`ScalarSubqueryResults`] container.

---
