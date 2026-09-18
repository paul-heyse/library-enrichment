# `datafusion_functions::math::monotonicity::atan_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.atan_order.json).

<a id="op-69360ca151dc2af98c49b002"></a>
## atan_order

`function` · `datafusion_functions::math::monotonicity::atan_order` · datafusion-functions 55.1.0

```rust
fn atan_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
