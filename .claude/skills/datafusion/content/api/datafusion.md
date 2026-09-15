# `datafusion`

Crate `datafusion` · 2 public items · structured records in [`model/datafusion.json`](../model/datafusion.json)

## DATAFUSION_VERSION

`constant` · `datafusion::DATAFUSION_VERSION`

```rust
const DATAFUSION_VERSION: &str = "55.1.0"
```

DataFusion crate version

---

## dataframe

`macro` · `datafusion::dataframe`

Also reachable as `datafusion::prelude::dataframe`

```rust
macro_rules! dataframe
```

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

---
