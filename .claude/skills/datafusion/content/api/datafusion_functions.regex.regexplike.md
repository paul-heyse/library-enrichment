# `datafusion_functions::regex::regexplike`

Crate `datafusion-functions` · 2 public items · structured records in [`model/datafusion_functions.regex.regexplike.json`](../model/datafusion_functions.regex.regexplike.json)

## regexp_like

`function` · `datafusion_functions::regex::regexplike::regexp_like`

```rust
fn regexp_like(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

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

---

## RegexpLikeFunc

`struct` · `datafusion_functions::regex::regexplike::RegexpLikeFunc`

```rust
struct RegexpLikeFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

---
