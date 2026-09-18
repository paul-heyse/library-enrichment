# `datafusion_functions::math::monotonicity::acosh_order`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.monotonicity.acosh_order.json).

<a id="op-bffb38cd8cf2fb52c7c11cf5"></a>
## acosh_order

`function` · `datafusion_functions::math::monotonicity::acosh_order` · datafusion-functions 55.1.0

```rust
fn acosh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Source: `src/math/monotonicity.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Non-decreasing for x ≥ 1, undefined otherwise.
