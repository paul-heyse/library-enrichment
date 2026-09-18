# `arrow_array::temporal_conversions::as_time`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.as_time.json).

<a id="op-28b39da9bdf78f46e750cb6b"></a>
## as_time

`function` · `arrow_array::temporal_conversions::as_time` · arrow-array 59.3.0

```rust
fn as_time<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::NaiveTime>
```

Source: `src/temporal_conversions.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803) to [`NaiveTime`]

Unresolved upstream links (retained, not inferred): ``NaiveTime``.
