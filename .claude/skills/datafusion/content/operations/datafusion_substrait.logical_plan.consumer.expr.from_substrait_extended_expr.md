# `datafusion_substrait::logical_plan::consumer::expr::from_substrait_extended_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.from_substrait_extended_expr.json).

<a id="op-b3e4c58bec4dd6a09dda5967"></a>
## from_substrait_extended_expr

`function` · `datafusion_substrait::logical_plan::consumer::expr::from_substrait_extended_expr` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_extended_expr(state: &datafusion::execution::SessionState, extended_expr: &substrait::proto::ExtendedExpression) -> datafusion::common::Result<ExprContainer>
```

Source: `src/logical_plan/consumer/expr/mod.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait ExtendedExpression to ExprContainer

A Substrait ExtendedExpression message contains one or more expressions,
with names for the outputs, and an input schema.  These pieces are all included
in the ExprContainer.

This is a top-level message and can be used to send expressions (not plans)
between systems.  This is often useful for scenarios like pushdown where filter
expressions need to be sent to remote systems.
