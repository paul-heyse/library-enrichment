# `datafusion_substrait::logical_plan::consumer::expr::literal`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.literal.json`](../model/datafusion_substrait.logical_plan.consumer.expr.literal.json)

## from_literal

`function` · `datafusion_substrait::logical_plan::consumer::expr::literal::from_literal`

```rust
async fn from_literal(consumer: &impl SubstraitConsumer, expr: &substrait::proto::expression::Literal) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

---
