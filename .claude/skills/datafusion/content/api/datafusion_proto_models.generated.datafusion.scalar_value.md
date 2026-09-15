# `datafusion_proto_models::generated::datafusion::scalar_value`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.scalar_value.json`](../model/datafusion_proto_models.generated.datafusion.scalar_value.json)

## Value

`enum` · `datafusion_proto_models::generated::datafusion::scalar_value::Value`

```rust
enum Value
```

**Variants**: `NullValue`, `BoolValue`, `Utf8Value`, `LargeUtf8Value`, `Utf8ViewValue`, `Int8Value`, `Int16Value`, `Int32Value`, `Int64Value`, `Uint8Value`, `Uint16Value`, `Uint32Value`, `Uint64Value`, `Float32Value`, `Float64Value`, `Date32Value`, `Time32Value`, `LargeListValue`, `ListValue`, `FixedSizeListValue`, `ListViewValue`, `LargeListViewValue`, `StructValue`, `MapValue`, `Decimal32Value`, `Decimal64Value`, `Decimal128Value`, `Decimal256Value`, `Date64Value`, `IntervalYearmonthValue`, `DurationSecondValue`, `DurationMillisecondValue`, `DurationMicrosecondValue`, `DurationNanosecondValue`, `TimestampValue`, `DictionaryValue`, `BinaryValue`, `LargeBinaryValue`, `BinaryViewValue`, `Time64Value`, `IntervalDaytimeValue`, `IntervalMonthDayNano`, `FixedSizeBinaryValue`, `UnionValue`, `RunEndEncodedValue`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<Value>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---
