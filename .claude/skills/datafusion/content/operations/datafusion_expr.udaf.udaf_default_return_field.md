# `datafusion_expr::udaf::udaf_default_return_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.udaf_default_return_field.json).

<a id="op-68368c697ec178d5ba9a8d4d"></a>
## udaf_default_return_field

`function` · `datafusion_expr::udaf::udaf_default_return_field` · datafusion-expr 55.1.0

```rust
fn udaf_default_return_field<F: AggregateUDFImpl + ?Sized>(func: &F, arg_fields: &[arrow::datatypes::FieldRef]) -> datafusion_common::Result<arrow::datatypes::FieldRef>
```

Source: `src/udaf.rs:1201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Encapsulates default implementation of [`AggregateUDFImpl::return_field`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-bc99f6392eb81fae9a622a87).
