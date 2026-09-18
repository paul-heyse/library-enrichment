# `arrow_ipc::gen::Message::MessageHeader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.MessageHeader.json).

<a id="op-975380024d5aeee3afb2788b"></a>
## MessageHeader

`struct` · `arrow_ipc::gen::Message::MessageHeader` · arrow-ipc 59.3.0

```rust
struct MessageHeader
```

Source: `src/gen/Message.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

----------------------------------------------------------------------
The root Message type
This union enables us to easily send different message types without
redundant storage, and in the future we can easily add new message types.

Arrow implementations do not need to implement all of the message types,
which may include experimental metadata types. For maximum compatibility,
it is best to send data using RecordBatch

<a id="op-0adbef421900b6ffc0f1d432"></a>
## 0

`struct_field` · `arrow_ipc::gen::Message::MessageHeader::0` · arrow-ipc 59.3.0

```rust
0: u8
```

Source: `src/gen/Message.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e26cf1a3004d0e5bbbbf78d0"></a>
## DictionaryBatch

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::DictionaryBatch` · arrow-ipc 59.3.0

```rust
DictionaryBatch
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bfdaa4cd33a6c1e4825a41a"></a>
## ENUM_MAX

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::ENUM_MAX` · arrow-ipc 59.3.0

```rust
ENUM_MAX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b68b191a0d02bde403257cd9"></a>
## ENUM_MIN

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::ENUM_MIN` · arrow-ipc 59.3.0

```rust
ENUM_MIN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:266`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-424985f29b5ac04886f9ef57"></a>
## ENUM_VALUES

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::ENUM_VALUES` · arrow-ipc 59.3.0

```rust
ENUM_VALUES
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-056d9e8fefb5b1d60b81fba8"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Message::MessageHeader::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [305, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-033aa037c42e887e7eaa5169"></a>
## NONE

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::NONE` · arrow-ipc 59.3.0

```rust
NONE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:259`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93cb8a14c090ec54c45cff11"></a>
## Output

`assoc_type` · `arrow_ipc::gen::Message::MessageHeader::Output` · arrow-ipc 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 1], "end": [315, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Message.rs:308`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8aa2813013565cc6b37d2683"></a>
## RecordBatch

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::RecordBatch` · arrow-ipc 59.3.0

```rust
RecordBatch
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:262`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3af30a940066b1ee0c4f9bf"></a>
## Scalar

`assoc_type` · `arrow_ipc::gen::Message::MessageHeader::Scalar` · arrow-ipc 59.3.0

```rust
Scalar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [317, 1], "end": [329, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Message.rs:318`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52e04a200beaf3085bb0f6cf"></a>
## Schema

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::Schema` · arrow-ipc 59.3.0

```rust
Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-272869b1f0427bcdea251ae6"></a>
## SparseTensor

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::SparseTensor` · arrow-ipc 59.3.0

```rust
SparseTensor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:264`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-763c088175f6ead36874c096"></a>
## Tensor

`assoc_const` · `arrow_ipc::gen::Message::MessageHeader::Tensor` · arrow-ipc 59.3.0

```rust
Tensor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc4b7415c62184ef4249bdc2"></a>
## clone

`function` · `arrow_ipc::gen::Message::MessageHeader::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> MessageHeader
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 10], "end": [254, 15], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Message.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11fca118f575afdea3306067"></a>
## cmp

`function` · `arrow_ipc::gen::Message::MessageHeader::cmp` · arrow-ipc 59.3.0

```rust
fn cmp(&self, other: &MessageHeader) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 50], "end": [254, 53], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/gen/Message.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-225f6f281e99103725d7609c"></a>
## default

`function` · `arrow_ipc::gen::Message::MessageHeader::default` · arrow-ipc 59.3.0

```rust
fn default() -> MessageHeader
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 61], "end": [254, 68], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Message.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4420524bee3a203562992dd"></a>
## eq

`function` · `arrow_ipc::gen::Message::MessageHeader::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &MessageHeader) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 23], "end": [254, 32], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Message.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc676c7ce3a1c9a62507ba40"></a>
## fmt

`function` · `arrow_ipc::gen::Message::MessageHeader::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [297, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Message.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ca3468d3e7e0f97d101f4e"></a>
## follow

`function` · `arrow_ipc::gen::Message::MessageHeader::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [305, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fff77bbf123626e8146e0d78"></a>
## from_little_endian

`function` · `arrow_ipc::gen::Message::MessageHeader::from_little_endian` · arrow-ipc 59.3.0

```rust
fn from_little_endian(v: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [317, 1], "end": [329, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Message.rs:325`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dd96c4bb69ebe870b91bf30"></a>
## hash

`function` · `arrow_ipc::gen::Message::MessageHeader::hash` · arrow-ipc 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 55], "end": [254, 59], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/gen/Message.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d69315d726cb4e4f2a6aff8a"></a>
## partial_cmp

`function` · `arrow_ipc::gen::Message::MessageHeader::partial_cmp` · arrow-ipc 59.3.0

```rust
fn partial_cmp(&self, other: &MessageHeader) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 38], "end": [254, 48], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/gen/Message.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80e412088ae735bc87f82b16"></a>
## push

`function` · `arrow_ipc::gen::Message::MessageHeader::push` · arrow-ipc 59.3.0

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 1], "end": [315, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Message.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8c3e42323df358e54cc6d06"></a>
## run_verifier

`function` · `arrow_ipc::gen::Message::MessageHeader::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 1], "end": [340, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Message.rs:333`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5a4cf769a4c17e5df4e4050"></a>
## to_little_endian

`function` · `arrow_ipc::gen::Message::MessageHeader::to_little_endian` · arrow-ipc 59.3.0

```rust
fn to_little_endian(self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [317, 1], "end": [329, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::endian_scalar::EndianScalar", "path": "EndianScalar"}, "trait_path": "flatbuffers::endian_scalar::EndianScalar"}`

Source: `src/gen/Message.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14cdcb1a4d643c23669c4b2d"></a>
## variant_name

`function` · `arrow_ipc::gen::Message::MessageHeader::variant_name` · arrow-ipc 59.3.0

```rust
fn variant_name(self) -> Option<&'static str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::MessageHeader", "path": "MessageHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [288, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Returns the variant's name or "" if unknown.
