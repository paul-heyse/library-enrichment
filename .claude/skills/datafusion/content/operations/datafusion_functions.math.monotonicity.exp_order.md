# `datafusion_functions::math::monotonicity::exp_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.exp_order.json).

<a id="op-8b288fa574c37db51c86a63a"></a>
## exp_order

`function` · `datafusion_functions::math::monotonicity::exp_order` · datafusion-functions 55.1.0

```rust
fn exp_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
