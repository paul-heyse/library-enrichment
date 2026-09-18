# `datafusion_expr::utils::split_conjunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.split_conjunction.json).

<a id="op-5791a4653b80e79ef7dc7ad7"></a>
## split_conjunction

`function` · `datafusion_expr::utils::split_conjunction` · datafusion-expr 55.1.0

```rust
fn split_conjunction(expr: &Expr) -> Vec<&Expr>
```

Source: `src/utils.rs:1119`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Splits a conjunctive [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`](../operations/datafusion_expr.utils.split_conjunction_owned.md#op-5340352c3d5c39486d2c6946) for more details and an example.
