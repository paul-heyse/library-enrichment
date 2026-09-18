# `deltalake_core::delta_datafusion::expr::parse_predicate_expression`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.expr.parse_predicate_expression.json).

<a id="op-e38c85fcbb9b4c341d26f807"></a>
## parse_predicate_expression

`function` · `deltalake_core::delta_datafusion::expr::parse_predicate_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_predicate_expression(schema: &datafusion::common::DFSchema, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<datafusion::logical_expr::Expr>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/expr.rs#L290).

Source: `crates/core/src/delta_datafusion/expr.rs:290`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse a string predicate into an `Expr`
