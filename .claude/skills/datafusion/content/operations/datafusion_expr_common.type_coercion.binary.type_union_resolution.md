# `datafusion_expr_common::type_coercion::binary::type_union_resolution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.type_union_resolution.json).

<a id="op-942560bb471d38e06413c7ac"></a>
## type_union_resolution

`function` · `datafusion_expr_common::type_coercion::binary::type_union_resolution` · datafusion-expr-common 55.1.0

```rust
fn type_union_resolution(data_types: &[arrow::datatypes::DataType]) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:638`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coerce dissimilar data types to a single data type.
ARRAY literals, VALUES, COALESCE, and array concatenation are examples
of contexts that use this function.
See <https://www.postgresql.org/docs/current/typeconv-union-case.html> for more information.
The rules in the document provide a clue, but adhering strictly to them doesn't precisely
align with the behavior of Postgres. Therefore, we've made slight adjustments to the rules
to better match the behavior of both Postgres and DuckDB. For example, we expect adjusted
decimal precision and scale when coercing decimal types.

This function doesn't preserve correct field name and nullability for the struct type, we only care about data type.

Returns Option because we might want to continue on the code even if the data types are not coercible to the common type
