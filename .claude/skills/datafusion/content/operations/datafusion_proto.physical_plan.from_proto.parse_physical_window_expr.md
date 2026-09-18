# `datafusion_proto::physical_plan::from_proto::parse_physical_window_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_physical_window_expr.json).

<a id="op-6c260b37a02500617683c45c"></a>
## parse_physical_window_expr

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_window_expr` · datafusion-proto 55.1.0

```rust
fn parse_physical_window_expr(proto: &protobuf::PhysicalWindowExprNode, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<std::sync::Arc<dyn WindowExpr>>
```

Source: `src/physical_plan/from_proto.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Parses a physical window expr from a protobuf.

# Arguments

* `proto` - Input proto with physical window expression node.
* `name` - Name of the window expression.
* `input_schema` - The Arrow schema for the input, used for determining
  expression data types when performing type coercion.
* `ctx` - Decode context carrying the task context, extension codec, and
  any scoped state needed during recursive deserialization.
* `proto_converter` - Converter hooks used for recursive physical plan and
  expression deserialization.
