# `datafusion_functions::math::monotonicity::tan_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.tan_order.json).

<a id="op-5d323630f7fc7af2be35b03e"></a>
## tan_order

`function` · `datafusion_functions::math::monotonicity::tan_order` · datafusion-functions 55.1.0

```rust
fn tan_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:680`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing between vertical asymptotes at x = k * π ± π / 2 for any
integer k.
