# `datafusion_substrait::logical_plan::producer::expr::scalar_function`

Crate `datafusion-substrait` · 9 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.scalar_function.json`](../model/datafusion_substrait.logical_plan.producer.expr.scalar_function.json)

## custom_argument_handler

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::custom_argument_handler`

```rust
fn custom_argument_handler(name: &str, args: Vec<substrait::proto::FunctionArgument>) -> Vec<substrait::proto::FunctionArgument>
```

---

## from_between

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_between`

```rust
fn from_between(producer: &mut impl SubstraitProducer, between: &datafusion::logical_expr::Between, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## from_binary_expr

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_binary_expr`

```rust
fn from_binary_expr(producer: &mut impl SubstraitProducer, expr: &datafusion::logical_expr::BinaryExpr, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## from_higher_order_function

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_higher_order_function`

```rust
fn from_higher_order_function(producer: &mut impl SubstraitProducer, fun: &expr::HigherOrderFunction, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## from_like

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_like`

```rust
fn from_like(producer: &mut impl SubstraitProducer, like: &datafusion::logical_expr::Like, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## from_scalar_function

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_scalar_function`

```rust
fn from_scalar_function(producer: &mut impl SubstraitProducer, fun: &expr::ScalarFunction, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## from_unary_expr

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_unary_expr`

```rust
fn from_unary_expr(producer: &mut impl SubstraitProducer, expr: &datafusion::logical_expr::Expr, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## make_binary_op_scalar_func

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::make_binary_op_scalar_func`

```rust
fn make_binary_op_scalar_func(producer: &mut impl SubstraitProducer, lhs: &substrait::proto::Expression, rhs: &substrait::proto::Expression, op: datafusion::logical_expr::Operator, output_type: &substrait::proto::Type) -> substrait::proto::Expression
```

Return Substrait scalar function with two arguments

---

## operator_to_name

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::operator_to_name`

```rust
fn operator_to_name(op: datafusion::logical_expr::Operator) -> &'static str
```

---
