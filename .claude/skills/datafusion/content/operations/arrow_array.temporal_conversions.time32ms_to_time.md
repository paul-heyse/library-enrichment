# `arrow_array::temporal_conversions::time32ms_to_time`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.time32ms_to_time.json).

<a id="op-79ba2f54c8a5da5b3130425d"></a>
## time32ms_to_time

`function` · `arrow_array::temporal_conversions::time32ms_to_time` · arrow-array 59.3.0

```rust
fn time32ms_to_time(v: i32) -> Option<chrono::NaiveTime>
```

Source: `src/temporal_conversions.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

converts a `i32` representing a `time32(ms)` to [`NaiveDateTime`]

Unresolved upstream links (retained, not inferred): ``NaiveDateTime``.
