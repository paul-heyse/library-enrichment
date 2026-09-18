# `datafusion_functions::math::monotonicity::floor_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.floor_order.json).

<a id="op-28266a1752bd356cd8e7d98f"></a>
## floor_order

`function` · `datafusion_functions::math::monotonicity::floor_order` · datafusion-functions 55.1.0

```rust
fn floor_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
