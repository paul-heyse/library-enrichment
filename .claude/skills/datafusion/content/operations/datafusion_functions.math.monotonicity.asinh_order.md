# `datafusion_functions::math::monotonicity::asinh_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.asinh_order.json).

<a id="op-848527a508c6d15e493bb86c"></a>
## asinh_order

`function` · `datafusion_functions::math::monotonicity::asinh_order` · datafusion-functions 55.1.0

```rust
fn asinh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
