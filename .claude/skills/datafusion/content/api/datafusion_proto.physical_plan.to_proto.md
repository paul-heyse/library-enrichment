# `datafusion_proto::physical_plan::to_proto`

Crate `datafusion-proto` · 11 public items · structured records in [`model/datafusion_proto.physical_plan.to_proto.json`](../model/datafusion_proto.physical_plan.to_proto.json)

## serialize_file_scan_config

`function` · `datafusion_proto::physical_plan::to_proto::serialize_file_scan_config`

```rust
fn serialize_file_scan_config(conf: &datafusion_datasource::file_scan_config::FileScanConfig, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::FileScanExecConf>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_file_scan_config.md).


---

## serialize_maybe_filter

`function` · `datafusion_proto::physical_plan::to_proto::serialize_maybe_filter`

```rust
fn serialize_maybe_filter(expr: Option<std::sync::Arc<dyn PhysicalExpr>>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::MaybeFilter>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_maybe_filter.md).


---

## serialize_partitioning

`function` · `datafusion_proto::physical_plan::to_proto::serialize_partitioning`

```rust
fn serialize_partitioning(partitioning: &datafusion_physical_plan::Partitioning, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::Partitioning>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_partitioning.md).


---

## serialize_physical_aggr_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_aggr_expr`

```rust
fn serialize_physical_aggr_expr(aggr_expr: std::sync::Arc<datafusion_physical_plan::udaf::AggregateFunctionExpr>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalExprNode>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_physical_aggr_expr.md).


---

## serialize_physical_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_expr`

```rust
fn serialize_physical_expr(value: &std::sync::Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<protobuf::PhysicalExprNode>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_physical_expr.md).


Serialize a `PhysicalExpr` to default protobuf representation.

If required, a [`PhysicalExtensionCodec`] can be provided which can handle
serialization of udfs requiring specialized serialization (see [`PhysicalExtensionCodec::try_encode_udf`])

---

## serialize_physical_expr_with_converter

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_expr_with_converter`

```rust
fn serialize_physical_expr_with_converter(value: &std::sync::Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalExprNode>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_physical_expr_with_converter.md).


Serialize a `PhysicalExpr` to default protobuf representation.

If required, a [`PhysicalExtensionCodec`] can be provided which can handle
serialization of udfs requiring specialized serialization (see [`PhysicalExtensionCodec::try_encode_udf`]).
A [`PhysicalProtoConverterExtension`] can be provided to handle the
conversion process (see [`PhysicalProtoConverterExtension::physical_expr_to_proto`]).

---

## serialize_physical_exprs

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_exprs`

```rust
fn serialize_physical_exprs<'a, I>(values: I, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<protobuf::PhysicalExprNode>> where I: IntoIterator<Item = &'a std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_physical_exprs.md).


---

## serialize_physical_sort_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_sort_expr`

```rust
fn serialize_physical_sort_expr(sort_expr: datafusion_physical_expr_common::sort_expr::PhysicalSortExpr, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalSortExprNode>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_physical_sort_expr.md).


---

## serialize_physical_sort_exprs

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_sort_exprs`

```rust
fn serialize_physical_sort_exprs<I>(sort_exprs: I, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<Vec<protobuf::PhysicalSortExprNode>> where I: IntoIterator<Item = datafusion_physical_expr_common::sort_expr::PhysicalSortExpr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_physical_sort_exprs.md).


---

## serialize_physical_window_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_window_expr`

```rust
fn serialize_physical_window_expr(window_expr: &std::sync::Arc<dyn WindowExpr>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalWindowExprNode>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_physical_window_expr.md).


---

## serialize_record_batches

`function` · `datafusion_proto::physical_plan::to_proto::serialize_record_batches`

> **Deprecated** — since 55.0.0: unused by DataFusion; `MemorySourceConfig` serializes its record batches itself via `DataSource::try_to_proto`

```rust
fn serialize_record_batches(batches: &[arrow::array::RecordBatch]) -> datafusion_common::Result<Vec<u8>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.to_proto.serialize_record_batches.md).


---
