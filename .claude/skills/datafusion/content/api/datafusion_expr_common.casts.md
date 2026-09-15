# `datafusion_expr_common::casts`

Crate `datafusion-expr-common` · 4 public items · structured records in [`model/datafusion_expr_common.casts.json`](../model/datafusion_expr_common.casts.json)

## is_date_narrowing_cast

`function` · `datafusion_expr_common::casts::is_date_narrowing_cast`

```rust
fn is_date_narrowing_cast(from_type: &arrow::datatypes::DataType, to_type: &arrow::datatypes::DataType) -> bool
```

Returns true when casting a date column from `from_type` to `to_type` narrows
`Date64` (milliseconds) to `Date32` (days).

Like [`is_timestamp_precision_narrowing_cast`], this guards comparison cast
unwrapping against a many-to-one column cast. `CAST(date64 AS Date32) = lit_day`
matches any millisecond within that day, but the rewritten `date64 = lit_ms`
matches only midnight. Arrow does not require `Date64` values to be whole days
(see arrow-rs#5288), so the column may carry sub-day values the planner cannot
see; the widening direction (`Date32 -> Date64`) is injective and stays allowed.

---

## is_supported_type

`function` · `datafusion_expr_common::casts::is_supported_type`

```rust
fn is_supported_type(data_type: &arrow::datatypes::DataType) -> bool
```

Returns true if unwrap_cast_in_comparison supports this data type

---

## is_timestamp_precision_narrowing_cast

`function` · `datafusion_expr_common::casts::is_timestamp_precision_narrowing_cast`

```rust
fn is_timestamp_precision_narrowing_cast(from_type: &arrow::datatypes::DataType, to_type: &arrow::datatypes::DataType) -> bool
```

Returns true when casting a timestamp from `from_type` to `to_type` loses
timestamp precision.

This is used by comparison cast unwrapping to avoid rewrites such as
`CAST(ts_ns AS timestamp(ms)) = lit_ms` -> `ts_ns = lit_ns`. The original
predicate can match any nanosecond value in the same millisecond, while the
rewritten predicate only matches the exact millisecond boundary.

---

## try_cast_literal_to_type

`function` · `datafusion_expr_common::casts::try_cast_literal_to_type`

```rust
fn try_cast_literal_to_type(lit_value: &datafusion_common::ScalarValue, target_type: &arrow::datatypes::DataType) -> Option<datafusion_common::ScalarValue>
```

Convert a literal [`ScalarValue`] to `target_type`, preserving the exact value.

Returns `None` if the value cannot be represented in `target_type`
*exactly*.

This is a restricted, value-preserving cast used to rewrite comparison
predicates of the form `CAST(col AS target_type) <op> literal` into
`col <op> try_cast_literal_to_type(literal, col_type)`. That rewrite is
only valid when the cast cannot change the comparison result.

# Supported Casts
* numeric → numeric, including integers, decimals, `Date32`/`Date64` and
  `Timestamp`s, rejecting values outside the target's range or that would
  lose decimal digits
* string → string between `Utf8`, `LargeUtf8` and `Utf8View`
* wrapping a value into, or unwrapping it out of, a `Dictionary` whose value
  type matches the literal's type
* `Binary` → `FixedSizeBinary` of the matching length
* `Timestamp` → `Timestamp` cast between different time units is allowed even
  though it can truncate (for example nanoseconds → seconds), and a unit
  conversion that overflows yields a `NULL` literal rather than `None`.

# See Also
- [`ScalarValue::cast_to`]: a general-purpose cast that can lose information
  or change a value's meaning.

---
