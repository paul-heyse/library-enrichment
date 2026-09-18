# `arrow_ipc::gen::Message::BodyCompression`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.BodyCompression.json).

<a id="op-950bc94652bb2fee1e3763ed"></a>
## BodyCompression

`struct` · `arrow_ipc::gen::Message::BodyCompression` · arrow-ipc 59.3.0

```rust
struct BodyCompression<'a>
```

Source: `src/gen/Message.rs:495`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Optional compression for the memory buffers constituting IPC message
bodies. Intended for use with RecordBatch but could be used for other
message types

<a id="op-028cc8e9e791d9a20e0c34ed"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Message::BodyCompression::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [507, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:500`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4046edb661e87e6eed73f9d"></a>
## VT_CODEC

`assoc_const` · `arrow_ipc::gen::Message::BodyCompression::VT_CODEC` · arrow-ipc 59.3.0

```rust
VT_CODEC
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [556, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-538ef31d9e2572d3d33e5d10"></a>
## VT_METHOD

`assoc_const` · `arrow_ipc::gen::Message::BodyCompression::VT_METHOD` · arrow-ipc 59.3.0

```rust
VT_METHOD
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [556, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:511`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07a61f1d62aea6a75a165d52"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Message::BodyCompression::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Message.rs:496`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ddcd8014738dd81277b1870"></a>
## clone

`function` · `arrow_ipc::gen::Message::BodyCompression::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> BodyCompression<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 16], "end": [490, 21], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Message.rs:490`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86043c584ea13f2d150ab890"></a>
## codec

`function` · `arrow_ipc::gen::Message::BodyCompression::codec` · arrow-ipc 59.3.0

```rust
fn codec(&self) -> CompressionType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [556, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:531`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Compressor library.
For LZ4_FRAME, each compressed buffer must consist of a single frame.

<a id="op-0946387fc637c7556679ae9a"></a>
## create

`function` · `arrow_ipc::gen::Message::BodyCompression::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args BodyCompressionArgs) -> flatbuffers::WIPOffset<BodyCompression<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [556, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:518`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5f96c8870f03a959066f4e1"></a>
## eq

`function` · `arrow_ipc::gen::Message::BodyCompression::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &BodyCompression<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 23], "end": [490, 32], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Message.rs:490`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d7bf47d9d87b14d6c7ea739"></a>
## fmt

`function` · `arrow_ipc::gen::Message::BodyCompression::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [624, 1], "end": [631, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Message.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cdf5d33e5957a93cda667b1"></a>
## follow

`function` · `arrow_ipc::gen::Message::BodyCompression::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [507, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:502`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12c7656aa9b32e10b433aeec"></a>
## init_from_table

`function` · `arrow_ipc::gen::Message::BodyCompression::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [556, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:514`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18952ea450dc66a8c72c5484"></a>
## method

`function` · `arrow_ipc::gen::Message::BodyCompression::method` · arrow-ipc 59.3.0

```rust
fn method(&self) -> BodyCompressionMethod
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [556, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:543`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Indicates the way the record batch body was compressed

<a id="op-c20f0deb90a0b3d77a1f568b"></a>
## run_verifier

`function` · `arrow_ipc::gen::Message::BodyCompression::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::BodyCompression", "path": "BodyCompression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [558, 1], "end": [571, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Message.rs:560`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
