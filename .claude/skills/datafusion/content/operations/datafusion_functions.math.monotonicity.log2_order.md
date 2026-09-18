# `datafusion_functions::math::monotonicity::log2_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.log2_order.json).

<a id="op-fea95d8c718dcc45639b8ab1"></a>
## log2_order

`function` · `datafusion_functions::math::monotonicity::log2_order` · datafusion-functions 55.1.0

```rust
fn log2_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for x ≥ 0, undefined otherwise.
