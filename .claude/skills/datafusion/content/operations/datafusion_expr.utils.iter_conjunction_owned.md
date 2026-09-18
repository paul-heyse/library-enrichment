# `datafusion_expr::utils::iter_conjunction_owned`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.iter_conjunction_owned.json).

<a id="op-b4a2c3cc1d4c515f4ca97d30"></a>
## iter_conjunction_owned

`function` · `datafusion_expr::utils::iter_conjunction_owned` · datafusion-expr 55.1.0

```rust
fn iter_conjunction_owned(expr: Expr) -> impl Iterator<Item = Expr>
```

Source: `src/utils.rs:1168`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Iterate parts in a conjunctive [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`](../operations/datafusion_expr.utils.split_conjunction_owned.md#op-5340352c3d5c39486d2c6946) for more details and an example.
