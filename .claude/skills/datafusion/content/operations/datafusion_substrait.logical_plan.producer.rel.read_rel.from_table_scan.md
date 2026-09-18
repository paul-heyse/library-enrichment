# `datafusion_substrait::logical_plan::producer::rel::read_rel::from_table_scan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.rel.read_rel.from_table_scan.json).

<a id="op-87fae340894dffb42db518ca"></a>
## from_table_scan

`function` · `datafusion_substrait::logical_plan::producer::rel::read_rel::from_table_scan` · datafusion-substrait 55.1.0

```rust
fn from_table_scan(producer: &mut impl SubstraitProducer, scan: &datafusion::logical_expr::TableScan) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

Source: `src/logical_plan/producer/rel/read_rel.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
