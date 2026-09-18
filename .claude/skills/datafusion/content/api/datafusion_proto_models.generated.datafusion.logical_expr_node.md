# `datafusion_proto_models::generated::datafusion::logical_expr_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.logical_expr_node.json`](../model/datafusion_proto_models.generated.datafusion.logical_expr_node.json)

## ExprType

`enum` · `datafusion_proto_models::generated::datafusion::logical_expr_node::ExprType`

```rust
enum ExprType
```

**Variants**: `Column`, `Alias`, `Literal`, `BinaryExpr`, `IsNullExpr`, `IsNotNullExpr`, `NotExpr`, `Between`, `Case`, `Cast`, `Negative`, `InList`, `Wildcard`, `TryCast`, `WindowExpr`, `AggregateUdfExpr`, `ScalarUdfExpr`, `GroupingSet`, `Cube`, `Rollup`, `IsTrue`, `IsFalse`, `IsUnknown`, `IsNotTrue`, `IsNotFalse`, `IsNotUnknown`, `Like`, `Ilike`, `SimilarTo`, `Placeholder`, `Unnest`, `ScalarSubqueryExpr`, `HigherOrderUdfExpr`, `Lambda`, `LambdaVariable`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<ExprType>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.logical_expr_node.ExprType.md).


---
