# `arrow_array::temporal_conversions::time64ns_to_time`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.time64ns_to_time.json).

<a id="op-0e60f3fbff1ef74639f52ce7"></a>
## time64ns_to_time

`function` · `arrow_array::temporal_conversions::time64ns_to_time` · arrow-array 59.3.0

```rust
fn time64ns_to_time(v: i64) -> Option<chrono::NaiveTime>
```

Source: `src/temporal_conversions.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

converts a `i64` representing a `time64(ns)` to [`NaiveDateTime`]

Unresolved upstream links (retained, not inferred): ``NaiveDateTime``.
