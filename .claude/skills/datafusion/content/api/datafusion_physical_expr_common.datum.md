# `datafusion_physical_expr_common::datum`

Crate `datafusion-physical-expr-common` · 5 public items · structured records in [`model/datafusion_physical_expr_common.datum.json`](../model/datafusion_physical_expr_common.datum.json)

## apply

`function` · `datafusion_physical_expr_common::datum::apply`

```rust
fn apply(lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue, f: impl Fn(&dyn Datum, &dyn Datum) -> datafusion_common::Result<arrow::array::ArrayRef, arrow::error::ArrowError>) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue>
```

Applies a binary [`Datum`] kernel `f` to `lhs` and `rhs`

This maps arrow-rs' [`Datum`] kernels to DataFusion's [`ColumnarValue`] abstraction

---

## apply_cmp

`function` · `datafusion_physical_expr_common::datum::apply_cmp`

```rust
fn apply_cmp(op: datafusion_expr_common::operator::Operator, lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue>
```

Applies a binary [`Datum`] comparison operator `op` to `lhs` and `rhs`

---

## apply_cmp_for_nested

`function` · `datafusion_physical_expr_common::datum::apply_cmp_for_nested`

```rust
fn apply_cmp_for_nested(op: datafusion_expr_common::operator::Operator, lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue>
```

Applies a binary [`Datum`] comparison operator `op` to `lhs` and `rhs` for nested type like
List, FixedSizeList, LargeList, Struct, Union, Map, or a dictionary of a nested type

---

## compare_op_for_nested

`function` · `datafusion_physical_expr_common::datum::compare_op_for_nested`

```rust
fn compare_op_for_nested(op: datafusion_expr_common::operator::Operator, lhs: &dyn Datum, rhs: &dyn Datum) -> datafusion_common::Result<arrow::array::BooleanArray>
```

Compare on nested type List, Struct, and so on

---

## compare_with_eq

`function` · `datafusion_physical_expr_common::datum::compare_with_eq`

```rust
fn compare_with_eq(lhs: &dyn Datum, rhs: &dyn Datum, is_nested: bool) -> datafusion_common::Result<arrow::array::BooleanArray>
```

Compare with eq with either nested or non-nested

---
