# `datafusion_optimizer::push_down_limit`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.push_down_limit.json`](../model/datafusion_optimizer.push_down_limit.json)

## PushDownLimit

`struct` · `datafusion_optimizer::push_down_limit::PushDownLimit`

```rust
struct PushDownLimit
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

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.push_down_limit.PushDownLimit.md).


Optimization rule that tries to push down `LIMIT`.

---
