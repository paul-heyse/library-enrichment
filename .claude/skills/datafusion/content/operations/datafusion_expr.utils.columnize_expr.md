# `datafusion_expr::utils::columnize_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.columnize_expr.json).

<a id="op-c5f32dcaba2add07cb13e432"></a>
## columnize_expr

`function` · `datafusion_expr::utils::columnize_expr` · datafusion-expr 55.1.0

```rust
fn columnize_expr(e: Expr, input: &LogicalPlan) -> datafusion_common::Result<Expr>
```

Source: `src/utils.rs:882`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert an expression into Column expression if it's already provided as input plan.

For example, it rewrites:

```text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), sum(col("c2"))?
```

Into:

```text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), col("SUM(c2)")?
```
