# `datafusion_expr::utils::split_binary`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.split_binary.json).

<a id="op-ce12fe235114ce9cc0c42eaf"></a>
## split_binary

`function` · `datafusion_expr::utils::split_binary` · datafusion-expr 55.1.0

```rust
fn split_binary(expr: &Expr, op: Operator) -> Vec<&Expr>
```

Source: `src/utils.rs:1257`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Splits an binary operator tree [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) such as `A <OP> B <OP> C` => `[A, B, C]`

See [`split_binary_owned`](../operations/datafusion_expr.utils.split_binary_owned.md#op-3e497ad1686197c52df82b23) for more details and an example.
