# `datafusion_proto::physical_plan::from_proto::parse_physical_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_physical_expr.json).

<a id="op-d93a8ff095273b786a92b7dc"></a>
## parse_physical_expr

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_expr` · datafusion-proto 55.1.0

```rust
fn parse_physical_expr(proto: &protobuf::PhysicalExprNode, ctx: &datafusion_execution::TaskContext, input_schema: &arrow::datatypes::Schema, codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/physical_plan/from_proto.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Parses a physical expression from a protobuf.

# Arguments

* `proto` - Input proto with physical expression node
* `ctx` - Task context used to resolve registered functions.
* `input_schema` - The Arrow schema for the input, used for determining
  expression data types when performing type coercion.
* `codec` - Physical extension codec used to construct the root decode
  context for deserialization.
