# `arrow_array::temporal_conversions::timestamp_s_to_time`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.timestamp_s_to_time.json).

<a id="op-c2b46f7af8ce885c516234dc"></a>
## timestamp_s_to_time

`function` · `arrow_array::temporal_conversions::timestamp_s_to_time` · arrow-array 59.3.0

```rust
fn timestamp_s_to_time(secs: i64) -> Option<chrono::NaiveDateTime>
```

Source: `src/temporal_conversions.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Similar to timestamp_s_to_datetime but only compute `time`
