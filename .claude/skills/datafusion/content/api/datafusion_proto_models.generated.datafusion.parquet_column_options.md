# `datafusion_proto_models::generated::datafusion::parquet_column_options`

Crate `datafusion-proto-models` · 7 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.parquet_column_options.json`](../model/datafusion_proto_models.generated.datafusion.parquet_column_options.json)

## BloomFilterEnabledOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_column_options::BloomFilterEnabledOpt`

```rust
enum BloomFilterEnabledOpt
```

**Variants**: `BloomFilterEnabled`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<BloomFilterEnabledOpt>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---

## BloomFilterFppOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_column_options::BloomFilterFppOpt`

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

---

## BloomFilterNdvOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_column_options::BloomFilterNdvOpt`

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

---

## CompressionOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_column_options::CompressionOpt`

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

---

## DictionaryEnabledOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_column_options::DictionaryEnabledOpt`

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

---

## EncodingOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_column_options::EncodingOpt`

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

---

## StatisticsEnabledOpt

`enum` · `datafusion_proto_models::generated::datafusion::parquet_column_options::StatisticsEnabledOpt`

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

---
