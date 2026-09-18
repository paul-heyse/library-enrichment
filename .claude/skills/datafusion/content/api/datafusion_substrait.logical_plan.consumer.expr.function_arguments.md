# `datafusion_substrait::logical_plan::consumer::expr::function_arguments`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.function_arguments.json`](../model/datafusion_substrait.logical_plan.consumer.expr.function_arguments.json)

## from_substrait_func_args

`function` · `datafusion_substrait::logical_plan::consumer::expr::function_arguments::from_substrait_func_args`

```rust
async fn from_substrait_func_args(consumer: &impl SubstraitConsumer, arguments: &Vec<substrait::proto::FunctionArgument>, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<Vec<datafusion::logical_expr::Expr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.function_arguments.from_substrait_func_args.md).


Convert Substrait FunctionArguments to DataFusion Exprs

---
