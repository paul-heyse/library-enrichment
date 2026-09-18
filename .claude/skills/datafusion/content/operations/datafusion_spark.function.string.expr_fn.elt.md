# `datafusion_spark::function::string::expr_fn::elt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.elt.json).

<a id="op-db17d6aca160bf79ffc411b8"></a>
## elt

`function` · `datafusion_spark::function::string::expr_fn::elt` · datafusion-spark 55.1.0

```rust
fn elt(select_col: datafusion_expr::Expr, arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, argn: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the n-th input (1-indexed), e.g. returns 2nd input when n is 2. The function returns NULL if the index is 0 or exceeds the length of the array.
