# `datafusion_expr::type_coercion::functions::fields_with_aggregate_udf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.fields_with_aggregate_udf.json).

<a id="op-f3dee69d278e027134177be1"></a>
## fields_with_aggregate_udf

`function` · `datafusion_expr::type_coercion::functions::fields_with_aggregate_udf` · datafusion-expr 55.1.0

```rust
fn fields_with_aggregate_udf(current_fields: &[arrow::datatypes::FieldRef], func: &AggregateUDF) -> datafusion_common::Result<Vec<arrow::datatypes::FieldRef>>
```

Source: `src/type_coercion/functions.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Performs type coercion for aggregate function arguments.

Returns the fields to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.
