# `datafusion_substrait::logical_plan::consumer::types::from_substrait_named_struct`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.types.from_substrait_named_struct.json).

<a id="op-6d71aa99333aa9a11a58a642"></a>
## from_substrait_named_struct

`function` · `datafusion_substrait::logical_plan::consumer::types::from_substrait_named_struct` · datafusion-substrait 55.1.0

```rust
fn from_substrait_named_struct(consumer: &impl SubstraitConsumer, base_schema: &substrait::proto::NamedStruct) -> datafusion::common::Result<datafusion::common::DFSchema>
```

Source: `src/logical_plan/consumer/types.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait NamedStruct to DataFusion DFSchemaRef
