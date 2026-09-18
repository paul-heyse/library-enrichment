# `datafusion_substrait::logical_plan::consumer::expr::aggregate_function::from_substrait_agg_func`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.aggregate_function.from_substrait_agg_func.json).

<a id="op-c066be65d85c295e45e71cdd"></a>
## from_substrait_agg_func

`function` · `datafusion_substrait::logical_plan::consumer::expr::aggregate_function::from_substrait_agg_func` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_agg_func(consumer: &impl SubstraitConsumer, f: &substrait::proto::AggregateFunction, input_schema: &datafusion::common::DFSchema, filter: Option<Box<datafusion::logical_expr::Expr>>, order_by: Vec<datafusion::logical_expr::SortExpr>, distinct: bool) -> datafusion::common::Result<std::sync::Arc<datafusion::logical_expr::Expr>>
```

Source: `src/logical_plan/consumer/expr/aggregate_function.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait AggregateFunction to DataFusion Expr
