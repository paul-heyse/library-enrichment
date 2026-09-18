# `datafusion_expr::type_coercion::other::get_coerce_type_for_case_when`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.other.get_coerce_type_for_case_when.json).

<a id="op-afe15e0da9c397d9788f44cf"></a>
## get_coerce_type_for_case_when

`function` · `datafusion_expr::type_coercion::other::get_coerce_type_for_case_when` · datafusion-expr 55.1.0

```rust
fn get_coerce_type_for_case_when(when_types: &[arrow::datatypes::DataType], case_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/other.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Find a common coerceable type for `CASE expr WHEN val1 WHEN val2 ...`
conditions. Returns the common type for `case_type` and all `when_types`.

Uses comparison coercion because `CASE expr WHEN val` is semantically
equivalent to `expr = val`.
