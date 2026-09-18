# `datafusion_functions::math::monotonicity::log10_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.log10_order.json).

<a id="op-01976f1d5176cece6f998a5f"></a>
## log10_order

`function` · `datafusion_functions::math::monotonicity::log10_order` · datafusion-functions 55.1.0

```rust
fn log10_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for x ≥ 0, undefined otherwise.
