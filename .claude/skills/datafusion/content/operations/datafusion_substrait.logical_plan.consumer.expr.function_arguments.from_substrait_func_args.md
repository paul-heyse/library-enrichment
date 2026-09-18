# `datafusion_substrait::logical_plan::consumer::expr::function_arguments::from_substrait_func_args`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.function_arguments.from_substrait_func_args.json).

<a id="op-abab3941c101f3b16d58b627"></a>
## from_substrait_func_args

`function` · `datafusion_substrait::logical_plan::consumer::expr::function_arguments::from_substrait_func_args` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_func_args(consumer: &impl SubstraitConsumer, arguments: &Vec<substrait::proto::FunctionArgument>, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<Vec<datafusion::logical_expr::Expr>>
```

Source: `src/logical_plan/consumer/expr/function_arguments.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait FunctionArguments to DataFusion Exprs
