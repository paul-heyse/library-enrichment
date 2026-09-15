# `datafusion_optimizer::eliminate_limit`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.eliminate_limit.json`](../model/datafusion_optimizer.eliminate_limit.json)

## EliminateLimit

`struct` · `datafusion_optimizer::eliminate_limit::EliminateLimit`

```rust
struct EliminateLimit
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
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, datafusion_common::DataFusionError>
fn supports_rewrite(&self) -> bool
```

Optimizer rule to replace `LIMIT 0` or `LIMIT` whose ancestor LIMIT's skip is
greater than or equal to current's fetch

It can cooperate with `propagate_empty_relation` and `limit_push_down`. on a
plan with an empty relation.

This rule also removes OFFSET 0 from the [LogicalPlan]

---
