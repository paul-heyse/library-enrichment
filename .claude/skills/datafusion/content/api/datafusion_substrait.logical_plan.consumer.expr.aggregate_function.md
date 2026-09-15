# `datafusion_substrait::logical_plan::consumer::expr::aggregate_function`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.aggregate_function.json`](../model/datafusion_substrait.logical_plan.consumer.expr.aggregate_function.json)

## from_substrait_agg_func

`function` · `datafusion_substrait::logical_plan::consumer::expr::aggregate_function::from_substrait_agg_func`

```rust
async fn from_substrait_agg_func(consumer: &impl SubstraitConsumer, f: &substrait::proto::AggregateFunction, input_schema: &datafusion::common::DFSchema, filter: Option<Box<datafusion::logical_expr::Expr>>, order_by: Vec<datafusion::logical_expr::SortExpr>, distinct: bool) -> datafusion::common::Result<std::sync::Arc<datafusion::logical_expr::Expr>>
```

Convert Substrait AggregateFunction to DataFusion Expr

---
