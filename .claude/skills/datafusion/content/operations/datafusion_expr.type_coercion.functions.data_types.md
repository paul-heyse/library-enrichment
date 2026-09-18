# `datafusion_expr::type_coercion::functions::data_types`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.data_types.json).

<a id="op-c0f854369a71378595084033"></a>
## data_types

`function` · `datafusion_expr::type_coercion::functions::data_types` · datafusion-expr 55.1.0

```rust
fn data_types(function_name: impl AsRef<str>, current_types: &[arrow::datatypes::DataType], signature: &Signature) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

Source: `src/type_coercion/functions.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Performs type coercion for function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.
