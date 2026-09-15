# `datafusion_proto_models::generated::datafusion::table_reference`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.table_reference.json`](../model/datafusion_proto_models.generated.datafusion.table_reference.json)

## TableReferenceEnum

`enum` · `datafusion_proto_models::generated::datafusion::table_reference::TableReferenceEnum`

```rust
enum TableReferenceEnum
```

**Variants**: `Bare`, `Partial`, `Full`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<TableReferenceEnum>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---
