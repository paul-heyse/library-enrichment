# `datafusion_functions::math::monotonicity::ln_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.ln_order.json).

<a id="op-1dd1d1078ca2af9f1eb85a5e"></a>
## ln_order

`function` · `datafusion_functions::math::monotonicity::ln_order` · datafusion-functions 55.1.0

```rust
fn ln_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for x ≥ 0, undefined otherwise.
