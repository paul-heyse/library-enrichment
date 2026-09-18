# `datafusion_proto::physical_plan::to_proto::serialize_physical_aggr_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_physical_aggr_expr.json).

<a id="op-dee7ea3ae6a9dbb5b1661b31"></a>
## serialize_physical_aggr_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_aggr_expr` · datafusion-proto 55.1.0

```rust
fn serialize_physical_aggr_expr(aggr_expr: std::sync::Arc<datafusion_physical_plan::udaf::AggregateFunctionExpr>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalExprNode>
```

Source: `src/physical_plan/to_proto.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
