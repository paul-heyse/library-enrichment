# `datafusion_expr::udaf::udaf_default_display_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.udaf_default_display_name.json).

<a id="op-ea420d8e986da6173d670c50"></a>
## udaf_default_display_name

`function` · `datafusion_expr::udaf::udaf_default_display_name` · datafusion-expr 55.1.0

```rust
fn udaf_default_display_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::AggregateFunctionParams) -> datafusion_common::Result<String>
```

Source: `src/udaf.rs:1100`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Encapsulates default implementation of [`AggregateUDFImpl::display_name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-1bb6dfbde26e11e415def938).
