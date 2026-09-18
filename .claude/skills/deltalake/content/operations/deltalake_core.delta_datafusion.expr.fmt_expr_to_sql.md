# `deltalake_core::delta_datafusion::expr::fmt_expr_to_sql`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.expr.fmt_expr_to_sql.json).

<a id="op-9daabbfaa3f7090c91b7f545"></a>
## fmt_expr_to_sql

`function` · `deltalake_core::delta_datafusion::expr::fmt_expr_to_sql` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt_expr_to_sql(expr: &datafusion::logical_expr::Expr) -> datafusion::common::Result<String, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/expr.rs#L649).

Source: `crates/core/src/delta_datafusion/expr.rs:649`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Format an `Expr` to a parsable SQL expression
