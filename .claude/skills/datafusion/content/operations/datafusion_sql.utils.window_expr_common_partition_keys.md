# `datafusion_sql::utils::window_expr_common_partition_keys`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.utils.window_expr_common_partition_keys.json).

<a id="op-0c064fb87040f78484d8b337"></a>
## window_expr_common_partition_keys

`function` · `datafusion_sql::utils::window_expr_common_partition_keys` · datafusion-sql 55.1.0

```rust
fn window_expr_common_partition_keys(window_exprs: &[datafusion_expr::Expr]) -> datafusion_common::Result<&[datafusion_expr::Expr]>
```

Source: `src/utils.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Given a slice of window expressions sharing the same sort key, find their common partition
keys.
