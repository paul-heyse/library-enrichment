# `datafusion_physical_expr::intervals::utils::is_operator_supported`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.utils.is_operator_supported.json).

<a id="op-4184cf5217605ad7a719c227"></a>
## is_operator_supported

`function` · `datafusion_physical_expr::intervals::utils::is_operator_supported` · datafusion-physical-expr 55.1.0

```rust
fn is_operator_supported(op: &datafusion_expr::Operator) -> bool
```

Source: `src/intervals/utils.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Indicates whether interval arithmetic is supported for the given operator.
