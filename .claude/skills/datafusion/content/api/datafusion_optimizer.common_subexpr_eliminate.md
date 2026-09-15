# `datafusion_optimizer::common_subexpr_eliminate`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.common_subexpr_eliminate.json`](../model/datafusion_optimizer.common_subexpr_eliminate.json)

## CommonSubexprEliminate

`struct` · `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate`

```rust
struct CommonSubexprEliminate
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

Performs Common Sub-expression Elimination optimization.

This optimization improves query performance by computing expressions that
appear more than once and reusing those results rather than re-computing the
same value

Currently only common sub-expressions within a single `LogicalPlan` are
eliminated.

# Example

Given a projection that computes the same expensive expression
multiple times such as parsing as string as a date with `to_date` twice:

```text
ProjectionExec(expr=[extract (day from to_date(c1)), extract (year from to_date(c1))])
```

This optimization will rewrite the plan to compute the common expression once
using a new `ProjectionExec` and then rewrite the original expressions to
refer to that new column.

```text
ProjectionExec(exprs=[extract (day from new_col), extract (year from new_col)]) <-- reuse here
  ProjectionExec(exprs=[to_date(c1) as new_col]) <-- compute to_date once
```

---
