# `datafusion_substrait::logical_plan::consumer::expr::nested::from_nested`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.nested.from_nested.json).

<a id="op-94e114c4cc10c78b3641e7c0"></a>
## from_nested

`function` · `datafusion_substrait::logical_plan::consumer::expr::nested::from_nested` · datafusion-substrait 55.1.0

```rust
async fn from_nested(consumer: &impl SubstraitConsumer, nested: &substrait::proto::expression::Nested, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/nested.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Converts a Substrait [Nested] expression into a DataFusion [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc).

Substrait Nested expressions represent complex type constructors (list, struct, map)
where elements are full expressions rather than just literals. This is used by
producers that emit `Nested { list: ... }` for array construction, as opposed to
`Literal { list: ... }` which only supports scalar values.

Unresolved upstream links (retained, not inferred): `Nested`.
