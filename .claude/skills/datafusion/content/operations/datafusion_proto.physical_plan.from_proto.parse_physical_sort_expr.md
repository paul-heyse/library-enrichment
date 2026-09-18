# `datafusion_proto::physical_plan::from_proto::parse_physical_sort_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_physical_sort_expr.json).

<a id="op-3019efb780d3d367ff7cfc89"></a>
## parse_physical_sort_expr

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_sort_expr` · datafusion-proto 55.1.0

```rust
fn parse_physical_sort_expr(proto: &protobuf::PhysicalSortExprNode, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<datafusion_physical_expr::PhysicalSortExpr>
```

Source: `src/physical_plan/from_proto.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Parses a physical sort expression from a protobuf.

# Arguments

* `proto` - Input proto with physical sort expression node
* `input_schema` - The Arrow schema for the input, used for determining expression data types
  when performing type coercion.
* `ctx` - Decode context carrying the task context, extension codec, and
  any scoped state needed during recursive deserialization.
* `proto_converter` - Converter hooks used for recursive physical plan and
  expression deserialization.
