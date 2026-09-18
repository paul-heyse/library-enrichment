# `datafusion_proto_models::generated::datafusion::parquet_options`

Crate `datafusion-proto-models` · 13 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.parquet_options.json`](../model/datafusion_proto_models.generated.datafusion.parquet_options.json)

## BloomFilterFppOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::BloomFilterFppOpt`

```rust
enum BloomFilterFppOpt
```

**Variants**: `BloomFilterFpp`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<BloomFilterFppOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.BloomFilterFppOpt.md).


---

## BloomFilterNdvOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::BloomFilterNdvOpt`

```rust
enum BloomFilterNdvOpt
```

**Variants**: `BloomFilterNdv`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<BloomFilterNdvOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.BloomFilterNdvOpt.md).


---

## CoerceInt96Opt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96Opt`

```rust
enum CoerceInt96Opt
```

**Variants**: `CoerceInt96`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<CoerceInt96Opt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.CoerceInt96Opt.md).


---

## CoerceInt96TzOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::CoerceInt96TzOpt`

```rust
enum CoerceInt96TzOpt
```

**Variants**: `CoerceInt96Tz`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<CoerceInt96TzOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.CoerceInt96TzOpt.md).


Optional timezone applied to INT96-coerced timestamps when `coerce_int96`
is set. When `Some`, INT96 columns coerce to
`Timestamp(<coerce_int96>, Some(<tz>))` instead of the default
`Timestamp(<coerce_int96>, None)`. No effect when `coerce_int96` is unset.

---

## ColumnIndexTruncateLengthOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::ColumnIndexTruncateLengthOpt`

```rust
enum ColumnIndexTruncateLengthOpt
```

**Variants**: `ColumnIndexTruncateLength`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<ColumnIndexTruncateLengthOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.ColumnIndexTruncateLengthOpt.md).


---

## CompressionOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::CompressionOpt`

```rust
enum CompressionOpt
```

**Variants**: `Compression`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<CompressionOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.CompressionOpt.md).


---

## DictionaryEnabledOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::DictionaryEnabledOpt`

```rust
enum DictionaryEnabledOpt
```

**Variants**: `DictionaryEnabled`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<DictionaryEnabledOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.DictionaryEnabledOpt.md).


---

## EncodingOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::EncodingOpt`

```rust
enum EncodingOpt
```

**Variants**: `Encoding`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<EncodingOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.EncodingOpt.md).


---

## MaxPredicateCacheSizeOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::MaxPredicateCacheSizeOpt`

```rust
enum MaxPredicateCacheSizeOpt
```

**Variants**: `MaxPredicateCacheSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<MaxPredicateCacheSizeOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.MaxPredicateCacheSizeOpt.md).


---

## MaxRowGroupBytesOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::MaxRowGroupBytesOpt`

```rust
enum MaxRowGroupBytesOpt
```

**Variants**: `MaxRowGroupBytes`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<MaxRowGroupBytesOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.MaxRowGroupBytesOpt.md).


---

## MetadataSizeHintOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::MetadataSizeHintOpt`

```rust
enum MetadataSizeHintOpt
```

**Variants**: `MetadataSizeHint`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<MetadataSizeHintOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.MetadataSizeHintOpt.md).


---

## StatisticsEnabledOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::StatisticsEnabledOpt`

```rust
enum StatisticsEnabledOpt
```

**Variants**: `StatisticsEnabled`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<StatisticsEnabledOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.StatisticsEnabledOpt.md).


---

## StatisticsTruncateLengthOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_options::StatisticsTruncateLengthOpt`

```rust
enum StatisticsTruncateLengthOpt
```

**Variants**: `StatisticsTruncateLength`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<StatisticsTruncateLengthOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.parquet_options.StatisticsTruncateLengthOpt.md).


---
