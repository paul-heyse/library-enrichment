# `datafusion_proto::physical_plan::from_proto`

Crate `datafusion-proto` · 12 public items · structured records in [`model/datafusion_proto.physical_plan.from_proto.json`](../model/datafusion_proto.physical_plan.from_proto.json)

## parse_physical_expr

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_expr`

```rust
fn parse_physical_expr(proto: &protobuf::PhysicalExprNode, ctx: &datafusion_execution::TaskContext, input_schema: &arrow::datatypes::Schema, codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_physical_expr.md).


Parses a physical expression from a protobuf.

# Arguments

* `proto` - Input proto with physical expression node
* `ctx` - Task context used to resolve registered functions.
* `input_schema` - The Arrow schema for the input, used for determining
  expression data types when performing type coercion.
* `codec` - Physical extension codec used to construct the root decode
  context for deserialization.

---

## parse_physical_expr_with_converter

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_expr_with_converter`

```rust
fn parse_physical_expr_with_converter(proto: &protobuf::PhysicalExprNode, input_schema: &arrow::datatypes::Schema, ctx: &super::PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_physical_expr_with_converter.md).


Parses a physical expression from a protobuf.

# Arguments

* `proto` - Input proto with physical expression node
* `input_schema` - The Arrow schema for the input, used for determining
  expression data types when performing type coercion.
* `ctx` - Decode context carrying the task context, extension codec, and
  any scoped state needed during recursive deserialization.
* `proto_converter` - Converter hooks used for recursive physical plan and
  expression deserialization.

---

## parse_physical_exprs

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_exprs`

```rust
fn parse_physical_exprs<'a, I>(protos: I, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<std::sync::Arc<dyn PhysicalExpr>>> where I: IntoIterator<Item = &'a protobuf::PhysicalExprNode>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_physical_exprs.md).


---

## parse_physical_sort_expr

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_sort_expr`

```rust
fn parse_physical_sort_expr(proto: &protobuf::PhysicalSortExprNode, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<datafusion_physical_expr::PhysicalSortExpr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_physical_sort_expr.md).


Parses a physical sort expression from a protobuf.

# Arguments

* `proto` - Input proto with physical sort expression node
* `input_schema` - The Arrow schema for the input, used for determining expression data types
  when performing type coercion.
* `ctx` - Decode context carrying the task context, extension codec, and
  any scoped state needed during recursive deserialization.
* `proto_converter` - Converter hooks used for recursive physical plan and
  expression deserialization.

---

## parse_physical_sort_exprs

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_sort_exprs`

```rust
fn parse_physical_sort_exprs(proto: &[protobuf::PhysicalSortExprNode], ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<datafusion_physical_expr::PhysicalSortExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_physical_sort_exprs.md).


Parses a physical sort expressions from a protobuf.

# Arguments

* `proto` - Input proto with vector of physical sort expression node
* `input_schema` - The Arrow schema for the input, used for determining expression data types
  when performing type coercion.
* `ctx` - Decode context carrying the task context, extension codec, and
  any scoped state needed during recursive deserialization.
* `proto_converter` - Converter hooks used for recursive physical plan and
  expression deserialization.

---

## parse_physical_window_expr

`function` · `datafusion_proto::physical_plan::from_proto::parse_physical_window_expr`

```rust
fn parse_physical_window_expr(proto: &protobuf::PhysicalWindowExprNode, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<std::sync::Arc<dyn WindowExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_physical_window_expr.md).


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

---

## parse_protobuf_file_scan_config

`function` · `datafusion_proto::physical_plan::from_proto::parse_protobuf_file_scan_config`

```rust
fn parse_protobuf_file_scan_config(proto: &protobuf::FileScanExecConf, ctx: &super::PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension, file_source: std::sync::Arc<dyn FileSource>) -> datafusion_common::Result<datafusion_datasource::file_scan_config::FileScanConfig>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_protobuf_file_scan_config.md).


---

## parse_protobuf_file_scan_schema

`function` · `datafusion_proto::physical_plan::from_proto::parse_protobuf_file_scan_schema`

> **Deprecated** — since 55.0.0: unused by DataFusion; use `FileScanConfig::parse_table_schema_from_proto` to reconstruct the full table schema

```rust
fn parse_protobuf_file_scan_schema(proto: &protobuf::FileScanExecConf) -> datafusion_common::Result<std::sync::Arc<arrow::datatypes::Schema>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_protobuf_file_scan_schema.md).


---

## parse_protobuf_hash_partitioning

`function` · `datafusion_proto::physical_plan::from_proto::parse_protobuf_hash_partitioning`

```rust
fn parse_protobuf_hash_partitioning(partitioning: Option<&protobuf::PhysicalHashRepartition>, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Option<datafusion_physical_plan::Partitioning>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_protobuf_hash_partitioning.md).


---

## parse_protobuf_partitioning

`function` · `datafusion_proto::physical_plan::from_proto::parse_protobuf_partitioning`

```rust
fn parse_protobuf_partitioning(partitioning: Option<&protobuf::Partitioning>, ctx: &super::PhysicalPlanDecodeContext<'_>, input_schema: &arrow::datatypes::Schema, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Option<datafusion_physical_plan::Partitioning>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_protobuf_partitioning.md).


---

## parse_record_batches

`function` · `datafusion_proto::physical_plan::from_proto::parse_record_batches`

> **Deprecated** — since 55.0.0: unused by DataFusion; `MemorySourceConfig` deserializes its record batches itself via `MemorySourceConfig::try_from_proto`

```rust
fn parse_record_batches(buf: &[u8]) -> datafusion_common::Result<Vec<arrow::array::RecordBatch>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_record_batches.md).


---

## parse_table_schema_from_proto

`function` · `datafusion_proto::physical_plan::from_proto::parse_table_schema_from_proto`

```rust
fn parse_table_schema_from_proto(proto: &protobuf::FileScanExecConf) -> datafusion_common::Result<datafusion_datasource::TableSchema>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.from_proto.parse_table_schema_from_proto.md).


Parses a TableSchema from protobuf, extracting the file schema and partition columns

---
