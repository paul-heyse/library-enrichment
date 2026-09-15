# `datafusion_proto::logical_plan::to_proto`

Crate `datafusion-proto` · 4 public items · structured records in [`model/datafusion_proto.logical_plan.to_proto.json`](../model/datafusion_proto.logical_plan.to_proto.json)

## serialize_expr

`function` · `datafusion_proto::logical_plan::to_proto::serialize_expr`

```rust
fn serialize_expr(expr: &datafusion_expr::Expr, codec: &dyn LogicalExtensionCodec) -> Result<protobuf::LogicalExprNode, protobuf::ToProtoError>
```

---

## serialize_exprs

`function` · `datafusion_proto::logical_plan::to_proto::serialize_exprs`

```rust
fn serialize_exprs<'a, I>(exprs: I, codec: &dyn LogicalExtensionCodec) -> Result<Vec<protobuf::LogicalExprNode>, protobuf::ToProtoError> where I: IntoIterator<Item = &'a datafusion_expr::Expr>
```

---

## serialize_merge_into_op

`function` · `datafusion_proto::logical_plan::to_proto::serialize_merge_into_op`

```rust
fn serialize_merge_into_op(op: &datafusion_expr::dml::MergeIntoOp, codec: &dyn LogicalExtensionCodec) -> Result<protobuf::MergeIntoOpNode, protobuf::ToProtoError>
```

---

## serialize_sorts

`function` · `datafusion_proto::logical_plan::to_proto::serialize_sorts`

```rust
fn serialize_sorts<'a, I>(sorts: I, codec: &dyn LogicalExtensionCodec) -> Result<Vec<protobuf::SortExprNode>, protobuf::ToProtoError> where I: IntoIterator<Item = &'a datafusion_expr::SortExpr>
```

---
