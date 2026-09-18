# `datafusion_optimizer::eliminate_group_by_constant`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.eliminate_group_by_constant.json`](../model/datafusion_optimizer.eliminate_group_by_constant.json)

## EliminateGroupByConstant

`struct` · `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant`

```rust
struct EliminateGroupByConstant
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

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.eliminate_group_by_constant.EliminateGroupByConstant.md).


Optimizer rule that removes constant expressions from `GROUP BY` clause
and places additional projection on top of aggregation, to preserve
original schema

---
