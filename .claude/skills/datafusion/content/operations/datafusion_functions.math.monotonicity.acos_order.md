# `datafusion_functions::math::monotonicity::acos_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.acos_order.json).

<a id="op-d09d8c4941901cc434bab5fd"></a>
## acos_order

`function` · `datafusion_functions::math::monotonicity::acos_order` · datafusion-functions 55.1.0

```rust
fn acos_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-increasing on the interval \[−1, 1\], undefined otherwise.
