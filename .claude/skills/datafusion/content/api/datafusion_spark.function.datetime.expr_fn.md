# `datafusion_spark::function::datetime::expr_fn`

Crate `datafusion-spark` · 23 public items · structured records in [`model/datafusion_spark.function.datetime.expr_fn.json`](../model/datafusion_spark.function.datetime.expr_fn.json)

## add_months

`function` · `datafusion_spark::function::datetime::expr_fn::add_months`

Also reachable as `datafusion_spark::expr_fn::add_months`

```rust
fn add_months(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.add_months.md).


Returns the date that is months months after start. The function returns NULL if at least one of the input parameters is NULL.

---

## date_add

`function` · `datafusion_spark::function::datetime::expr_fn::date_add`

Also reachable as `datafusion_spark::expr_fn::date_add`

```rust
fn date_add(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.date_add.md).


Returns the date that is days days after start. The function returns NULL if at least one of the input parameters is NULL.

---

## date_diff

`function` · `datafusion_spark::function::datetime::expr_fn::date_diff`

Also reachable as `datafusion_spark::expr_fn::date_diff`

```rust
fn date_diff(end: datafusion_expr::Expr, start: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.date_diff.md).


Returns the number of days from start `start` to end `end`.

---

## date_part

`function` · `datafusion_spark::function::datetime::expr_fn::date_part`

Also reachable as `datafusion_spark::expr_fn::date_part`

```rust
fn date_part(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.date_part.md).


Extracts a part of the date or time from a date, time, or timestamp expression.

---

## date_sub

`function` · `datafusion_spark::function::datetime::expr_fn::date_sub`

Also reachable as `datafusion_spark::expr_fn::date_sub`

```rust
fn date_sub(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.date_sub.md).


Returns the date that is days days before start. The function returns NULL if at least one of the input parameters is NULL.

---

## date_trunc

`function` · `datafusion_spark::function::datetime::expr_fn::date_trunc`

Also reachable as `datafusion_spark::expr_fn::date_trunc`

```rust
fn date_trunc(fmt: datafusion_expr::Expr, ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.date_trunc.md).


Truncates a timestamp `ts` to the unit specified by the format `fmt`.

---

## from_utc_timestamp

`function` · `datafusion_spark::function::datetime::expr_fn::from_utc_timestamp`

Also reachable as `datafusion_spark::expr_fn::from_utc_timestamp`

```rust
fn from_utc_timestamp(ts: datafusion_expr::Expr, tz: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.from_utc_timestamp.md).


Interpret a given timestamp `ts` in UTC timezone and then convert it to timezone `tz`.

---

## hour

`function` · `datafusion_spark::function::datetime::expr_fn::hour`

Also reachable as `datafusion_spark::expr_fn::hour`

```rust
fn hour(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.hour.md).


Extracts the hour component of a timestamp.

---

## last_day

`function` · `datafusion_spark::function::datetime::expr_fn::last_day`

Also reachable as `datafusion_spark::expr_fn::last_day`

```rust
fn last_day(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.last_day.md).


Returns the last day of the month which the date belongs to.

---

## make_dt_interval

`function` · `datafusion_spark::function::datetime::expr_fn::make_dt_interval`

Also reachable as `datafusion_spark::expr_fn::make_dt_interval`

```rust
fn make_dt_interval(days: datafusion_expr::Expr, hours: datafusion_expr::Expr, mins: datafusion_expr::Expr, secs: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.make_dt_interval.md).


Make a day time interval from given days, hours, mins and secs (return type is actually a Duration(Microsecond))

---

## make_interval

`function` · `datafusion_spark::function::datetime::expr_fn::make_interval`

Also reachable as `datafusion_spark::expr_fn::make_interval`

```rust
fn make_interval(years: datafusion_expr::Expr, months: datafusion_expr::Expr, weeks: datafusion_expr::Expr, days: datafusion_expr::Expr, hours: datafusion_expr::Expr, mins: datafusion_expr::Expr, secs: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.make_interval.md).


Make interval from years, months, weeks, days, hours, mins and secs.

---

## minute

`function` · `datafusion_spark::function::datetime::expr_fn::minute`

Also reachable as `datafusion_spark::expr_fn::minute`

```rust
fn minute(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.minute.md).


Extracts the minute component of a timestamp.

---

## monthname

`function` · `datafusion_spark::function::datetime::expr_fn::monthname`

Also reachable as `datafusion_spark::expr_fn::monthname`

```rust
fn monthname(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.monthname.md).


Returns the three-letter abbreviated month name from a date or timestamp.

---

## next_day

`function` · `datafusion_spark::function::datetime::expr_fn::next_day`

Also reachable as `datafusion_spark::expr_fn::next_day`

```rust
fn next_day(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.next_day.md).


Returns the first date which is later than start_date and named as indicated. The function returns NULL if at least one of the input parameters is NULL.

---

## second

`function` · `datafusion_spark::function::datetime::expr_fn::second`

Also reachable as `datafusion_spark::expr_fn::second`

```rust
fn second(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.second.md).


Extracts the second component of a timestamp.

---

## time_trunc

`function` · `datafusion_spark::function::datetime::expr_fn::time_trunc`

Also reachable as `datafusion_spark::expr_fn::time_trunc`

```rust
fn time_trunc(fmt: datafusion_expr::Expr, t: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.time_trunc.md).


Truncates a time `t` to the unit specified by the format `fmt`.

---

## to_utc_timestamp

`function` · `datafusion_spark::function::datetime::expr_fn::to_utc_timestamp`

Also reachable as `datafusion_spark::expr_fn::to_utc_timestamp`

```rust
fn to_utc_timestamp(ts: datafusion_expr::Expr, tz: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.to_utc_timestamp.md).


Interpret a given timestamp `ts` in timezone `tz` and then convert it to UTC timezone.

---

## trunc

`function` · `datafusion_spark::function::datetime::expr_fn::trunc`

Also reachable as `datafusion_spark::expr_fn::trunc`

```rust
fn trunc(dt: datafusion_expr::Expr, fmt: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.trunc.md).


Truncates a date `dt` to the unit specified by the format `fmt`.

---

## unix_date

`function` · `datafusion_spark::function::datetime::expr_fn::unix_date`

Also reachable as `datafusion_spark::expr_fn::unix_date`

```rust
fn unix_date(dt: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.unix_date.md).


Returns the number of days since epoch (1970-01-01) for the given date `dt`.

---

## unix_micros

`function` · `datafusion_spark::function::datetime::expr_fn::unix_micros`

Also reachable as `datafusion_spark::expr_fn::unix_micros`

```rust
fn unix_micros(ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.unix_micros.md).


Returns the number of microseconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp `ts`.

---

## unix_millis

`function` · `datafusion_spark::function::datetime::expr_fn::unix_millis`

Also reachable as `datafusion_spark::expr_fn::unix_millis`

```rust
fn unix_millis(ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.unix_millis.md).


Returns the number of milliseconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp `ts`.

---

## unix_seconds

`function` · `datafusion_spark::function::datetime::expr_fn::unix_seconds`

Also reachable as `datafusion_spark::expr_fn::unix_seconds`

```rust
fn unix_seconds(ts: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.unix_seconds.md).


Returns the number of seconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp `ts`.

---

## weekday

`function` · `datafusion_spark::function::datetime::expr_fn::weekday`

Also reachable as `datafusion_spark::expr_fn::weekday`

```rust
fn weekday(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.datetime.expr_fn.weekday.md).


Returns the day of the week for date/timestamp as an integer where Monday = 0, Tuesday = 1, ..., Sunday = 6.

---
