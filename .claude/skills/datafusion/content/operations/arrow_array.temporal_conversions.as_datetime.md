# `arrow_array::temporal_conversions::as_datetime`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.as_datetime.json).

<a id="op-4563fc5ebb5f89382a03c307"></a>
## as_datetime

`function` · `arrow_array::temporal_conversions::as_datetime` · arrow-array 59.3.0

```rust
fn as_datetime<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::NaiveDateTime>
```

Source: `src/temporal_conversions.rs:243`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803) to [`NaiveDateTime`]

Unresolved upstream links (retained, not inferred): ``NaiveDateTime``.
