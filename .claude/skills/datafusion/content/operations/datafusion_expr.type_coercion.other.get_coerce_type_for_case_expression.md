# `datafusion_expr::type_coercion::other::get_coerce_type_for_case_expression`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.other.get_coerce_type_for_case_expression.json).

<a id="op-e73674bc80676f8f8c47dc47"></a>
## get_coerce_type_for_case_expression

`function` · `datafusion_expr::type_coercion::other::get_coerce_type_for_case_expression` · datafusion-expr 55.1.0

```rust
fn get_coerce_type_for_case_expression(then_types: &[arrow::datatypes::DataType], else_type: Option<&arrow::datatypes::DataType>) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/other.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Find a common coerceable type for CASE THEN/ELSE result expressions.
Returns the common data type for `then_types` and `else_type`.

Uses type union coercion because the result branches must be brought to a
common type (like UNION), not compared.
