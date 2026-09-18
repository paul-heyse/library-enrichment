# `datafusion_substrait::logical_plan::producer::rel::filter_rel::from_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.rel.filter_rel.from_filter.json).

<a id="op-26ffe7e59f99da026778add2"></a>
## from_filter

`function` · `datafusion_substrait::logical_plan::producer::rel::filter_rel::from_filter` · datafusion-substrait 55.1.0

```rust
fn from_filter(producer: &mut impl SubstraitProducer, filter: &datafusion::logical_expr::Filter) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

Source: `src/logical_plan/producer/rel/filter_rel.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
