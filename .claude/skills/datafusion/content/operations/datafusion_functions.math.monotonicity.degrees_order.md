# `datafusion_functions::math::monotonicity::degrees_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.degrees_order.json).

<a id="op-24d1f45d56a06f51ae34e3be"></a>
## degrees_order

`function` · `datafusion_functions::math::monotonicity::degrees_order` · datafusion-functions 55.1.0

```rust
fn degrees_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing function that converts radians to degrees.
