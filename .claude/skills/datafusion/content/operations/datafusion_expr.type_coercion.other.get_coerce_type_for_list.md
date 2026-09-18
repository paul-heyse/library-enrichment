# `datafusion_expr::type_coercion::other::get_coerce_type_for_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.other.get_coerce_type_for_list.json).

<a id="op-f1c2fe47ef4414bff68dfb76"></a>
## get_coerce_type_for_list

`function` · `datafusion_expr::type_coercion::other::get_coerce_type_for_list` · datafusion-expr 55.1.0

```rust
fn get_coerce_type_for_list(expr_type: &arrow::datatypes::DataType, list_types: &[arrow::datatypes::DataType]) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/other.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Attempts to coerce the types of `list_types` to be comparable with the
`expr_type` for IN list predicates.
Returns the common data type for `expr_type` and `list_types`.

Uses comparison coercion because `x IN (a, b)` is semantically equivalent
to `x = a OR x = b`.
