# `datafusion_proto::physical_plan::to_proto::serialize_physical_sort_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_physical_sort_expr.json).

<a id="op-fced35a4cb6b373eab751b9d"></a>
## serialize_physical_sort_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_sort_expr` · datafusion-proto 55.1.0

```rust
fn serialize_physical_sort_expr(sort_expr: datafusion_physical_expr_common::sort_expr::PhysicalSortExpr, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalSortExprNode>
```

Source: `src/physical_plan/to_proto.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
