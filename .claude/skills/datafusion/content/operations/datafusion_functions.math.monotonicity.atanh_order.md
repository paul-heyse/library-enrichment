# `datafusion_functions::math::monotonicity::atanh_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.atanh_order.json).

<a id="op-468d093f4c63a99f5a3b12e4"></a>
## atanh_order

`function` · `datafusion_functions::math::monotonicity::atanh_order` · datafusion-functions 55.1.0

```rust
fn atanh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing on the interval \[−1, 1\], undefined otherwise.
