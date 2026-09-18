# `datafusion_substrait::logical_plan::producer::expr::subquery::from_exists`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.subquery.from_exists.json).

<a id="op-1f89ed72298e57e93ae96068"></a>
## from_exists

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_exists` · datafusion-substrait 55.1.0

```rust
fn from_exists(producer: &mut impl SubstraitProducer, exists: &datafusion::logical_expr::expr::Exists, _schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/subquery.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert DataFusion Exists expression to Substrait SetPredicate subquery type
