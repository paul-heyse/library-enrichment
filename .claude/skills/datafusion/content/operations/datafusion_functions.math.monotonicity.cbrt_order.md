# `datafusion_functions::math::monotonicity::cbrt_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.cbrt_order.json).

<a id="op-f1a46a3cb78832654a467ed3"></a>
## cbrt_order

`function` · `datafusion_functions::math::monotonicity::cbrt_order` · datafusion-functions 55.1.0

```rust
fn cbrt_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
