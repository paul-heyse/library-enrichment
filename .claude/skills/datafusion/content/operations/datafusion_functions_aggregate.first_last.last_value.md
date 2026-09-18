# `datafusion_functions_aggregate::first_last::last_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.last_value.json).

<a id="op-91f2e2026f051fa998473650"></a>
## last_value

`function` · `datafusion_functions_aggregate::first_last::last_value` · datafusion-functions-aggregate 55.1.0

```rust
fn last_value(expression: datafusion_expr::Expr, order_by: Vec<datafusion_expr::SortExpr>) -> datafusion_expr::Expr
```

Source: `src/first_last.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Returns the last value in a group of values.
