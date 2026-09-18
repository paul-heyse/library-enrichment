# `datafusion_substrait::logical_plan::producer::expr`

Crate `datafusion-substrait` · 3 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.json`](../model/datafusion_substrait.logical_plan.producer.expr.json)

## from_alias

`function` · `datafusion_substrait::logical_plan::producer::expr::from_alias`

```rust
fn from_alias(producer: &mut impl SubstraitProducer, alias: &datafusion::logical_expr::expr::Alias, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.expr.from_alias.md).


---

## to_substrait_extended_expr

`function` · `datafusion_substrait::logical_plan::producer::expr::to_substrait_extended_expr`

```rust
fn to_substrait_extended_expr(exprs: &[(&datafusion::logical_expr::Expr, &datafusion::arrow::datatypes::Field)], schema: &datafusion::common::DFSchemaRef, state: &datafusion::execution::SessionState) -> datafusion::common::Result<Box<substrait::proto::ExtendedExpression>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.expr.to_substrait_extended_expr.md).


Serializes a collection of expressions to a Substrait ExtendedExpression message

The ExtendedExpression message is a top-level message that can be used to send
expressions (not plans) between systems.

Each expression is also given names for the output type.  These are provided as a
field and not a String (since the names may be nested, e.g. a struct).  The data
type and nullability of this field is redundant (those can be determined by the
Expr) and will be ignored.

Substrait also requires the input schema of the expressions to be included in the
message.  The field names of the input schema will be serialized.

---

## to_substrait_rex

`function` · `datafusion_substrait::logical_plan::producer::expr::to_substrait_rex`

```rust
fn to_substrait_rex(producer: &mut impl SubstraitProducer, expr: &datafusion::logical_expr::Expr, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.expr.to_substrait_rex.md).


Convert DataFusion Expr to Substrait Rex

# Arguments
* `producer` - SubstraitProducer implementation which the handles the actual conversion
* `expr` - DataFusion expression to convert into a Substrait expression
* `schema` - DataFusion input schema for looking up columns

---
