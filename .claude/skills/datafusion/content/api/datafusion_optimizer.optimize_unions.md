# `datafusion_optimizer::optimize_unions`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.optimize_unions.json`](../model/datafusion_optimizer.optimize_unions.json)

## OptimizeUnions

`struct` · `datafusion_optimizer::optimize_unions::OptimizeUnions`

```rust
struct OptimizeUnions
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

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.optimize_unions.OptimizeUnions.md).


An optimization rule that
1. replaces nested unions with a single union.
2. removes unions with a single input.

---
