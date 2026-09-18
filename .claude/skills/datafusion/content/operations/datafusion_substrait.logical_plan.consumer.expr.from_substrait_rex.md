# `datafusion_substrait::logical_plan::consumer::expr::from_substrait_rex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.from_substrait_rex.json).

<a id="op-1899a1f2099109d69e5b39e5"></a>
## from_substrait_rex

`function` · `datafusion_substrait::logical_plan::consumer::expr::from_substrait_rex` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_rex(consumer: &impl SubstraitConsumer, expression: &substrait::proto::Expression, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/mod.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait Rex to DataFusion Expr
