# `arrow_array::temporal_conversions::timestamp_ms_to_datetime`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.timestamp_ms_to_datetime.json).

<a id="op-e1ab1ffa7f38c717bf28d0ae"></a>
## timestamp_ms_to_datetime

`function` · `arrow_array::temporal_conversions::timestamp_ms_to_datetime` · arrow-array 59.3.0

```rust
fn timestamp_ms_to_datetime(v: i64) -> Option<chrono::NaiveDateTime>
```

Source: `src/temporal_conversions.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

converts a `i64` representing a `timestamp(ms)` to [`NaiveDateTime`]

Unresolved upstream links (retained, not inferred): ``NaiveDateTime``.
