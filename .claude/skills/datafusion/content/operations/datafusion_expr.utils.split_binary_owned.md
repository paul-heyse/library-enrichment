# `datafusion_expr::utils::split_binary_owned`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.split_binary_owned.json).

<a id="op-3e497ad1686197c52df82b23"></a>
## split_binary_owned

`function` · `datafusion_expr::utils::split_binary_owned` · datafusion-expr 55.1.0

```rust
fn split_binary_owned(expr: Expr, op: Operator) -> Vec<Expr>
```

Source: `src/utils.rs:1230`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Splits an owned binary operator tree [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) such as `A <OP> B <OP> C` => `[A, B, C]`

This is often used to "split" expressions such as `col1 = 5
AND col2 = 10` into [`col1 = 5`, `col2 = 10`];

# Example
```
# use datafusion_expr::{col, lit, Operator};
# use datafusion_expr::utils::split_binary_owned;
# use std::ops::Add;
// a=1 + b=2
let expr = col("a").eq(lit(1)).add(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use split_binary_owned to split them
assert_eq!(split_binary_owned(expr, Operator::Plus), split);
```
