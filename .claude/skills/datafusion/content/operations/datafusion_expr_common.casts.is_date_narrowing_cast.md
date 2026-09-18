# `datafusion_expr_common::casts::is_date_narrowing_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.casts.is_date_narrowing_cast.json).

<a id="op-ce56f72b120055a245bce581"></a>
## is_date_narrowing_cast

`function` · `datafusion_expr_common::casts::is_date_narrowing_cast` · datafusion-expr-common 55.1.0

```rust
fn is_date_narrowing_cast(from_type: &arrow::datatypes::DataType, to_type: &arrow::datatypes::DataType) -> bool
```

Source: `src/casts.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns true when casting a date column from `from_type` to `to_type` narrows
`Date64` (milliseconds) to `Date32` (days).

Like [`is_timestamp_precision_narrowing_cast`](../operations/datafusion_expr_common.casts.is_timestamp_precision_narrowing_cast.md#op-e6e8fe96ffe5b9841ba161bb), this guards comparison cast
unwrapping against a many-to-one column cast. `CAST(date64 AS Date32) = lit_day`
matches any millisecond within that day, but the rewritten `date64 = lit_ms`
matches only midnight. Arrow does not require `Date64` values to be whole days
(see arrow-rs#5288), so the column may carry sub-day values the planner cannot
see; the widening direction (`Date32 -> Date64`) is injective and stays allowed.
