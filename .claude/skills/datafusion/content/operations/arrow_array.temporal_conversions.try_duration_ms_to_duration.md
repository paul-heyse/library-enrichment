# `arrow_array::temporal_conversions::try_duration_ms_to_duration`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.try_duration_ms_to_duration.json).

<a id="op-7ccd3ec10474c00ce10f6834"></a>
## try_duration_ms_to_duration

`function` · `arrow_array::temporal_conversions::try_duration_ms_to_duration` · arrow-array 59.3.0

```rust
fn try_duration_ms_to_duration(v: i64) -> Option<chrono::Duration>
```

Source: `src/temporal_conversions.rs:226`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

converts a `i64` representing a `duration(ms)` to [`Option<Duration>`]

Unresolved upstream links (retained, not inferred): ``Option<Duration>``.
