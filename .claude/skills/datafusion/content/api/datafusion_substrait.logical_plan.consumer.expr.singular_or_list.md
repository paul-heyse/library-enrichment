# `datafusion_substrait::logical_plan::consumer::expr::singular_or_list`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.singular_or_list.json`](../model/datafusion_substrait.logical_plan.consumer.expr.singular_or_list.json)

## from_singular_or_list

`function` · `datafusion_substrait::logical_plan::consumer::expr::singular_or_list::from_singular_or_list`

```rust
async fn from_singular_or_list(consumer: &impl SubstraitConsumer, expr: &substrait::proto::expression::SingularOrList, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

---
