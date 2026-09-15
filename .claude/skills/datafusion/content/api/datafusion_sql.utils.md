# `datafusion_sql::utils`

Crate `datafusion-sql` · 2 public items · structured records in [`model/datafusion_sql.utils.json`](../model/datafusion_sql.utils.json)

## UNNEST_PLACEHOLDER

`constant` · `datafusion_sql::utils::UNNEST_PLACEHOLDER`

```rust
const UNNEST_PLACEHOLDER: &str = "__unnest_placeholder"
```

---

## window_expr_common_partition_keys

`function` · `datafusion_sql::utils::window_expr_common_partition_keys`

```rust
fn window_expr_common_partition_keys(window_exprs: &[datafusion_expr::Expr]) -> datafusion_common::Result<&[datafusion_expr::Expr]>
```

Given a slice of window expressions sharing the same sort key, find their common partition
keys.

---
