# `datafusion_expr::type_coercion`

Crate `datafusion-expr` · 4 public items · structured records in [`model/datafusion_expr.type_coercion.json`](../model/datafusion_expr.type_coercion.json)

## is_datetime

`function` · `datafusion_expr::type_coercion::is_datetime`

```rust
fn is_datetime(dt: &arrow::datatypes::DataType) -> bool
```

Determine whether the given data type `dt` is a `Date` or `Timestamp`.

---

## is_interval

`function` · `datafusion_expr::type_coercion::is_interval`

```rust
fn is_interval(dt: &arrow::datatypes::DataType) -> bool
```

Determine whether the given data type 'dt' is a `Interval`.

---

## is_signed_numeric

`function` · `datafusion_expr::type_coercion::is_signed_numeric`

```rust
fn is_signed_numeric(dt: &arrow::datatypes::DataType) -> bool
```

Determine whether the given data type `dt` represents signed numeric values.

---

## is_timestamp

`function` · `datafusion_expr::type_coercion::is_timestamp`

```rust
fn is_timestamp(dt: &arrow::datatypes::DataType) -> bool
```

Determine whether the given data type `dt` is a `Timestamp`.

---
