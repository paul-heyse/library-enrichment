# `datafusion_spark::function::math::expr_fn::factorial`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.expr_fn.factorial.json).

<a id="op-7c435a11da109b3348146141"></a>
## factorial

`function` · `datafusion_spark::function::math::expr_fn::factorial` · datafusion-spark 55.1.0

```rust
fn factorial(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/math/mod.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the factorial of expr. expr is [0..20]. Otherwise, null.
