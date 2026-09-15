# `datafusion_proto_models::generated::datafusion::window_agg_exec_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.window_agg_exec_node.json`](../model/datafusion_proto_models.generated.datafusion.window_agg_exec_node.json)

## InputOrderMode

`enum` · `datafusion_proto_models::generated::datafusion::window_agg_exec_node::InputOrderMode`

```rust
enum InputOrderMode
```

**Variants**: `Linear`, `PartiallySorted`, `Sorted`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<InputOrderMode>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Set optional to `None` for `BoundedWindowAggExec`.

---
