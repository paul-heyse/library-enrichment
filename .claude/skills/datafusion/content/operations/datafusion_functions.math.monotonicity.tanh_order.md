# `datafusion_functions::math::monotonicity::tanh_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.tanh_order.json).

<a id="op-668aa67b957106cfdb3cec11"></a>
## tanh_order

`function` · `datafusion_functions::math::monotonicity::tanh_order` · datafusion-functions 55.1.0

```rust
fn tanh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
