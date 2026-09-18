# `datafusion_optimizer::scalar_subquery_to_join`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.scalar_subquery_to_join.json`](../model/datafusion_optimizer.scalar_subquery_to_join.json)

## ScalarSubqueryToJoin

`struct` · `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin`

```rust
struct ScalarSubqueryToJoin
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

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.scalar_subquery_to_join.ScalarSubqueryToJoin.md).


Optimizer rule that rewrites scalar subquery filters to joins and places an
additional projection on top of the filter to preserve the original schema.

When [`datafusion_common::config::OptimizerOptions::enable_physical_uncorrelated_scalar_subquery`] is
true (the default), only *correlated* scalar subqueries are rewritten here;
uncorrelated ones are left for physical execution via `ScalarSubqueryExec`.
When the option is false, all scalar subqueries — correlated and
uncorrelated — are rewritten to left joins by this rule.

---
