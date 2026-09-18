# `datafusion_functions::math::monotonicity::ceil_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.ceil_order.json).

<a id="op-60dc9cccfbcb9197471f717d"></a>
## ceil_order

`function` · `datafusion_functions::math::monotonicity::ceil_order` · datafusion-functions 55.1.0

```rust
fn ceil_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
