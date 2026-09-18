# `datafusion_substrait::logical_plan::consumer::expr::scalar_function`

Crate `datafusion-substrait` · 4 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.scalar_function.json`](../model/datafusion_substrait.logical_plan.consumer.expr.scalar_function.json)

## from_scalar_function

`function` · `datafusion_substrait::logical_plan::consumer::expr::scalar_function::from_scalar_function`

```rust
async fn from_scalar_function(consumer: &impl SubstraitConsumer, f: &substrait::proto::expression::ScalarFunction, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.scalar_function.from_scalar_function.md).


---

## name_to_op

`function` · `datafusion_substrait::logical_plan::consumer::expr::scalar_function::name_to_op`

```rust
fn name_to_op(name: &str) -> Option<datafusion::logical_expr::Operator>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.scalar_function.name_to_op.md).


---

## substrait_fun_name

`function` · `datafusion_substrait::logical_plan::consumer::expr::scalar_function::substrait_fun_name`

```rust
fn substrait_fun_name(name: &str) -> &str
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.scalar_function.substrait_fun_name.md).


---

## substrait_to_df_name

`function` · `datafusion_substrait::logical_plan::consumer::expr::scalar_function::substrait_to_df_name`

```rust
fn substrait_to_df_name(name: &str) -> Option<&str>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.scalar_function.substrait_to_df_name.md).


---
