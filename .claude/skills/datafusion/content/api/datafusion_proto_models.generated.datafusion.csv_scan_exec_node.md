# `datafusion_proto_models::generated::datafusion::csv_scan_exec_node`

Crate `datafusion-proto-models` · 2 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.csv_scan_exec_node.json`](../model/datafusion_proto_models.generated.datafusion.csv_scan_exec_node.json)

## OptionalComment

`enum` · `datafusion_proto_models::generated::datafusion::csv_scan_exec_node::OptionalComment`

```rust
enum OptionalComment
```

**Variants**: `Comment`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<OptionalComment>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---

## OptionalEscape

`enum` · `datafusion_proto_models::generated::datafusion::csv_scan_exec_node::OptionalEscape`

```rust
enum OptionalEscape
```

**Variants**: `Escape`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
fn encoded_len(&self) -> usize
fn merge(field: &mut ::core::option::Option<OptionalEscape>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

---
