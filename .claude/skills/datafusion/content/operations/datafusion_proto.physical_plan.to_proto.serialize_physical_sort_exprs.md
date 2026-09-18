# `datafusion_proto::physical_plan::to_proto::serialize_physical_sort_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_physical_sort_exprs.json).

<a id="op-fc45ac301ca356c16cfe26e5"></a>
## serialize_physical_sort_exprs

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_sort_exprs` · datafusion-proto 55.1.0

```rust
fn serialize_physical_sort_exprs<I>(sort_exprs: I, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<protobuf::PhysicalSortExprNode>> where I: IntoIterator<Item = datafusion_physical_expr_common::sort_expr::PhysicalSortExpr>
```

Source: `src/physical_plan/to_proto.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
