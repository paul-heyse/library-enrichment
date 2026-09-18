# `datafusion_expr::type_coercion::functions::data_types_with_scalar_udf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.data_types_with_scalar_udf.json).

<a id="op-09d5a828f77bd1bb13b802da"></a>
## data_types_with_scalar_udf

`function` · `datafusion_expr::type_coercion::functions::data_types_with_scalar_udf` · datafusion-expr 55.1.0

```rust
fn data_types_with_scalar_udf(current_types: &[arrow::datatypes::DataType], func: &ScalarUDF) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

Source: `src/type_coercion/functions.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Performs type coercion for scalar function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.
