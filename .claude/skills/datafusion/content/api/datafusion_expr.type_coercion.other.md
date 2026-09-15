# `datafusion_expr::type_coercion::other`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.type_coercion.other.json`](../model/datafusion_expr.type_coercion.other.json)

## get_coerce_type_for_case_expression

`function` · `datafusion_expr::type_coercion::other::get_coerce_type_for_case_expression`

```rust
fn get_coerce_type_for_case_expression(then_types: &[arrow::datatypes::DataType], else_type: Option<&arrow::datatypes::DataType>) -> Option<arrow::datatypes::DataType>
```

Find a common coerceable type for CASE THEN/ELSE result expressions.
Returns the common data type for `then_types` and `else_type`.

Uses type union coercion because the result branches must be brought to a
common type (like UNION), not compared.

---

## get_coerce_type_for_case_when

`function` · `datafusion_expr::type_coercion::other::get_coerce_type_for_case_when`

```rust
fn get_coerce_type_for_case_when(when_types: &[arrow::datatypes::DataType], case_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Find a common coerceable type for `CASE expr WHEN val1 WHEN val2 ...`
conditions. Returns the common type for `case_type` and all `when_types`.

Uses comparison coercion because `CASE expr WHEN val` is semantically
equivalent to `expr = val`.

---

## get_coerce_type_for_list

`function` · `datafusion_expr::type_coercion::other::get_coerce_type_for_list`

```rust
fn get_coerce_type_for_list(expr_type: &arrow::datatypes::DataType, list_types: &[arrow::datatypes::DataType]) -> Option<arrow::datatypes::DataType>
```

Attempts to coerce the types of `list_types` to be comparable with the
`expr_type` for IN list predicates.
Returns the common data type for `expr_type` and `list_types`.

Uses comparison coercion because `x IN (a, b)` is semantically equivalent
to `x = a OR x = b`.

---
