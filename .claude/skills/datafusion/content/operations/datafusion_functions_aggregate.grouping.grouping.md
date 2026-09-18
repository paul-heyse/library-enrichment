# `datafusion_functions_aggregate::grouping::grouping`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.grouping.grouping.json).

<a id="op-071c5b279ec4b14175e87e7a"></a>
## grouping

`function` · `datafusion_functions_aggregate::grouping::grouping` · datafusion-functions-aggregate 55.1.0

```rust
fn grouping(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/grouping.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Returns 1 if the data is aggregated across the specified column or 0 for not aggregated in the result set.
