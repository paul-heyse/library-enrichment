# `datafusion_substrait::logical_plan::producer::plan::from_subquery_alias`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.plan.from_subquery_alias.json).

<a id="op-f835eb2b87c575e549fe517b"></a>
## from_subquery_alias

`function` · `datafusion_substrait::logical_plan::producer::plan::from_subquery_alias` · datafusion-substrait 55.1.0

```rust
fn from_subquery_alias(producer: &mut impl SubstraitProducer, alias: &datafusion::logical_expr::SubqueryAlias) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

Source: `src/logical_plan/producer/plan.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
