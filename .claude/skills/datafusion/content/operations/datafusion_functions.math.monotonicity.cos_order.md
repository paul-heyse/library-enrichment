# `datafusion_functions::math::monotonicity::cos_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.cos_order.json).

<a id="op-41f2d2868c3ec51b16a67d19"></a>
## cos_order

`function` · `datafusion_functions::math::monotonicity::cos_order` · datafusion-functions 55.1.0

```rust
fn cos_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-increasing on \[0, π\] and then non-decreasing on \[π, 2π\].
This pattern repeats periodically with a period of 2π.
