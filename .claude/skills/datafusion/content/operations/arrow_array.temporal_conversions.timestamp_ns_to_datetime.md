# `arrow_array::temporal_conversions::timestamp_ns_to_datetime`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.timestamp_ns_to_datetime.json).

<a id="op-b84a21de77f12d4975f79d3d"></a>
## timestamp_ns_to_datetime

`function` · `arrow_array::temporal_conversions::timestamp_ns_to_datetime` · arrow-array 59.3.0

```rust
fn timestamp_ns_to_datetime(v: i64) -> Option<chrono::NaiveDateTime>
```

Source: `src/temporal_conversions.rs:202`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

converts a `i64` representing a `timestamp(ns)` to [`NaiveDateTime`]

Unresolved upstream links (retained, not inferred): ``NaiveDateTime``.
