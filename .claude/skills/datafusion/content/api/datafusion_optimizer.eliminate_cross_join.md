# `datafusion_optimizer::eliminate_cross_join`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.eliminate_cross_join.json`](../model/datafusion_optimizer.eliminate_cross_join.json)

## EliminateCrossJoin

`struct` · `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin`

```rust
struct EliminateCrossJoin
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
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

---
