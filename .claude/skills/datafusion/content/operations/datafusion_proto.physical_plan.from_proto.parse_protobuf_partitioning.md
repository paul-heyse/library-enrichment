# `datafusion_proto::physical_plan::from_proto::parse_protobuf_partitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_protobuf_partitioning.json).

<a id="op-9732ef84cc005c8f186cc424"></a>
## parse_protobuf_partitioning

`function` · `datafusion_proto::physical_plan::from_proto::parse_protobuf_partitioning` · datafusion-proto 55.1.0

```rust
fn parse_protobuf_partitioning(partitioning: Option<&protobuf::Partitioning>, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Option<datafusion_physical_plan::Partitioning>>
```

Source: `src/physical_plan/from_proto.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
