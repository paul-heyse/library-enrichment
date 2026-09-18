# `datafusion_substrait::logical_plan::consumer::types::from_substrait_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.types.from_substrait_type.json).

<a id="op-dcfd7fa505a55828fe66765d"></a>
## from_substrait_type

`function` · `datafusion_substrait::logical_plan::consumer::types::from_substrait_type` · datafusion-substrait 55.1.0

```rust
fn from_substrait_type(consumer: &impl SubstraitConsumer, dt: &substrait::proto::Type, dfs_names: &[String], name_idx: &mut usize) -> datafusion::common::Result<datafusion::arrow::datatypes::DataType>
```

Source: `src/logical_plan/consumer/types.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
