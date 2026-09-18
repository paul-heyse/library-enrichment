# `datafusion_proto::logical_plan::from_proto::parse_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.from_proto.parse_exprs.json).

<a id="op-fb1c7be44579b56211354c37"></a>
## parse_exprs

`function` · `datafusion_proto::logical_plan::from_proto::parse_exprs` · datafusion-proto 55.1.0

```rust
fn parse_exprs<'a, I>(protos: I, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<Vec<datafusion_expr::Expr>, datafusion_proto_common::FromProtoError> where I: IntoIterator<Item = &'a protobuf::LogicalExprNode>
```

Source: `src/logical_plan/from_proto.rs:624`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Parse a vector of `protobuf::LogicalExprNode`s.
