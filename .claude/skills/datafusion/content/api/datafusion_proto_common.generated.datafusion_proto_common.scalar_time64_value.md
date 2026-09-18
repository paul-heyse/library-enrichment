# `datafusion_proto_common::generated::datafusion_proto_common::scalar_time64_value`

Crate `datafusion-proto-common` · 1 public items · structured records in [`model/datafusion_proto_common.generated.datafusion_proto_common.scalar_time64_value.json`](../model/datafusion_proto_common.generated.datafusion_proto_common.scalar_time64_value.json)

## Value

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::scalar_time64_value::Value`

```rust
enum Value
```

**Variants**: `Time64MicrosecondValue`, `Time64NanosecondValue`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<Value>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_common.generated.datafusion_proto_common.scalar_time64_value.Value.md).


---
