# `deltalake_core::delta_datafusion::expr::parse_sql_predicate_to_kernel`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.expr.parse_sql_predicate_to_kernel.json).

<a id="op-a8a79ec70475a56063ee458f"></a>
## parse_sql_predicate_to_kernel

`function` · `deltalake_core::delta_datafusion::expr::parse_sql_predicate_to_kernel` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_sql_predicate_to_kernel(predicate: impl AsRef<str>, table_schema: &delta_kernel::schema::StructType, session: &dyn Session) -> DeltaResult<delta_kernel::expressions::Predicate>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/expr.rs#L305).

Source: `crates/core/src/delta_datafusion/expr.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse a SQL predicate string into a kernel [`Predicate`](../operations/buoyant_kernel.expressions.Predicate.md#op-0c0ee21b4ebfcd851e64ceb4) for file skipping
during log replay.

The predicate is resolved against the table's logical schema, coercing
literals to the referenced column types the same way operation predicates
(delete, update, merge) are resolved. Only constructs the kernel can
evaluate against file-level metadata are accepted.
