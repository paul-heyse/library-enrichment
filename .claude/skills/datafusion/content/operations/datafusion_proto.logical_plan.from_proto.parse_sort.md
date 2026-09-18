# `datafusion_proto::logical_plan::from_proto::parse_sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.from_proto.parse_sort.json).

<a id="op-105600685f4853c7b86b1189"></a>
## parse_sort

`function` · `datafusion_proto::logical_plan::from_proto::parse_sort` · datafusion-proto 55.1.0

```rust
fn parse_sort(sort: &protobuf::SortExprNode, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::expr::Sort, datafusion_proto_common::FromProtoError>
```

Source: `src/logical_plan/from_proto.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
