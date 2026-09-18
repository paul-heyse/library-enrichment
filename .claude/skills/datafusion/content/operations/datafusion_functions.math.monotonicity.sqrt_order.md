# `datafusion_functions::math::monotonicity::sqrt_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.sqrt_order.json).

<a id="op-b65a3448b2ae62918753c541"></a>
## sqrt_order

`function` · `datafusion_functions::math::monotonicity::sqrt_order` · datafusion-functions 55.1.0

```rust
fn sqrt_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for x ≥ 0, undefined otherwise.
