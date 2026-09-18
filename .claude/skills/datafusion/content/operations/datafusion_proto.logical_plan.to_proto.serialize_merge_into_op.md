# `datafusion_proto::logical_plan::to_proto::serialize_merge_into_op`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.to_proto.serialize_merge_into_op.json).

<a id="op-614fa91ad0055d60286716a2"></a>
## serialize_merge_into_op

`function` · `datafusion_proto::logical_plan::to_proto::serialize_merge_into_op` · datafusion-proto 55.1.0

```rust
fn serialize_merge_into_op(op: &datafusion_expr::dml::MergeIntoOp, codec: &dyn LogicalExtensionCodec) -> Result<protobuf::MergeIntoOpNode, protobuf::ToProtoError>
```

Source: `src/logical_plan/to_proto.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
