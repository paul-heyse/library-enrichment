# `arrow_array::temporal_conversions::timestamp_s_to_date`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.timestamp_s_to_date.json).

<a id="op-6c0c9a4fb0b9205d247f75c6"></a>
## timestamp_s_to_date

`function` · `arrow_array::temporal_conversions::timestamp_s_to_date` · arrow-array 59.3.0

```rust
fn timestamp_s_to_date(secs: i64) -> Option<chrono::NaiveDateTime>
```

Source: `src/temporal_conversions.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Similar to timestamp_s_to_datetime but only compute `date`
