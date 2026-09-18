# `datafusion_proto::physical_plan::to_proto::serialize_physical_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_physical_exprs.json).

<a id="op-7046dd3bb90fcd21f78b977e"></a>
## serialize_physical_exprs

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_exprs` · datafusion-proto 55.1.0

```rust
fn serialize_physical_exprs<'a, I>(values: I, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<protobuf::PhysicalExprNode>> where I: IntoIterator<Item = &'a std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/physical_plan/to_proto.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
