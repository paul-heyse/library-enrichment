# `datafusion_proto::physical_plan::from_proto::parse_physical_sort_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_physical_sort_exprs.json).

<a id="op-95b08b18c004bec8d23836c6"></a>
## parse_physical_sort_exprs

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_sort_exprs` · datafusion-proto 55.1.0

```rust
fn parse_physical_sort_exprs(proto: &[protobuf::PhysicalSortExprNode], ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<datafusion_physical_expr::PhysicalSortExpr>>
```

Source: `src/physical_plan/from_proto.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Parses a physical sort expressions from a protobuf.

# Arguments

* `proto` - Input proto with vector of physical sort expression node
* `input_schema` - The Arrow schema for the input, used for determining expression data types
  when performing type coercion.
* `ctx` - Decode context carrying the task context, extension codec, and
  any scoped state needed during recursive deserialization.
* `proto_converter` - Converter hooks used for recursive physical plan and
  expression deserialization.
