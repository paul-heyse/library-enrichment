# `datafusion_proto::logical_plan::from_proto::parse_write_op`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.from_proto.parse_write_op.json).

<a id="op-cc0568c443fe1bae4c3605f0"></a>
## parse_write_op

`function` · `datafusion_proto::logical_plan::from_proto::parse_write_op` · datafusion-proto 55.1.0

```rust
fn parse_write_op(node: &protobuf::DmlNode, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::WriteOp, datafusion_proto_common::FromProtoError>
```

Source: `src/logical_plan/from_proto.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Reconstruct a [`WriteOp`](../operations/datafusion_expr.logical_plan.dml.WriteOp.md#op-18e21f9cfeca26c2ee4449fa) from a [`protobuf::DmlNode`](../operations/datafusion_proto_models.generated.datafusion.DmlNode.md#op-5f6a85ca652beb2607298e9d), reading the
`merge_into` payload when the type tag is `MergeInto`.
