# `datafusion_proto::logical_plan::from_proto::parse_sorts`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.from_proto.parse_sorts.json).

<a id="op-ffed5fba61d295712ab21c2c"></a>
## parse_sorts

`function` · `datafusion_proto::logical_plan::from_proto::parse_sorts` · datafusion-proto 55.1.0

```rust
fn parse_sorts<'a, I>(protos: I, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<Vec<datafusion_expr::expr::Sort>, datafusion_proto_common::FromProtoError> where I: IntoIterator<Item = &'a protobuf::SortExprNode>
```

Source: `src/logical_plan/from_proto.rs:641`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
