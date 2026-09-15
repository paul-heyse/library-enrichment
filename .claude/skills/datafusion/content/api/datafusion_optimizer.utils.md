# `datafusion_optimizer::utils`

Crate `datafusion-optimizer` · 3 public items · structured records in [`model/datafusion_optimizer.utils.json`](../model/datafusion_optimizer.utils.json)

## evaluates_to_null

`function` · `datafusion_optimizer::utils::evaluates_to_null`

```rust
fn evaluates_to_null<'a>(predicate: datafusion_expr::Expr, null_columns: impl IntoIterator<Item = &'a datafusion_common::Column>) -> datafusion_common::Result<bool>
```

Determines if an expression will always evaluate to null.
`c0 + 8` return true
`c0 IS NULL` return false
`CASE WHEN c0 > 1 then 0 else 1` return false

---

## is_restrict_null_predicate

`function` · `datafusion_optimizer::utils::is_restrict_null_predicate`

```rust
fn is_restrict_null_predicate<'a>(predicate: datafusion_expr::Expr, join_cols_of_predicate: impl IntoIterator<Item = &'a datafusion_common::Column>) -> datafusion_common::Result<bool>
```

Determine whether a predicate can restrict NULLs. e.g.
`c0 > 8` return true;
`c0 IS NULL` return false.

---

## log_plan

`function` · `datafusion_optimizer::utils::log_plan`

```rust
fn log_plan(description: &str, plan: &datafusion_expr::logical_plan::LogicalPlan)
```

Log the plan in debug/tracing mode after some part of the optimizer runs

---
