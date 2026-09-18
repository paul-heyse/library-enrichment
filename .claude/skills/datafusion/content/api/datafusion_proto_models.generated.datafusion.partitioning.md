# `datafusion_proto_models::generated::datafusion::partitioning`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.partitioning.json`](../model/datafusion_proto_models.generated.datafusion.partitioning.json)

## PartitionMethod

`enum` · `datafusion_proto_models::generated::datafusion::partitioning::PartitionMethod`

```rust
enum PartitionMethod
```

**Variants**: `RoundRobin`, `Hash`, `Unknown`, `Range`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<PartitionMethod>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.partitioning.PartitionMethod.md).


---
