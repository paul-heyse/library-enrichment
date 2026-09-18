# `datafusion_functions::math::monotonicity::asin_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.asin_order.json).

<a id="op-597f6df7f9b79ae0a8e22bec"></a>
## asin_order

`function` · `datafusion_functions::math::monotonicity::asin_order` · datafusion-functions 55.1.0

```rust
fn asin_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing on the interval \[−1, 1\], undefined otherwise.
