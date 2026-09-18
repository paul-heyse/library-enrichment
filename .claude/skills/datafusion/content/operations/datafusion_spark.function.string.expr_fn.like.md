# `datafusion_spark::function::string::expr_fn::like`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.like.json).

<a id="op-313cf678006583611073567d"></a>
## like

`function` · `datafusion_spark::function::string::expr_fn::like` · datafusion-spark 55.1.0

```rust
fn like(str: datafusion_expr::Expr, pattern: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns true if str matches pattern (case sensitive).
