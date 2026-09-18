# `datafusion_substrait::logical_plan::consumer::expr`

Crate `datafusion-substrait` · 4 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.json`](../model/datafusion_substrait.logical_plan.consumer.expr.json)

## from_substrait_extended_expr

`function` · `datafusion_substrait::logical_plan::consumer::expr::from_substrait_extended_expr`

```rust
async fn from_substrait_extended_expr(state: &datafusion::execution::SessionState, extended_expr: &substrait::proto::ExtendedExpression) -> datafusion::common::Result<ExprContainer>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.from_substrait_extended_expr.md).


Convert Substrait ExtendedExpression to ExprContainer

A Substrait ExtendedExpression message contains one or more expressions,
with names for the outputs, and an input schema.  These pieces are all included
in the ExprContainer.

This is a top-level message and can be used to send expressions (not plans)
between systems.  This is often useful for scenarios like pushdown where filter
expressions need to be sent to remote systems.

---

## from_substrait_rex

`function` · `datafusion_substrait::logical_plan::consumer::expr::from_substrait_rex`

```rust
async fn from_substrait_rex(consumer: &impl SubstraitConsumer, expression: &substrait::proto::Expression, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.from_substrait_rex.md).


Convert Substrait Rex to DataFusion Expr

---

## from_substrait_rex_vec

`function` · `datafusion_substrait::logical_plan::consumer::expr::from_substrait_rex_vec`

```rust
async fn from_substrait_rex_vec(consumer: &impl SubstraitConsumer, exprs: &Vec<substrait::proto::Expression>, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<Vec<datafusion::logical_expr::Expr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.from_substrait_rex_vec.md).


Convert Substrait Expressions to DataFusion Exprs

---

## ExprContainer

`struct` · `datafusion_substrait::logical_plan::consumer::expr::ExprContainer`

```rust
struct ExprContainer
```

**Fields**: `input_schema`, `exprs`

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.ExprContainer.md).


An ExprContainer is a container for a collection of expressions with a common input schema

In addition, each expression is associated with a field, which defines the
expression's output.  The data type and nullability of the field are calculated from the
expression and the input schema.  However the names of the field (and its nested fields) are
derived from the Substrait message.

---
