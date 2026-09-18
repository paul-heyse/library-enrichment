# `datafusion_spark::function::string::expr_fn::luhn_check`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.luhn_check.json).

<a id="op-c718f74e4ce081cb1ff628d8"></a>
## luhn_check

`function` · `datafusion_spark::function::string::expr_fn::luhn_check` · datafusion-spark 55.1.0

```rust
fn luhn_check(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns whether the input string of digits is valid according to the Luhn algorithm.
