# `datafusion_proto::physical_plan::to_proto::serialize_physical_window_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_physical_window_expr.json).

<a id="op-9cacc9926875b58bbb072b65"></a>
## serialize_physical_window_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_window_expr` · datafusion-proto 55.1.0

```rust
fn serialize_physical_window_expr(window_expr: &std::sync::Arc<dyn WindowExpr>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalWindowExprNode>
```

Source: `src/physical_plan/to_proto.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
