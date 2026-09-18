# `datafusion_expr::udaf::udaf_default_window_function_display_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.udaf_default_window_function_display_name.json).

<a id="op-9d7ece999fd80b805c520ea0"></a>
## udaf_default_window_function_display_name

`function` · `datafusion_expr::udaf::udaf_default_window_function_display_name` · datafusion-expr 55.1.0

```rust
fn udaf_default_window_function_display_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::WindowFunctionParams) -> datafusion_common::Result<String>
```

Source: `src/udaf.rs:1142`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Encapsulates default implementation of [`AggregateUDFImpl::window_function_display_name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-1a19753a1d07d2da97a2dedc).
