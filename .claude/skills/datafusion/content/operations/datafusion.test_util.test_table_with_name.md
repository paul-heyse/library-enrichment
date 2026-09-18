# `datafusion::test_util::test_table_with_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.test_table_with_name.json).

<a id="op-6abfdf892aa97e57bf48b4b1"></a>
## test_table_with_name

`function` · `datafusion::test_util::test_table_with_name` · datafusion 55.1.0

```rust
async fn test_table_with_name(name: &str) -> error::Result<dataframe::DataFrame>
```

Source: `src/test_util/mod.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a table from the aggregate_test_100.csv file with the specified name
