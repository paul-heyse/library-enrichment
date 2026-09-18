# `datafusion_functions::math::monotonicity::sin_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.sin_order.json).

<a id="op-10e0099ec33f2b455f752893"></a>
## sin_order

`function` · `datafusion_functions::math::monotonicity::sin_order` · datafusion-functions 55.1.0

```rust
fn sin_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing on \[0, π\] and then non-increasing on \[π, 2π\].
This pattern repeats periodically with a period of 2π.
