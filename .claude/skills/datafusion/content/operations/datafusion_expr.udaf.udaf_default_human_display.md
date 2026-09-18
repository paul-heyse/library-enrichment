# `datafusion_expr::udaf::udaf_default_human_display`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.udaf_default_human_display.json).

<a id="op-8d2a17c3d0c1548e62849221"></a>
## udaf_default_human_display

`function` · `datafusion_expr::udaf::udaf_default_human_display` · datafusion-expr 55.1.0

```rust
fn udaf_default_human_display<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::AggregateFunctionParams) -> datafusion_common::Result<String>
```

Source: `src/udaf.rs:1001`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Encapsulates default implementation of [`AggregateUDFImpl::human_display`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-7058483a5bb0c7b0f33d0444).
