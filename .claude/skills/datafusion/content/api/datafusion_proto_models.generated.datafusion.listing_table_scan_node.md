# `datafusion_proto_models::generated::datafusion::listing_table_scan_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.listing_table_scan_node.json`](../model/datafusion_proto_models.generated.datafusion.listing_table_scan_node.json)

## FileFormatType

`enum` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType`

```rust
enum FileFormatType
```

**Variants**: `Csv`, `Parquet`, `Avro`, `Json`, `Arrow`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<FileFormatType>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---
