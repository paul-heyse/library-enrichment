# `datafusion_spark::function::string::expr_fn::ilike`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.ilike.json).

<a id="op-4af74175fcff0607ae6fe0e8"></a>
## ilike

`function` · `datafusion_spark::function::string::expr_fn::ilike` · datafusion-spark 55.1.0

```rust
fn ilike(str: datafusion_expr::Expr, pattern: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns true if str matches pattern (case insensitive).
