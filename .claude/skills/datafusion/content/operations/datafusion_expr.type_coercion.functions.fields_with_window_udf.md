# `datafusion_expr::type_coercion::functions::fields_with_window_udf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.fields_with_window_udf.json).

<a id="op-6ac1736b88b4a8cb81716965"></a>
## fields_with_window_udf

`function` · `datafusion_expr::type_coercion::functions::fields_with_window_udf` · datafusion-expr 55.1.0

```rust
fn fields_with_window_udf(current_fields: &[arrow::datatypes::FieldRef], func: &WindowUDF) -> datafusion_common::Result<Vec<arrow::datatypes::FieldRef>>
```

Source: `src/type_coercion/functions.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Performs type coercion for window function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.
