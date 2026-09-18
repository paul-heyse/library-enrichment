# `datafusion_proto::physical_plan::from_proto::parse_protobuf_hash_partitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_protobuf_hash_partitioning.json).

<a id="op-b266be09422fb1f5bece1bef"></a>
## parse_protobuf_hash_partitioning

`function` · `datafusion_proto::physical_plan::from_proto::parse_protobuf_hash_partitioning` · datafusion-proto 55.1.0

```rust
fn parse_protobuf_hash_partitioning(partitioning: Option<&protobuf::PhysicalHashRepartition>, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Option<datafusion_physical_plan::Partitioning>>
```

Source: `src/physical_plan/from_proto.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
