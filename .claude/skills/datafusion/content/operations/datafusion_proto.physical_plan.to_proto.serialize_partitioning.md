# `datafusion_proto::physical_plan::to_proto::serialize_partitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_partitioning.json).

<a id="op-d76b65b105c44cd858175719"></a>
## serialize_partitioning

`function` · `datafusion_proto::physical_plan::to_proto::serialize_partitioning` · datafusion-proto 55.1.0

```rust
fn serialize_partitioning(partitioning: &datafusion_physical_plan::Partitioning, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::Partitioning>
```

Source: `src/physical_plan/to_proto.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
