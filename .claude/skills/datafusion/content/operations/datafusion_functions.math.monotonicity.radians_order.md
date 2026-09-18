# `datafusion_functions::math::monotonicity::radians_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.radians_order.json).

<a id="op-60b8522c7d3c3a3940e2a95a"></a>
## radians_order

`function` · `datafusion_functions::math::monotonicity::radians_order` · datafusion-functions 55.1.0

```rust
fn radians_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:561`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers x.
