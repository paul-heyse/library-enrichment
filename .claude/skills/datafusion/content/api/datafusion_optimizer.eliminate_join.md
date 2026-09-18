# `datafusion_optimizer::eliminate_join`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.eliminate_join.json`](../model/datafusion_optimizer.eliminate_join.json)

## EliminateJoin

`struct` · `datafusion_optimizer::eliminate_join::EliminateJoin`

```rust
struct EliminateJoin
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.eliminate_join.EliminateJoin.md).


Rewrites an inner join to a semi join when one input only filters the
other, removes an outer join whose non-preserved side is unused and cannot
multiply the preserved side's rows, and replaces an always-false inner join
with an empty relation.

---
