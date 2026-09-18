# `datafusion_functions::math::monotonicity::cosh_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.cosh_order.json).

<a id="op-413b23d14520ab2c4d702a55"></a>
## cosh_order

`function` · `datafusion_functions::math::monotonicity::cosh_order` · datafusion-functions 55.1.0

```rust
fn cosh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for x ≥ 0 and symmetrically non-increasing for x ≤ 0.
