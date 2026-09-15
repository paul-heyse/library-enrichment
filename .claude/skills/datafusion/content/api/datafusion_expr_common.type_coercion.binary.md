# `datafusion_expr_common::type_coercion::binary`

Crate `datafusion-expr-common` · 12 public items · structured records in [`model/datafusion_expr_common.type_coercion.binary.json`](../model/datafusion_expr_common.type_coercion.binary.json)

## binary_numeric_coercion

`function` · `datafusion_expr_common::type_coercion::binary::binary_numeric_coercion`

Also reachable as `datafusion_expr::binary::binary_numeric_coercion`, `datafusion_expr::type_coercion::binary::binary_numeric_coercion`

```rust
fn binary_numeric_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Coerce `lhs_type` and `rhs_type` to a common type where both are numeric

---

## binary_to_string_coercion

`function` · `datafusion_expr_common::type_coercion::binary::binary_to_string_coercion`

Also reachable as `datafusion_expr::binary::binary_to_string_coercion`, `datafusion_expr::type_coercion::binary::binary_to_string_coercion`

```rust
fn binary_to_string_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Coercion rules for binary (Binary/LargeBinary) to string (Utf8/LargeUtf8):
If one argument is binary and the other is a string then coerce to string
(e.g. for `like`)

---

## comparison_coercion

`function` · `datafusion_expr_common::type_coercion::binary::comparison_coercion`

Also reachable as `datafusion_expr::binary::comparison_coercion`, `datafusion_expr::type_coercion::binary::comparison_coercion`

```rust
fn comparison_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Coerce `lhs_type` and `rhs_type` to a common type for comparison
contexts — any context where two values are compared rather than
unified. This includes binary comparison operators, IN lists,
CASE/WHEN conditions, and BETWEEN.

When the two types differ, this function determines the common type
to cast to.

# Numeric comparisons

The lower precision type is widened to the higher precision type
(e.g., `Int32` vs `Int64` → `Int64`).

# Numeric / String comparisons

Prefers the numeric type (e.g., `'2' > 1` where `1` is `Int32` coerces
`'2'` to `Int32`).

For type unification contexts (UNION, CASE THEN/ELSE), use
[`type_union_coercion`] instead.

---

## decimal_coercion

`function` · `datafusion_expr_common::type_coercion::binary::decimal_coercion`

Also reachable as `datafusion_expr::binary::decimal_coercion`, `datafusion_expr::type_coercion::binary::decimal_coercion`

```rust
fn decimal_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Decimal coercion rules.

---

## like_coercion

`function` · `datafusion_expr_common::type_coercion::binary::like_coercion`

Also reachable as `datafusion_expr::binary::like_coercion`, `datafusion_expr::type_coercion::binary::like_coercion`

```rust
fn like_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Coercion rules for like operations.
This is a union of string coercion rules, dictionary coercion rules, and REE coercion rules
Note: list_coercion is intentionally NOT included here because LIKE is a string pattern
matching operation and is not supported for nested types (List, Struct, etc.)

---

## regex_coercion

`function` · `datafusion_expr_common::type_coercion::binary::regex_coercion`

Also reachable as `datafusion_expr::binary::regex_coercion`, `datafusion_expr::type_coercion::binary::regex_coercion`

```rust
fn regex_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Coercion rules for regular expression comparison operations.
This is a union of string coercion rules, dictionary coercion rules, and REE coercion rules.

---

## string_coercion

`function` · `datafusion_expr_common::type_coercion::binary::string_coercion`

Also reachable as `datafusion_expr::binary::string_coercion`, `datafusion_expr::type_coercion::binary::string_coercion`

```rust
fn string_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Coercion rules for string view types (Utf8/LargeUtf8/Utf8View):
If at least one argument is a string view, we coerce to string view
based on the observation that StringArray to StringViewArray is cheap but not vice versa.

Between Utf8 and LargeUtf8, we coerce to LargeUtf8.

---

## try_type_union_resolution

`function` · `datafusion_expr_common::type_coercion::binary::try_type_union_resolution`

Also reachable as `datafusion_expr::binary::try_type_union_resolution`, `datafusion_expr::type_coercion::binary::try_type_union_resolution`

```rust
fn try_type_union_resolution(data_types: &[arrow::datatypes::DataType]) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

Handle type union resolution including struct type and others.

---

## try_type_union_resolution_with_struct

`function` · `datafusion_expr_common::type_coercion::binary::try_type_union_resolution_with_struct`

Also reachable as `datafusion_expr::binary::try_type_union_resolution_with_struct`, `datafusion_expr::type_coercion::binary::try_type_union_resolution_with_struct`

```rust
fn try_type_union_resolution_with_struct(data_types: &[arrow::datatypes::DataType]) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

---

## type_union_coercion

`function` · `datafusion_expr_common::type_coercion::binary::type_union_coercion`

Also reachable as `datafusion_expr::binary::type_union_coercion`, `datafusion_expr::type_coercion::binary::type_union_coercion`

```rust
fn type_union_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Coerce `lhs_type` and `rhs_type` to a common type for type unification
contexts — where two values must be brought to a common type but are not
being compared. Examples: UNION, CASE THEN/ELSE branches, NVL2. For other
contexts, [`comparison_coercion`] should typically be used instead.

The intuition is that we try to find the "widest" type that can represent
all values from both sides. When one side is a string and the other is
numeric, this prefers strings because every number has a textual
representation but not every string can be parsed as a number (e.g., `SELECT
1 UNION SELECT 'a'` coerces both sides to a string). This is in contrast to
[`comparison_coercion`], which prefers numeric types so that ordering and
equality follow numeric semantics.

---

## type_union_resolution

`function` · `datafusion_expr_common::type_coercion::binary::type_union_resolution`

Also reachable as `datafusion_expr::binary::type_union_resolution`, `datafusion_expr::type_coercion::binary::type_union_resolution`

```rust
fn type_union_resolution(data_types: &[arrow::datatypes::DataType]) -> Option<arrow::datatypes::DataType>
```

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

---

## BinaryTypeCoercer

`struct` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer`

Also reachable as `datafusion_expr::binary::BinaryTypeCoercer`, `datafusion_expr::type_coercion::binary::BinaryTypeCoercer`

```rust
struct BinaryTypeCoercer<'a>
```

**Methods** (6)

```rust
fn get_input_types(&'a self) -> Result<(DataType, DataType)>
fn get_result_type(&'a self) -> Result<DataType>
fn new(lhs: &'a DataType, op: &'a Operator, rhs: &'a DataType) -> Self
fn set_lhs_spans(&mut self, spans: Spans)
fn set_op_spans(&mut self, spans: Spans)
fn set_rhs_spans(&mut self, spans: Spans)
```

Provides type information about a binary expression, coercing different
input types into a sensible output type.

---
