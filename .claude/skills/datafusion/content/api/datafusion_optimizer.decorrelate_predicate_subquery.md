# `datafusion_optimizer::decorrelate_predicate_subquery`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.decorrelate_predicate_subquery.json`](../model/datafusion_optimizer.decorrelate_predicate_subquery.json)

## DecorrelatePredicateSubquery

`struct` · `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery`

```rust
struct DecorrelatePredicateSubquery
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

Optimizer rule for rewriting predicate(IN/EXISTS) subquery to left semi/anti joins

---
