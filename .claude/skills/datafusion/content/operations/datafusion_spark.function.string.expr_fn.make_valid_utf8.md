# `datafusion_spark::function::string::expr_fn::make_valid_utf8`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.make_valid_utf8.json).

<a id="op-f4cf4630e40509b04fb2db90"></a>
## make_valid_utf8

`function` · `datafusion_spark::function::string::expr_fn::make_valid_utf8` · datafusion-spark 55.1.0

```rust
fn make_valid_utf8(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the original string if str is a valid UTF-8 string, otherwise returns a new string whose invalid UTF8 byte sequences are replaced using the UNICODE replacement character U+FFFD.
