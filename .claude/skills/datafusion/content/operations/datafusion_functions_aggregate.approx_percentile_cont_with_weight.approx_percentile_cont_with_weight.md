# `datafusion_functions_aggregate::approx_percentile_cont_with_weight::approx_percentile_cont_with_weight`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_percentile_cont_with_weight.approx_percentile_cont_with_weight.json).

<a id="op-fcd7c3960cea729996f1db54"></a>
## approx_percentile_cont_with_weight

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::approx_percentile_cont_with_weight` · datafusion-functions-aggregate 55.1.0

```rust
fn approx_percentile_cont_with_weight(order_by: datafusion_expr::expr::Sort, weight: datafusion_expr::Expr, percentile: datafusion_expr::Expr, centroids: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/approx_percentile_cont_with_weight.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Computes the approximate percentile continuous with weight of a set of numbers
