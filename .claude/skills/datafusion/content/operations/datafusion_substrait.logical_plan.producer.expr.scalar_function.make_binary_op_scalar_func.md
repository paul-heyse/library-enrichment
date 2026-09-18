# `datafusion_substrait::logical_plan::producer::expr::scalar_function::make_binary_op_scalar_func`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.scalar_function.make_binary_op_scalar_func.json).

<a id="op-c8dbddbbae28ad4c858e4bd3"></a>
## make_binary_op_scalar_func

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::make_binary_op_scalar_func` · datafusion-substrait 55.1.0

```rust
fn make_binary_op_scalar_func(producer: &mut impl SubstraitProducer, lhs: &substrait::proto::Expression, rhs: &substrait::proto::Expression, op: datafusion::logical_expr::Operator, output_type: &substrait::proto::Type) -> substrait::proto::Expression
```

Source: `src/logical_plan/producer/expr/scalar_function.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Return Substrait scalar function with two arguments
