# `datafusion_functions::datetime::expr_fn::to_char`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_char.json).

<a id="op-5345b750a66c53b9345dccc2"></a>
## to_char

`function` · `datafusion_functions::datetime::expr_fn::to_char` · datafusion-functions 55.1.0

```rust
fn to_char(datetime: datafusion_expr::Expr, format: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

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
