# `datafusion_proto_common::generated::datafusion_proto_common::constraint`

Crate `datafusion-proto-common` · 1 public items · structured records in [`model/datafusion_proto_common.generated.datafusion_proto_common.constraint.json`](../model/datafusion_proto_common.generated.datafusion_proto_common.constraint.json)

## ConstraintMode

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::constraint::ConstraintMode`

```rust
enum ConstraintMode
```

**Variants**: `PrimaryKey`, `Unique`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<ConstraintMode>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---
