# `datafusion_spark::function::json::expr_fn::json_tuple`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.json.expr_fn.json_tuple.json).

<a id="op-fe980c38d70abed3ae2b73b2"></a>
## json_tuple

`function` · `datafusion_spark::function::json::expr_fn::json_tuple` · datafusion-spark 55.1.0

```rust
fn json_tuple(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/function/json/mod.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Extracts top-level fields from a JSON string and returns them as a struct.
