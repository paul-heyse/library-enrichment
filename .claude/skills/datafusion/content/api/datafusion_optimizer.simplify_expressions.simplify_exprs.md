# `datafusion_optimizer::simplify_expressions::simplify_exprs`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.simplify_expressions.simplify_exprs.json`](../model/datafusion_optimizer.simplify_expressions.simplify_exprs.json)

## SimplifyExpressions

`struct` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions`

Also reachable as `datafusion_optimizer::simplify_expressions::SimplifyExpressions`

```rust
struct SimplifyExpressions
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
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn supports_rewrite(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.simplify_expressions.simplify_exprs.SimplifyExpressions.md).


Optimizer Pass that simplifies [`LogicalPlan`]s by rewriting
[`Expr`]`s evaluating constants and applying algebraic
simplifications

# Introduction
It uses boolean algebra laws to simplify or reduce the number of terms in expressions.

# Example:
`Filter: b > 2 AND b > 2`
is optimized to
`Filter: b > 2`

[`Expr`]: datafusion_expr::Expr

---
