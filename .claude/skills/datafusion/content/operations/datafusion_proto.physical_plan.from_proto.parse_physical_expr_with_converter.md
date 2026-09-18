# `datafusion_proto::physical_plan::from_proto::parse_physical_expr_with_converter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_physical_expr_with_converter.json).

<a id="op-b465e80e81c63f5aaa28a75b"></a>
## parse_physical_expr_with_converter

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_expr_with_converter` · datafusion-proto 55.1.0

```rust
fn parse_physical_expr_with_converter(proto: &protobuf::PhysicalExprNode, input_schema: &arrow::datatypes::Schema, ctx: &super::PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/physical_plan/from_proto.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Parses a physical expression from a protobuf.

# Arguments

* `proto` - Input proto with physical expression node
* `input_schema` - The Arrow schema for the input, used for determining
  expression data types when performing type coercion.
* `ctx` - Decode context carrying the task context, extension codec, and
  any scoped state needed during recursive deserialization.
* `proto_converter` - Converter hooks used for recursive physical plan and
  expression deserialization.
