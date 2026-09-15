# `datafusion_proto::logical_plan::from_proto`

Crate `datafusion-proto` · 6 public items · structured records in [`model/datafusion_proto.logical_plan.from_proto.json`](../model/datafusion_proto.logical_plan.from_proto.json)

## from_proto_binary_op

`function` · `datafusion_proto::logical_plan::from_proto::from_proto_binary_op`

```rust
fn from_proto_binary_op(op: &str) -> datafusion_common::Result<datafusion_expr::Operator, datafusion_proto_common::FromProtoError>
```

---

## parse_expr

`function` · `datafusion_proto::logical_plan::from_proto::parse_expr`

```rust
fn parse_expr(proto: &protobuf::LogicalExprNode, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::Expr, datafusion_proto_common::FromProtoError>
```

---

## parse_exprs

`function` · `datafusion_proto::logical_plan::from_proto::parse_exprs`

```rust
fn parse_exprs<'a, I>(protos: I, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<Vec<datafusion_expr::Expr>, datafusion_proto_common::FromProtoError> where I: IntoIterator<Item = &'a protobuf::LogicalExprNode>
```

Parse a vector of `protobuf::LogicalExprNode`s.

---

## parse_sort

`function` · `datafusion_proto::logical_plan::from_proto::parse_sort`

```rust
fn parse_sort(sort: &protobuf::SortExprNode, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::expr::Sort, datafusion_proto_common::FromProtoError>
```

---

## parse_sorts

`function` · `datafusion_proto::logical_plan::from_proto::parse_sorts`

```rust
fn parse_sorts<'a, I>(protos: I, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<Vec<datafusion_expr::expr::Sort>, datafusion_proto_common::FromProtoError> where I: IntoIterator<Item = &'a protobuf::SortExprNode>
```

---

## parse_write_op

`function` · `datafusion_proto::logical_plan::from_proto::parse_write_op`

```rust
fn parse_write_op(node: &protobuf::DmlNode, ctx: &datafusion_execution::TaskContext, codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::WriteOp, datafusion_proto_common::FromProtoError>
```

Reconstruct a [`WriteOp`] from a [`protobuf::DmlNode`], reading the
`merge_into` payload when the type tag is `MergeInto`.

---
