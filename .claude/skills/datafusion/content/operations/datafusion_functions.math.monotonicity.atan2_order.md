# `datafusion_functions::math::monotonicity::atan2_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.atan2_order.json).

<a id="op-8c69c5974a3bcc0daf1389b2"></a>
## atan2_order

`function` · `datafusion_functions::math::monotonicity::atan2_order` · datafusion-functions 55.1.0

```rust
fn atan2_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Order depends on the quadrant.
