# `datafusion_functions::datetime::to_local_time::adjust_to_local_time`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.to_local_time.adjust_to_local_time.json).

<a id="op-9d9d8c22d12aa80eb799da17"></a>
## adjust_to_local_time

`function` · `datafusion_functions::datetime::to_local_time::adjust_to_local_time` · datafusion-functions 55.1.0

```rust
fn adjust_to_local_time<T: ArrowTimestampType>(ts: i64, tz: arrow::array::timezone::Tz) -> datafusion_common::Result<i64>
```

Source: `src/datetime/to_local_time.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

This function converts a timestamp with a timezone to a timestamp without a timezone.
The display value of the adjusted timestamp remain the same, but the underlying timestamp
representation is adjusted according to the relative timezone offset to UTC.

This function uses chrono to handle daylight saving time changes.

For example,

```text
'2019-03-31T01:00:00Z'::timestamp at time zone 'Europe/Brussels'
```

is displayed as follows in datafusion-cli:

```text
2019-03-31T01:00:00+01:00
```

and is represented in DataFusion as:

```text
TimestampNanosecond(Some(1_553_990_400_000_000_000), Some("Europe/Brussels"))
```

To strip off the timezone while keeping the display value the same, we need to
adjust the underlying timestamp with the timezone offset value using `adjust_to_local_time()`

```text
adjust_to_local_time(1_553_990_400_000_000_000, "Europe/Brussels") --> 1_553_994_000_000_000_000
```

The difference between `1_553_990_400_000_000_000` and `1_553_994_000_000_000_000` is
`3600_000_000_000` ns, which corresponds to 1 hour. This matches with the timezone
offset for "Europe/Brussels" for this date.

Note that the offset varies with daylight savings time (DST), which makes this tricky! For
example, timezone "Europe/Brussels" has a 2-hour offset during DST and a 1-hour offset
when DST ends.

Consequently, DataFusion can represent the timestamp in local time (with no offset or
timezone information) as

```text
TimestampNanosecond(Some(1_553_994_000_000_000_000), None)
```

which is displayed as follows in datafusion-cli:

```text
2019-03-31T01:00:00
```

See `test_adjust_to_local_time()` for example
