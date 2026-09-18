# `datafusion_substrait::logical_plan::consumer::expr::from_substrait_rex_vec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.from_substrait_rex_vec.json).

<a id="op-a029a0477ec019676cb16cea"></a>
## from_substrait_rex_vec

`function` · `datafusion_substrait::logical_plan::consumer::expr::from_substrait_rex_vec` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_rex_vec(consumer: &impl SubstraitConsumer, exprs: &Vec<substrait::proto::Expression>, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<Vec<datafusion::logical_expr::Expr>>
```

Source: `src/logical_plan/consumer/expr/mod.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait Expressions to DataFusion Exprs
