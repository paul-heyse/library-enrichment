# `datafusion_functions::math::monotonicity::sinh_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.sinh_order.json).

<a id="op-27c8db2fecd58631857b102c"></a>
## sinh_order

`function` · `datafusion_functions::math::monotonicity::sinh_order` · datafusion-functions 55.1.0

```rust
fn sinh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for all real numbers.
