# `datafusion::dataframe`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.dataframe.json).

<a id="op-896f9ad2781e11ab4f33b656"></a>
## dataframe

`macro` · `datafusion::dataframe` · datafusion 55.1.0

```rust
macro_rules! dataframe
```

Source: `src/dataframe/mod.rs:2664`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Macro for creating DataFrame.
# Example
```
use datafusion::prelude::dataframe;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let df = dataframe!(
   "id" => [1, 2, 3],
   "name" => ["foo", "bar", "baz"]
 )?;
df.show().await?;
// +----+------+,
// | id | name |,
// +----+------+,
// | 1  | foo  |,
// | 2  | bar  |,
// | 3  | baz  |,
// +----+------+,
let df_empty = dataframe!()?; // empty DataFrame
assert_eq!(df_empty.schema().fields().len(), 0);
assert_eq!(df_empty.count().await?, 0);
# Ok(())
# }
```
