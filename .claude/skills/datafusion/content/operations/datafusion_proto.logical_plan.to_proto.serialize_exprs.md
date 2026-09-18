# `datafusion_proto::logical_plan::to_proto::serialize_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.to_proto.serialize_exprs.json).

<a id="op-43e1c8736c9538eed7e06a3c"></a>
## serialize_exprs

`function` · `datafusion_proto::logical_plan::to_proto::serialize_exprs` · datafusion-proto 55.1.0

```rust
fn serialize_exprs<'a, I>(exprs: I, codec: &dyn LogicalExtensionCodec) -> Result<Vec<protobuf::LogicalExprNode>, protobuf::ToProtoError> where I: IntoIterator<Item = &'a datafusion_expr::Expr>
```

Source: `src/logical_plan/to_proto.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
