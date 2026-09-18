# `datafusion_proto::physical_plan::from_proto::parse_physical_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_physical_exprs.json).

<a id="op-a3ff58bf9225b5097c43f32c"></a>
## parse_physical_exprs

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_exprs` · datafusion-proto 55.1.0

```rust
fn parse_physical_exprs<'a, I>(protos: I, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<std::sync::Arc<dyn PhysicalExpr>>> where I: IntoIterator<Item = &'a protobuf::PhysicalExprNode>
```

Source: `src/physical_plan/from_proto.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
