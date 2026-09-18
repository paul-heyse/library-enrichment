# `datafusion_substrait::logical_plan::producer::expr::from_alias`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.from_alias.json).

<a id="op-2df497c095aaf0f294143538"></a>
## from_alias

`function` · `datafusion_substrait::logical_plan::producer::expr::from_alias` · datafusion-substrait 55.1.0

```rust
fn from_alias(producer: &mut impl SubstraitProducer, alias: &datafusion::logical_expr::expr::Alias, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/mod.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
