# `datafusion_substrait::logical_plan::consumer::expr::subquery`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.subquery.json`](../model/datafusion_substrait.logical_plan.consumer.expr.subquery.json)

## from_subquery

`function` · `datafusion_substrait::logical_plan::consumer::expr::subquery::from_subquery`

```rust
async fn from_subquery(consumer: &impl SubstraitConsumer, subquery: &substrait_expression::Subquery, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

---
