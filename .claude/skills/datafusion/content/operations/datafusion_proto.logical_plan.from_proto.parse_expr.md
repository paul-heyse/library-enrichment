# `datafusion_proto::logical_plan::from_proto::parse_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.from_proto.parse_expr.json).

<a id="op-eed5d46438a5ec15a89070f8"></a>
## parse_expr

`function` · `datafusion_proto::logical_plan::from_proto::parse_expr` · datafusion-proto 55.1.0

```rust
fn parse_expr(proto: &protobuf::LogicalExprNode, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::Expr, datafusion_proto_common::FromProtoError>
```

Source: `src/logical_plan/from_proto.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
