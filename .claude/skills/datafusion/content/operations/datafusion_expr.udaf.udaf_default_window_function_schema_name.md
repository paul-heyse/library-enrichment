# `datafusion_expr::udaf::udaf_default_window_function_schema_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.udaf_default_window_function_schema_name.json).

<a id="op-c2422298e72bed5c72a02e8b"></a>
## udaf_default_window_function_schema_name

`function` · `datafusion_expr::udaf::udaf_default_window_function_schema_name` · datafusion-expr 55.1.0

```rust
fn udaf_default_window_function_schema_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::WindowFunctionParams) -> datafusion_common::Result<String>
```

Source: `src/udaf.rs:1041`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Encapsulates default implementation of [`AggregateUDFImpl::window_function_schema_name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-2179be90b6fdd3ca93bde667).
