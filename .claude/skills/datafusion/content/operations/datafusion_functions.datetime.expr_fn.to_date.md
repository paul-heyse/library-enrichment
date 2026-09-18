# `datafusion_functions::datetime::expr_fn::to_date`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.expr_fn.to_date.json).

<a id="op-c6af2c18766003d0ace68936"></a>
## to_date

`function` · `datafusion_functions::datetime::expr_fn::to_date` · datafusion-functions 55.1.0

```rust
fn to_date(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/datetime/mod.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

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
