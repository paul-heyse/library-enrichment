# `datafusion_functions::regex::regexpreplace::regexp_replace`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.regexpreplace.regexp_replace.json).

<a id="op-0557fd320af633139ff50d9b"></a>
## regexp_replace

`function` · `datafusion_functions::regex::regexpreplace::regexp_replace` · datafusion-functions 55.1.0

```rust
fn regexp_replace<'a, T: OffsetSizeTrait, U>(string_array: U, pattern_array: U, replacement_array: U, flags_array: Option<U>) -> datafusion_common::Result<arrow::array::ArrayRef> where U: ArrayAccessor<Item = &'a str>
```

Source: `src/regex/regexpreplace.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Replaces substring(s) matching a PCRE-like regular expression.

The full list of supported features and syntax can be found at
<https://docs.rs/regex/latest/regex/#syntax>

Supported flags with the addition of 'g' can be found at
<https://docs.rs/regex/latest/regex/#grouping-and-flags>

# Examples

```ignore
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/regex.csv", CsvReadOptions::new()).await?;

// use the regexp_replace function to replace substring(s) without flags
let df = df.with_column(
    "a",
    regexp_replace(vec![col("values"), col("patterns"), col("replacement")])
)?;
// use the regexp_replace function to replace substring(s) with flags
let df = df.with_column(
    "b",
    regexp_replace(vec![col("values"), col("patterns"), col("replacement"), col("flags")]),
)?;

// literals can be used as well
let df = df.with_column(
    "c",
    regexp_replace(vec![lit("foobarbequebaz"), lit("(bar)(beque)"), lit(r"\2")]),
)?;

df.show().await?;

# Ok(())
# }
```
