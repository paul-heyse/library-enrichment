# `datafusion_expr_common::columnar_value`

Crate `datafusion-expr-common` · 1 public items · structured records in [`model/datafusion_expr_common.columnar_value.json`](../model/datafusion_expr_common.columnar_value.json)

## ColumnarValue

`enum` · `datafusion_expr_common::columnar_value::ColumnarValue`

Also reachable as `datafusion::logical_expr::ColumnarValue`, `datafusion::physical_plan::ColumnarValue`, `datafusion_expr::ColumnarValue`, `datafusion_physical_plan::ColumnarValue`, `datafusion_physical_plan::execution_plan::ColumnarValue`

```rust
enum ColumnarValue
```

**Variants**: `Array`, `Scalar`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (8)

```rust
fn cast_to(&self, cast_type: &DataType, cast_options: Option<&CastOptions<'static>>) -> Result<ColumnarValue>
fn create_null_array(num_rows: usize) -> Self
fn data_type(&self) -> DataType
fn into_array(self, num_rows: usize) -> Result<ArrayRef>
fn into_array_of_size(self, num_rows: usize) -> Result<ArrayRef>
fn to_array(&self, num_rows: usize) -> Result<ArrayRef>
fn to_array_of_size(&self, num_rows: usize) -> Result<ArrayRef>
fn values_to_arrays(args: &[ColumnarValue]) -> Result<Vec<ArrayRef>>
```

**via `core::convert::From`**

```rust
fn from(value: ScalarValue) -> Self
fn from(value: ArrayRef) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

The result of evaluating an expression.

[`ColumnarValue::Scalar`] represents a single value repeated any number of
times. This is an important performance optimization for handling values
that do not change across rows.

[`ColumnarValue::Array`] represents a column of data, stored as an  Arrow
[`ArrayRef`]

A slice of `ColumnarValue`s logically represents a table, with each column
having the same number of rows. This means that all `Array`s are the same
length.

# Example

A `ColumnarValue::Array` with an array of 5 elements and a
`ColumnarValue::Scalar` with the value 100

```text
┌──────────────┐
│ ┌──────────┐ │
│ │   "A"    │ │
│ ├──────────┤ │
│ │   "B"    │ │
│ ├──────────┤ │
│ │   "C"    │ │
│ ├──────────┤ │
│ │   "D"    │ │        ┌──────────────┐
│ ├──────────┤ │        │ ┌──────────┐ │
│ │   "E"    │ │        │ │   100    │ │
│ └──────────┘ │        │ └──────────┘ │
└──────────────┘        └──────────────┘

 ColumnarValue::        ColumnarValue::
      Array                 Scalar
```

Logically represents the following table:

| Column 1| Column 2 |
| ------- | -------- |
| A | 100 |
| B | 100 |
| C | 100 |
| D | 100 |
| E | 100 |

# Performance Notes

When implementing functions or operators, it is important to consider the
performance implications of handling scalar values.

Because all functions must handle [`ArrayRef`], it is
convenient to convert [`ColumnarValue::Scalar`]s using
[`Self::into_array`]. For example,  [`ColumnarValue::values_to_arrays`]
converts multiple columnar values into arrays of the same length.

However, it is often much more performant to provide a different,
implementation that handles scalar values differently

---
