# `arrow_array::temporal_conversions`

Crate `arrow-array` · 33 public items · structured records in [`model/arrow_array.temporal_conversions.json`](../model/arrow_array.temporal_conversions.json)

## MICROSECONDS

`constant` · `arrow_array::temporal_conversions::MICROSECONDS`

Also reachable as `arrow::temporal_conversions::MICROSECONDS`

```rust
const MICROSECONDS: i64 = 1_000_000
```

Number of microseconds in a second

---

## MICROSECONDS_IN_DAY

`constant` · `arrow_array::temporal_conversions::MICROSECONDS_IN_DAY`

Also reachable as `arrow::temporal_conversions::MICROSECONDS_IN_DAY`

```rust
const MICROSECONDS_IN_DAY: i64 = _
```

Number of microseconds in a day

---

## MILLISECONDS

`constant` · `arrow_array::temporal_conversions::MILLISECONDS`

Also reachable as `arrow::temporal_conversions::MILLISECONDS`

```rust
const MILLISECONDS: i64 = 1_000
```

Number of milliseconds in a second

---

## MILLISECONDS_IN_DAY

`constant` · `arrow_array::temporal_conversions::MILLISECONDS_IN_DAY`

Also reachable as `arrow::temporal_conversions::MILLISECONDS_IN_DAY`

```rust
const MILLISECONDS_IN_DAY: i64 = _
```

Number of milliseconds in a day

---

## NANOSECONDS

`constant` · `arrow_array::temporal_conversions::NANOSECONDS`

Also reachable as `arrow::temporal_conversions::NANOSECONDS`

```rust
const NANOSECONDS: i64 = 1_000_000_000
```

Number of nanoseconds in a second

---

## NANOSECONDS_IN_DAY

`constant` · `arrow_array::temporal_conversions::NANOSECONDS_IN_DAY`

Also reachable as `arrow::temporal_conversions::NANOSECONDS_IN_DAY`

```rust
const NANOSECONDS_IN_DAY: i64 = _
```

Number of nanoseconds in a day

---

## SECONDS_IN_DAY

`constant` · `arrow_array::temporal_conversions::SECONDS_IN_DAY`

Also reachable as `arrow::temporal_conversions::SECONDS_IN_DAY`

```rust
const SECONDS_IN_DAY: i64 = 86_400
```

Number of seconds in a day

---

## UNIX_EPOCH_DAY

`constant` · `arrow_array::temporal_conversions::UNIX_EPOCH_DAY`

Also reachable as `arrow::temporal_conversions::UNIX_EPOCH_DAY`

```rust
const UNIX_EPOCH_DAY: i64 = 719_163
```

Constant from chrono crate

Number of days between Januari 1, 1970 and December 31, 1 BCE which we define to be day 0.
4 full leap year cycles until December 31, 1600     4 * 146097 = 584388
1 day until January 1, 1601                                           1
369 years until Januari 1, 1970                      369 * 365 = 134685
of which floor(369 / 4) are leap years          floor(369 / 4) =     92
except for 1700, 1800 and 1900                                       -3 +
                                                                 --------
                                                                 719163

---

## as_date

`function` · `arrow_array::temporal_conversions::as_date`

Also reachable as `arrow::temporal_conversions::as_date`

```rust
fn as_date<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::NaiveDate>
```

Converts an [`ArrowPrimitiveType`] to [`NaiveDate`]

---

## as_datetime

`function` · `arrow_array::temporal_conversions::as_datetime`

Also reachable as `arrow::temporal_conversions::as_datetime`

```rust
fn as_datetime<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::NaiveDateTime>
```

Converts an [`ArrowPrimitiveType`] to [`NaiveDateTime`]

---

## as_datetime_with_timezone

`function` · `arrow_array::temporal_conversions::as_datetime_with_timezone`

Also reachable as `arrow::temporal_conversions::as_datetime_with_timezone`

```rust
fn as_datetime_with_timezone<T: ArrowPrimitiveType>(v: i64, tz: timezone::Tz) -> Option<chrono::DateTime<timezone::Tz>>
```

Converts an [`ArrowPrimitiveType`] to [`DateTime<Tz>`]

---

## as_duration

`function` · `arrow_array::temporal_conversions::as_duration`

Also reachable as `arrow::temporal_conversions::as_duration`

```rust
fn as_duration<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::Duration>
```

Converts an [`ArrowPrimitiveType`] to [`Duration`]

---

## as_time

`function` · `arrow_array::temporal_conversions::as_time`

Also reachable as `arrow::temporal_conversions::as_time`

```rust
fn as_time<T: ArrowPrimitiveType>(v: i64) -> Option<chrono::NaiveTime>
```

Converts an [`ArrowPrimitiveType`] to [`NaiveTime`]

---

## date32_to_datetime

`function` · `arrow_array::temporal_conversions::date32_to_datetime`

Also reachable as `arrow::temporal_conversions::date32_to_datetime`

```rust
fn date32_to_datetime(v: i32) -> Option<chrono::NaiveDateTime>
```

converts a `i32` representing a `date32` to [`NaiveDateTime`]

---

## date64_to_datetime

`function` · `arrow_array::temporal_conversions::date64_to_datetime`

Also reachable as `arrow::temporal_conversions::date64_to_datetime`

```rust
fn date64_to_datetime(v: i64) -> Option<chrono::NaiveDateTime>
```

converts a `i64` representing a `date64` to [`NaiveDateTime`]

---

## duration_ns_to_duration

`function` · `arrow_array::temporal_conversions::duration_ns_to_duration`

Also reachable as `arrow::temporal_conversions::duration_ns_to_duration`

```rust
fn duration_ns_to_duration(v: i64) -> chrono::Duration
```

converts a `i64` representing a `duration(ns)` to [`Duration`]

---

## duration_us_to_duration

`function` · `arrow_array::temporal_conversions::duration_us_to_duration`

Also reachable as `arrow::temporal_conversions::duration_us_to_duration`

```rust
fn duration_us_to_duration(v: i64) -> chrono::Duration
```

converts a `i64` representing a `duration(us)` to [`Duration`]

---

## time32ms_to_time

`function` · `arrow_array::temporal_conversions::time32ms_to_time`

Also reachable as `arrow::temporal_conversions::time32ms_to_time`

```rust
fn time32ms_to_time(v: i32) -> Option<chrono::NaiveTime>
```

converts a `i32` representing a `time32(ms)` to [`NaiveDateTime`]

---

## time32s_to_time

`function` · `arrow_array::temporal_conversions::time32s_to_time`

Also reachable as `arrow::temporal_conversions::time32s_to_time`

```rust
fn time32s_to_time(v: i32) -> Option<chrono::NaiveTime>
```

converts a `i32` representing a `time32(s)` to [`NaiveDateTime`]

---

## time64ns_to_time

`function` · `arrow_array::temporal_conversions::time64ns_to_time`

Also reachable as `arrow::temporal_conversions::time64ns_to_time`

```rust
fn time64ns_to_time(v: i64) -> Option<chrono::NaiveTime>
```

converts a `i64` representing a `time64(ns)` to [`NaiveDateTime`]

---

## time64us_to_time

`function` · `arrow_array::temporal_conversions::time64us_to_time`

Also reachable as `arrow::temporal_conversions::time64us_to_time`

```rust
fn time64us_to_time(v: i64) -> Option<chrono::NaiveTime>
```

converts a `i64` representing a `time64(us)` to [`NaiveDateTime`]

---

## time_to_time32ms

`function` · `arrow_array::temporal_conversions::time_to_time32ms`

Also reachable as `arrow::temporal_conversions::time_to_time32ms`

```rust
fn time_to_time32ms(v: chrono::NaiveTime) -> i32
```

converts [`NaiveTime`] to a `i32` representing a `time32(ms)`

---

## time_to_time32s

`function` · `arrow_array::temporal_conversions::time_to_time32s`

Also reachable as `arrow::temporal_conversions::time_to_time32s`

```rust
fn time_to_time32s(v: chrono::NaiveTime) -> i32
```

converts [`NaiveTime`] to a `i32` representing a `time32(s)`

---

## time_to_time64ns

`function` · `arrow_array::temporal_conversions::time_to_time64ns`

Also reachable as `arrow::temporal_conversions::time_to_time64ns`

```rust
fn time_to_time64ns(v: chrono::NaiveTime) -> i64
```

converts [`NaiveTime`] to a `i64` representing a `time64(ns)`

---

## time_to_time64us

`function` · `arrow_array::temporal_conversions::time_to_time64us`

Also reachable as `arrow::temporal_conversions::time_to_time64us`

```rust
fn time_to_time64us(v: chrono::NaiveTime) -> i64
```

converts [`NaiveTime`] to a `i64` representing a `time64(us)`

---

## timestamp_ms_to_datetime

`function` · `arrow_array::temporal_conversions::timestamp_ms_to_datetime`

Also reachable as `arrow::temporal_conversions::timestamp_ms_to_datetime`

```rust
fn timestamp_ms_to_datetime(v: i64) -> Option<chrono::NaiveDateTime>
```

converts a `i64` representing a `timestamp(ms)` to [`NaiveDateTime`]

---

## timestamp_ns_to_datetime

`function` · `arrow_array::temporal_conversions::timestamp_ns_to_datetime`

Also reachable as `arrow::temporal_conversions::timestamp_ns_to_datetime`

```rust
fn timestamp_ns_to_datetime(v: i64) -> Option<chrono::NaiveDateTime>
```

converts a `i64` representing a `timestamp(ns)` to [`NaiveDateTime`]

---

## timestamp_s_to_date

`function` · `arrow_array::temporal_conversions::timestamp_s_to_date`

Also reachable as `arrow::temporal_conversions::timestamp_s_to_date`

```rust
fn timestamp_s_to_date(secs: i64) -> Option<chrono::NaiveDateTime>
```

Similar to timestamp_s_to_datetime but only compute `date`

---

## timestamp_s_to_datetime

`function` · `arrow_array::temporal_conversions::timestamp_s_to_datetime`

Also reachable as `arrow::temporal_conversions::timestamp_s_to_datetime`

```rust
fn timestamp_s_to_datetime(v: i64) -> Option<chrono::NaiveDateTime>
```

converts a `i64` representing a `timestamp(s)` to [`NaiveDateTime`]

---

## timestamp_s_to_time

`function` · `arrow_array::temporal_conversions::timestamp_s_to_time`

Also reachable as `arrow::temporal_conversions::timestamp_s_to_time`

```rust
fn timestamp_s_to_time(secs: i64) -> Option<chrono::NaiveDateTime>
```

Similar to timestamp_s_to_datetime but only compute `time`

---

## timestamp_us_to_datetime

`function` · `arrow_array::temporal_conversions::timestamp_us_to_datetime`

Also reachable as `arrow::temporal_conversions::timestamp_us_to_datetime`

```rust
fn timestamp_us_to_datetime(v: i64) -> Option<chrono::NaiveDateTime>
```

converts a `i64` representing a `timestamp(us)` to [`NaiveDateTime`]

---

## try_duration_ms_to_duration

`function` · `arrow_array::temporal_conversions::try_duration_ms_to_duration`

Also reachable as `arrow::temporal_conversions::try_duration_ms_to_duration`

```rust
fn try_duration_ms_to_duration(v: i64) -> Option<chrono::Duration>
```

converts a `i64` representing a `duration(ms)` to [`Option<Duration>`]

---

## try_duration_s_to_duration

`function` · `arrow_array::temporal_conversions::try_duration_s_to_duration`

Also reachable as `arrow::temporal_conversions::try_duration_s_to_duration`

```rust
fn try_duration_s_to_duration(v: i64) -> Option<chrono::Duration>
```

converts a `i64` representing a `duration(s)` to [`Option<Duration>`]

---
