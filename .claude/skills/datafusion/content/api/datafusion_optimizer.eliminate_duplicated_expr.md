# `datafusion_optimizer::eliminate_duplicated_expr`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.eliminate_duplicated_expr.json`](../model/datafusion_optimizer.eliminate_duplicated_expr.json)

## EliminateDuplicatedExpr

`struct` · `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr`

```rust
struct EliminateDuplicatedExpr
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

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.eliminate_duplicated_expr.EliminateDuplicatedExpr.md).


Optimization rule that eliminate duplicated expr.

---
