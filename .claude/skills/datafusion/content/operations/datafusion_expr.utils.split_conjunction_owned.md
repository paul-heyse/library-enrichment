# `datafusion_expr::utils::split_conjunction_owned`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.split_conjunction_owned.json).

<a id="op-5340352c3d5c39486d2c6946"></a>
## split_conjunction_owned

`function` · `datafusion_expr::utils::split_conjunction_owned` · datafusion-expr 55.1.0

```rust
fn split_conjunction_owned(expr: Expr) -> Vec<Expr>
```

Source: `src/utils.rs:1207`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Splits an owned conjunctive [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) such as `A AND B AND C` => `[A, B, C]`

This is often used to "split" filter expressions such as `col1 = 5
AND col2 = 10` into [`col1 = 5`, `col2 = 10`];

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::split_conjunction_owned;
// a=1 AND b=2
let expr = col("a").eq(lit(1)).and(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use split_conjunction_owned to split them
assert_eq!(split_conjunction_owned(expr), split);
```
