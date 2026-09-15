# `datafusion_functions::datetime::expr_fn`

Crate `datafusion-functions` · 19 public items · structured records in [`model/datafusion_functions.datetime.expr_fn.json`](../model/datafusion_functions.datetime.expr_fn.json)

## current_date

`function` · `datafusion_functions::datetime::expr_fn::current_date`

Also reachable as `datafusion::prelude::current_date`, `datafusion_functions::expr_fn::current_date`

```rust
fn current_date() -> datafusion_expr::Expr
```

returns current UTC date as a Date32 value

---

## current_time

`function` · `datafusion_functions::datetime::expr_fn::current_time`

Also reachable as `datafusion::prelude::current_time`, `datafusion_functions::expr_fn::current_time`

```rust
fn current_time() -> datafusion_expr::Expr
```

returns current UTC time as a Time64 value

---

## date_bin

`function` · `datafusion_functions::datetime::expr_fn::date_bin`

Also reachable as `datafusion::prelude::date_bin`, `datafusion_functions::expr_fn::date_bin`

```rust
fn date_bin(stride: datafusion_expr::Expr, source: datafusion_expr::Expr, origin: datafusion_expr::Expr) -> datafusion_expr::Expr
```

coerces an arbitrary timestamp to the start of the nearest specified interval

---

## date_part

`function` · `datafusion_functions::datetime::expr_fn::date_part`

Also reachable as `datafusion::prelude::date_part`, `datafusion_functions::expr_fn::date_part`

```rust
fn date_part(part: datafusion_expr::Expr, date: datafusion_expr::Expr) -> datafusion_expr::Expr
```

extracts a subfield from the date

---

## date_trunc

`function` · `datafusion_functions::datetime::expr_fn::date_trunc`

Also reachable as `datafusion::prelude::date_trunc`, `datafusion_functions::expr_fn::date_trunc`

```rust
fn date_trunc(part: datafusion_expr::Expr, date: datafusion_expr::Expr) -> datafusion_expr::Expr
```

truncates the date to a specified level of precision

---

## from_unixtime

`function` · `datafusion_functions::datetime::expr_fn::from_unixtime`

Also reachable as `datafusion::prelude::from_unixtime`, `datafusion_functions::expr_fn::from_unixtime`

```rust
fn from_unixtime(unixtime: datafusion_expr::Expr) -> datafusion_expr::Expr
```

converts an integer to RFC3339 timestamp format string

---

## make_date

`function` · `datafusion_functions::datetime::expr_fn::make_date`

Also reachable as `datafusion::prelude::make_date`, `datafusion_functions::expr_fn::make_date`

```rust
fn make_date(year: datafusion_expr::Expr, month: datafusion_expr::Expr, day: datafusion_expr::Expr) -> datafusion_expr::Expr
```

make a date from year, month and day component parts

---

## make_time

`function` · `datafusion_functions::datetime::expr_fn::make_time`

Also reachable as `datafusion::prelude::make_time`, `datafusion_functions::expr_fn::make_time`

```rust
fn make_time(hour: datafusion_expr::Expr, minute: datafusion_expr::Expr, second: datafusion_expr::Expr) -> datafusion_expr::Expr
```

make a time from hour, minute and second component parts

---

## now

`function` · `datafusion_functions::datetime::expr_fn::now`

Also reachable as `datafusion::prelude::now`, `datafusion_functions::expr_fn::now`

```rust
fn now() -> datafusion_expr::Expr
```

returns the current timestamp in nanoseconds, using the same value for all instances of now() in same statement

---

## to_char

`function` · `datafusion_functions::datetime::expr_fn::to_char`

Also reachable as `datafusion::prelude::to_char`, `datafusion_functions::expr_fn::to_char`

```rust
fn to_char(datetime: datafusion_expr::Expr, format: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns a string representation of a date, time, timestamp or duration based
on a Chrono pattern.

The syntax for the patterns can be found at
<https://docs.rs/chrono/latest/chrono/format/strftime/index.html>

# Examples

```ignore
# use chrono::prelude::*;
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::ScalarValue::TimestampNanosecond;
# use std::sync::Arc;
# use arrow::array::{Date32Array, RecordBatch, StringArray};
# use arrow::datatypes::{DataType, Field, Schema};
# #[tokio::main]
# async fn main() -> Result<()> {
let schema = Arc::new(Schema::new(vec![
    Field::new("values", DataType::Date32, false),
    Field::new("patterns", DataType::Utf8, false),
]));

let batch = RecordBatch::try_new(
    schema,
    vec![
        Arc::new(Date32Array::from(vec![
            18506,
            18507,
            18508,
            18509,
        ])),
        Arc::new(StringArray::from(vec![
            "%Y-%m-%d",
            "%Y:%m:%d",
            "%Y%m%d",
            "%d-%m-%Y",
        ])),
    ],
)?;

let ctx = SessionContext::new();
ctx.register_batch("t", batch)?;
let df = ctx.table("t").await?;

// use the to_char function to convert col 'values',
// to strings using patterns in col 'patterns'
let df = df.with_column(
    "date_str",
    to_char(col("values"), col("patterns"))
)?;
// Note that providing a scalar value for the pattern
// is more performant
let df = df.with_column(
    "date_str2",
    to_char(col("values"), lit("%d-%m-%Y"))
)?;
// literals can be used as well with dataframe calls
let timestamp = "2026-07-08T09:10:11"
    .parse::<NaiveDateTime>()
    .unwrap()
    .with_nanosecond(56789)
    .unwrap()
    .timestamp_nanos_opt()
    .unwrap();
let df = df.with_column(
    "timestamp_str",
    to_char(lit(TimestampNanosecond(Some(timestamp), None)), lit("%d-%m-%Y %H:%M:%S"))
)?;

df.show().await?;

# Ok(())
# }
```

---

## to_date

`function` · `datafusion_functions::datetime::expr_fn::to_date`

Also reachable as `datafusion::prelude::to_date`, `datafusion_functions::expr_fn::to_date`

```rust
fn to_date(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

```ignore
# use std::sync::Arc;

# use datafusion_common::Result;

# #[tokio::main]
# async fn main() -> Result<()> {
#  use arrow::array::StringArray;
#  use arrow::datatypes::{DataType, Field, Schema};
#  use arrow::record_batch::RecordBatch;
#  use datafusion_expr::col;
#  use datafusion::prelude::*;
#  use datafusion_functions::expr_fn::to_date;

    // define a schema.
    let schema = Arc::new(Schema::new(vec![Field::new("a", DataType::Utf8, false)]));

    // define data.
    let batch = RecordBatch::try_new(
        schema,
        vec![Arc::new(StringArray::from(vec![
            "2020-09-08T13:42:29Z",
            "2020-09-08T13:42:29.190855-05:00",
            "2020-08-09 12:13:29",
            "2020-01-02",
        ]))],
    )?;

    // declare a new context. In spark API, this corresponds to a new spark SQLsession
    let ctx = SessionContext::new();

    // declare a table in memory. In spark API, this corresponds to createDataFrame(...).
    ctx.register_batch("t", batch)?;
    let df = ctx.table("t").await?;

    // use to_date function to convert col 'a' to timestamp type using the default parsing
    let df = df.with_column("a", to_date(vec![col("a")]))?;

    let df = df.select_columns(&["a"])?;

    // print the results
    df.show().await?;

    # Ok(())
# }
```

---

## to_local_time

`function` · `datafusion_functions::datetime::expr_fn::to_local_time`

Also reachable as `datafusion::prelude::to_local_time`, `datafusion_functions::expr_fn::to_local_time`

```rust
fn to_local_time(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a timezone-aware timestamp to local time (with no offset or timezone information), i.e. strips off the timezone from the timestamp

---

## to_time

`function` · `datafusion_functions::datetime::expr_fn::to_time`

Also reachable as `datafusion::prelude::to_time`, `datafusion_functions::expr_fn::to_time`

```rust
fn to_time(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a string and optional formats to a `Time64(Nanoseconds)`

---

## to_timestamp

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp`

Also reachable as `datafusion::prelude::to_timestamp`, `datafusion_functions::expr_fn::to_timestamp`

```rust
fn to_timestamp(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a string and optional formats to a `Timestamp(Nanoseconds, TimeZone)`

---

## to_timestamp_micros

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_micros`

Also reachable as `datafusion::prelude::to_timestamp_micros`, `datafusion_functions::expr_fn::to_timestamp_micros`

```rust
fn to_timestamp_micros(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a string and optional formats to a `Timestamp(Microseconds, TimeZone)`

---

## to_timestamp_millis

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_millis`

Also reachable as `datafusion::prelude::to_timestamp_millis`, `datafusion_functions::expr_fn::to_timestamp_millis`

```rust
fn to_timestamp_millis(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a string and optional formats to a `Timestamp(Milliseconds, TimeZone)`

---

## to_timestamp_nanos

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_nanos`

Also reachable as `datafusion::prelude::to_timestamp_nanos`, `datafusion_functions::expr_fn::to_timestamp_nanos`

```rust
fn to_timestamp_nanos(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a string and optional formats to a `Timestamp(Nanoseconds, TimeZone)`

---

## to_timestamp_seconds

`function` · `datafusion_functions::datetime::expr_fn::to_timestamp_seconds`

Also reachable as `datafusion::prelude::to_timestamp_seconds`, `datafusion_functions::expr_fn::to_timestamp_seconds`

```rust
fn to_timestamp_seconds(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a string and optional formats to a `Timestamp(Seconds, TimeZone)`

---

## to_unixtime

`function` · `datafusion_functions::datetime::expr_fn::to_unixtime`

Also reachable as `datafusion::prelude::to_unixtime`, `datafusion_functions::expr_fn::to_unixtime`

```rust
fn to_unixtime(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

converts a value to seconds since the unix epoch

---
