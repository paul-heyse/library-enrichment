# `datafusion_substrait::logical_plan::producer::expr::to_substrait_rex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.to_substrait_rex.json).

<a id="op-ac32b021607f078e31bc6c19"></a>
## to_substrait_rex

`function` · `datafusion_substrait::logical_plan::producer::expr::to_substrait_rex` · datafusion-substrait 55.1.0

```rust
fn to_substrait_rex(producer: &mut impl SubstraitProducer, expr: &datafusion::logical_expr::Expr, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/mod.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert DataFusion Expr to Substrait Rex

# Arguments
* `producer` - SubstraitProducer implementation which the handles the actual conversion
* `expr` - DataFusion expression to convert into a Substrait expression
* `schema` - DataFusion input schema for looking up columns
