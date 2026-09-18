# `datafusion_proto_models::generated::datafusion::generate_series_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.generate_series_node.json`](../model/datafusion_proto_models.generated.datafusion.generate_series_node.json)

## Args

`enum` · `datafusion_proto_models::generated::datafusion::generate_series_node::Args`

```rust
enum Args
```

**Variants**: `ContainsNull`, `Int64Args`, `TimestampArgs`, `DateArgs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<Args>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.generate_series_node.Args.md).


---
