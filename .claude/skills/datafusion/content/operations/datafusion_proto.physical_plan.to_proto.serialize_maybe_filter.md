# `datafusion_proto::physical_plan::to_proto::serialize_maybe_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_maybe_filter.json).

<a id="op-933d4275aa7bca3ca09db7a2"></a>
## serialize_maybe_filter

`function` · `datafusion_proto::physical_plan::to_proto::serialize_maybe_filter` · datafusion-proto 55.1.0

```rust
fn serialize_maybe_filter(expr: Option<std::sync::Arc<dyn PhysicalExpr>>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::MaybeFilter>
```

Source: `src/physical_plan/to_proto.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
