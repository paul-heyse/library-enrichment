# `deltalake_core::delta_datafusion::expr`

Crate `deltalake-core` · 4 public items · structured records in [`model/deltalake_core.delta_datafusion.expr.json`](../model/deltalake_core.delta_datafusion.expr.json)

## EPOCH_DAYS_FROM_CE

`constant` · `deltalake_core::delta_datafusion::expr::EPOCH_DAYS_FROM_CE`

Also reachable as `deltalake::delta_datafusion::expr::EPOCH_DAYS_FROM_CE`

```rust
const EPOCH_DAYS_FROM_CE: i32 = 719_163
```

Epoch days from ce calendar until 1970-01-01

---

## fmt_expr_to_sql

`function` · `deltalake_core::delta_datafusion::expr::fmt_expr_to_sql`

Also reachable as `deltalake::delta_datafusion::expr::fmt_expr_to_sql`

```rust
fn fmt_expr_to_sql(expr: &datafusion::logical_expr::Expr) -> datafusion::common::Result<String, DeltaTableError>
```

Format an `Expr` to a parsable SQL expression

---

## parse_predicate_expression

`function` · `deltalake_core::delta_datafusion::expr::parse_predicate_expression`

Also reachable as `deltalake::delta_datafusion::expr::parse_predicate_expression`

```rust
fn parse_predicate_expression(schema: &datafusion::common::DFSchema, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<datafusion::logical_expr::Expr>
```

Parse a string predicate into an `Expr`

---

## parse_sql_predicate_to_kernel

`function` · `deltalake_core::delta_datafusion::expr::parse_sql_predicate_to_kernel`

Also reachable as `deltalake::delta_datafusion::expr::parse_sql_predicate_to_kernel`

```rust
fn parse_sql_predicate_to_kernel(predicate: impl AsRef<str>, table_schema: &delta_kernel::schema::StructType, session: &dyn Session) -> DeltaResult<delta_kernel::expressions::Predicate>
```

Parse a SQL predicate string into a kernel [`Predicate`] for file skipping
during log replay.

The predicate is resolved against the table's logical schema, coercing
literals to the referenced column types the same way operation predicates
(delete, update, merge) are resolved. Only constructs the kernel can
evaluate against file-level metadata are accepted.

---
