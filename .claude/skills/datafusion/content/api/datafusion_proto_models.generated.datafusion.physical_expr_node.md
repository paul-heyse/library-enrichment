# `datafusion_proto_models::generated::datafusion::physical_expr_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.physical_expr_node.json`](../model/datafusion_proto_models.generated.datafusion.physical_expr_node.json)

## ExprType

`enum` · `datafusion_proto_models::generated::datafusion::physical_expr_node::ExprType`

```rust
enum ExprType
```

**Variants**: `Column`, `Literal`, `BinaryExpr`, `AggregateExpr`, `IsNullExpr`, `IsNotNullExpr`, `NotExpr`, `Case`, `Cast`, `Sort`, `Negative`, `InList`, `TryCast`, `WindowExpr`, `ScalarUdf`, `LikeExpr`, `Extension`, `UnknownColumn`, `HashExpr`, `ScalarSubquery`, `DynamicFilter`, `HigherOrderUdf`, `Lambda`, `LambdaVariable`, `RangeExpr`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<ExprType>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.physical_expr_node.ExprType.md).


---
