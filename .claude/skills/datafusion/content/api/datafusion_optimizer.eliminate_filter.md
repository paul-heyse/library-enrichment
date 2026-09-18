# `datafusion_optimizer::eliminate_filter`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.eliminate_filter.json`](../model/datafusion_optimizer.eliminate_filter.json)

## EliminateFilter

`struct` · `datafusion_optimizer::eliminate_filter::EliminateFilter`

```rust
struct EliminateFilter
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
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.eliminate_filter.EliminateFilter.md).


Optimization rule that eliminate the scalar value (true/false/null) filter
with an [LogicalPlan::EmptyRelation]

This saves time in planning and executing the query.
Note that this rule should be applied after simplify expressions optimizer rule.

---
