# `datafusion_proto_models::generated::datafusion::window_frame`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.window_frame.json`](../model/datafusion_proto_models.generated.datafusion.window_frame.json)

## EndBound

`enum` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound`

```rust
enum EndBound
```

**Variants**: `Bound`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<EndBound>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

"optional" keyword is stable in protoc 3.15 but prost is still on 3.14 (see <https://github.com/tokio-rs/prost/issues/430> and <https://github.com/tokio-rs/prost/pull/455>)
this syntax is ugly but is binary compatible with the "optional" keyword (see <https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3>)

---
