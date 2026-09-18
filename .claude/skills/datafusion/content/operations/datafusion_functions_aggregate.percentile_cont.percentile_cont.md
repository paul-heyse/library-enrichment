# `datafusion_functions_aggregate::percentile_cont::percentile_cont`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.percentile_cont.percentile_cont.json).

<a id="op-e8c4b1a00db60b7b89655067"></a>
## percentile_cont

`function` · `datafusion_functions_aggregate::percentile_cont::percentile_cont` · datafusion-functions-aggregate 55.1.0

```rust
fn percentile_cont(order_by: datafusion_expr::expr::Sort, percentile: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/percentile_cont.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Computes the exact percentile continuous of a set of numbers
