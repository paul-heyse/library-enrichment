# `datafusion_functions_aggregate::first_last::first_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.first_value.json).

<a id="op-f040cb8ef02e59edb99c4614"></a>
## first_value

`function` · `datafusion_functions_aggregate::first_last::first_value` · datafusion-functions-aggregate 55.1.0

```rust
fn first_value(expression: datafusion_expr::Expr, order_by: Vec<datafusion_expr::SortExpr>) -> datafusion_expr::Expr
```

Source: `src/first_last.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Returns the first value in a group of values.
