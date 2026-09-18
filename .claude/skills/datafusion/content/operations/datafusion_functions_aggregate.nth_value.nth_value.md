# `datafusion_functions_aggregate::nth_value::nth_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.nth_value.nth_value.json).

<a id="op-80acd8524bfd4aae3f5b31cf"></a>
## nth_value

`function` · `datafusion_functions_aggregate::nth_value::nth_value` · datafusion-functions-aggregate 55.1.0

```rust
fn nth_value(expr: datafusion_expr::Expr, n: i64, order_by: Vec<datafusion_expr::SortExpr>) -> datafusion_expr::Expr
```

Source: `src/nth_value.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Returns the nth value in a group of values.
