# `datafusion_functions::regex::regexplike::regexp_like`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.regexplike.regexp_like.json).

<a id="op-a5748a3e58caa64ee87ac52a"></a>
## regexp_like

`function` · `datafusion_functions::regex::regexplike::regexp_like` · datafusion-functions 55.1.0

```rust
fn regexp_like(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/regex/regexplike.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Tests a string using a regular expression returning true if at
least one match, false otherwise.

The full list of supported features and syntax can be found at
<https://docs.rs/regex/latest/regex/#syntax>

Supported flags can be found at
<https://docs.rs/regex/latest/regex/#grouping-and-flags>

# Examples

```ignore
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/regex.csv", CsvReadOptions::new()).await?;

// use the regexp_like function to test col 'values',
// against patterns in col 'patterns' without flags
let df = df.with_column(
    "a",
    regexp_like(vec![col("values"), col("patterns")])
)?;
// use the regexp_like function to test col 'values',
// against patterns in col 'patterns' with flags
let df = df.with_column(
    "b",
    regexp_like(vec![col("values"), col("patterns"), col("flags")])
)?;
// literals can be used as well with dataframe calls
let df = df.with_column(
    "c",
    regexp_like(vec![lit("foobarbequebaz"), lit("(bar)(beque)")])
)?;

df.show().await?;

# Ok(())
# }
```
