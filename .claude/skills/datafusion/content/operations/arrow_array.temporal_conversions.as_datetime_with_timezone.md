# `arrow_array::temporal_conversions::as_datetime_with_timezone`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.as_datetime_with_timezone.json).

<a id="op-c4b6e763d90ad41201f97d8d"></a>
## as_datetime_with_timezone

`function` · `arrow_array::temporal_conversions::as_datetime_with_timezone` · arrow-array 59.3.0

```rust
fn as_datetime_with_timezone<T: ArrowPrimitiveType>(v: i64, tz: timezone::Tz) -> Option<chrono::DateTime<timezone::Tz>>
```

Source: `src/temporal_conversions.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803) to [`DateTime<Tz>`]

Unresolved upstream links (retained, not inferred): ``DateTime<Tz>``.
