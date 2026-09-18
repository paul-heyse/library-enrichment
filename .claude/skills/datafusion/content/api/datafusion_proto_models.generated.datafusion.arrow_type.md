# `datafusion_proto_models::generated::datafusion::arrow_type`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.arrow_type.json`](../model/datafusion_proto_models.generated.datafusion.arrow_type.json)

## ArrowTypeEnum

`enum` · `datafusion_proto_models::generated::datafusion::arrow_type::ArrowTypeEnum`

```rust
enum ArrowTypeEnum
```

**Variants**: `None`, `Bool`, `Uint8`, `Int8`, `Uint16`, `Int16`, `Uint32`, `Int32`, `Uint64`, `Int64`, `Float16`, `Float32`, `Float64`, `Utf8`, `Utf8View`, `LargeUtf8`, `Binary`, `BinaryView`, `FixedSizeBinary`, `LargeBinary`, `Date32`, `Date64`, `Duration`, `Timestamp`, `Time32`, `Time64`, `Interval`, `Decimal32`, `Decimal64`, `Decimal128`, `Decimal256`, `List`, `LargeList`, `FixedSizeList`, `ListView`, `LargeListView`, `Struct`, `Union`, `Dictionary`, `Map`, `RunEndEncoded`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<ArrowTypeEnum>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.arrow_type.ArrowTypeEnum.md).


---
