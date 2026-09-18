# `datafusion::test_util::plan_and_collect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.plan_and_collect.json).

<a id="op-64579b72e2b3cd52aa9da20b"></a>
## plan_and_collect

`function` · `datafusion::test_util::plan_and_collect` · datafusion 55.1.0

```rust
async fn plan_and_collect(ctx: &prelude::SessionContext, sql: &str) -> error::Result<Vec<arrow::record_batch::RecordBatch>>
```

Source: `src/test_util/mod.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute SQL and return results
