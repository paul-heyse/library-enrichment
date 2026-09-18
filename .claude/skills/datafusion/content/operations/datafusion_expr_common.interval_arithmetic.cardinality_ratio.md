# `datafusion_expr_common::interval_arithmetic::cardinality_ratio`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.interval_arithmetic.cardinality_ratio.json).

<a id="op-110bc7500f164e60a6cf2b87"></a>
## cardinality_ratio

`function` · `datafusion_expr_common::interval_arithmetic::cardinality_ratio` · datafusion-expr-common 55.1.0

```rust
fn cardinality_ratio(initial_interval: &Interval, final_interval: &Interval) -> f64
```

Source: `src/interval_arithmetic.rs:1728`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

This function computes the selectivity of an operation by computing the
cardinality ratio of the given input/output intervals. If this can not be
calculated for some reason, it returns `1.0` meaning fully selective (no
filtering).
