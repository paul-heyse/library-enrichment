# `datafusion_proto::logical_plan::to_proto::serialize_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.to_proto.serialize_expr.json).

<a id="op-860d88c2a1c85353a2579d96"></a>
## serialize_expr

`function` · `datafusion_proto::logical_plan::to_proto::serialize_expr` · datafusion-proto 55.1.0

```rust
fn serialize_expr(expr: &datafusion_expr::Expr, codec: &dyn LogicalExtensionCodec) -> Result<protobuf::LogicalExprNode, protobuf::ToProtoError>
```

Source: `src/logical_plan/to_proto.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
