# `datafusion_expr_common::casts::is_timestamp_precision_narrowing_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.casts.is_timestamp_precision_narrowing_cast.json).

<a id="op-e6e8fe96ffe5b9841ba161bb"></a>
## is_timestamp_precision_narrowing_cast

`function` · `datafusion_expr_common::casts::is_timestamp_precision_narrowing_cast` · datafusion-expr-common 55.1.0

```rust
fn is_timestamp_precision_narrowing_cast(from_type: &arrow::datatypes::DataType, to_type: &arrow::datatypes::DataType) -> bool
```

Source: `src/casts.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns true when casting a timestamp from `from_type` to `to_type` loses
timestamp precision.

This is used by comparison cast unwrapping to avoid rewrites such as
`CAST(ts_ns AS timestamp(ms)) = lit_ms` -> `ts_ns = lit_ns`. The original
predicate can match any nanosecond value in the same millisecond, while the
rewritten predicate only matches the exact millisecond boundary.
