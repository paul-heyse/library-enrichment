# `datafusion_expr::udaf::udaf_default_schema_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.udaf_default_schema_name.json).

<a id="op-bfc317db0cdd97bac4157014"></a>
## udaf_default_schema_name

`function` · `datafusion_expr::udaf::udaf_default_schema_name` · datafusion-expr 55.1.0

```rust
fn udaf_default_schema_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::AggregateFunctionParams) -> datafusion_common::Result<String>
```

Source: `src/udaf.rs:947`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Encapsulates default implementation of [`AggregateUDFImpl::schema_name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-2730cda6b3e779ac3cd35ede).
