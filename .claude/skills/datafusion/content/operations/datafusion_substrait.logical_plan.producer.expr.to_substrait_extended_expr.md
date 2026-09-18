# `datafusion_substrait::logical_plan::producer::expr::to_substrait_extended_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.to_substrait_extended_expr.json).

<a id="op-710930669a87f89e08a567f4"></a>
## to_substrait_extended_expr

`function` · `datafusion_substrait::logical_plan::producer::expr::to_substrait_extended_expr` · datafusion-substrait 55.1.0

```rust
fn to_substrait_extended_expr(exprs: &[(&datafusion::logical_expr::Expr, &datafusion::arrow::datatypes::Field)], schema: &datafusion::common::DFSchemaRef, state: &datafusion::execution::SessionState) -> datafusion::common::Result<Box<substrait::proto::ExtendedExpression>>
```

Source: `src/logical_plan/producer/expr/mod.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Serializes a collection of expressions to a Substrait ExtendedExpression message

The ExtendedExpression message is a top-level message that can be used to send
expressions (not plans) between systems.

Each expression is also given names for the output type.  These are provided as a
field and not a String (since the names may be nested, e.g. a struct).  The data
type and nullability of this field is redundant (those can be determined by the
Expr) and will be ignored.

Substrait also requires the input schema of the expressions to be included in the
message.  The field names of the input schema will be serialized.
