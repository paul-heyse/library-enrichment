# `datafusion_optimizer::decorrelate_lateral_join`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.decorrelate_lateral_join.json`](../model/datafusion_optimizer.decorrelate_lateral_join.json)

## DecorrelateLateralJoin

`struct` · `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin`

```rust
struct DecorrelateLateralJoin
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

Optimizer rule for rewriting lateral joins to joins

---
