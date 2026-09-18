# `datafusion_functions_aggregate::approx_percentile_cont::approx_percentile_cont`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_percentile_cont.approx_percentile_cont.json).

<a id="op-fd70f5f98b51f7ff9aaf87a4"></a>
## approx_percentile_cont

`function` · `datafusion_functions_aggregate::approx_percentile_cont::approx_percentile_cont` · datafusion-functions-aggregate 55.1.0

```rust
fn approx_percentile_cont(order_by: datafusion_expr::expr::Sort, percentile: datafusion_expr::Expr, centroids: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/approx_percentile_cont.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Computes the approximate percentile continuous of a set of numbers
