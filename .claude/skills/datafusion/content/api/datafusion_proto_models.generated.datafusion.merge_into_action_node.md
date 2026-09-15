# `datafusion_proto_models::generated::datafusion::merge_into_action_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.merge_into_action_node.json`](../model/datafusion_proto_models.generated.datafusion.merge_into_action_node.json)

## Action

`enum` · `datafusion_proto_models::generated::datafusion::merge_into_action_node::Action`

```rust
enum Action
```

**Variants**: `Update`, `Insert`, `Delete`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<Action>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---
