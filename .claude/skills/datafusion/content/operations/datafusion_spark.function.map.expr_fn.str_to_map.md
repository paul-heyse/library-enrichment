# `datafusion_spark::function::map::expr_fn::str_to_map`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.map.expr_fn.str_to_map.json).

<a id="op-2e074ba841144b6a4cccf898"></a>
## str_to_map

`function` · `datafusion_spark::function::map::expr_fn::str_to_map` · datafusion-spark 55.1.0

```rust
fn str_to_map(text: datafusion_expr::Expr, pair_delim: datafusion_expr::Expr, key_value_delim: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/map/mod.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Creates a map after splitting the text into key/value pairs using delimiters.
