# `datafusion_proto::logical_plan::to_proto::serialize_sorts`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.to_proto.serialize_sorts.json).

<a id="op-e8807c4daa6072ef7ddb3545"></a>
## serialize_sorts

`function` · `datafusion_proto::logical_plan::to_proto::serialize_sorts` · datafusion-proto 55.1.0

```rust
fn serialize_sorts<'a, I>(sorts: I, codec: &dyn LogicalExtensionCodec) -> Result<Vec<protobuf::SortExprNode>, protobuf::ToProtoError> where I: IntoIterator<Item = &'a datafusion_expr::SortExpr>
```

Source: `src/logical_plan/to_proto.rs:545`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
