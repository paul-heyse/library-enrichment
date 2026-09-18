# `arrow_ipc::gen::File::Block`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.Block.json).

<a id="op-221567b7302c20e4e222fdcd"></a>
## Block

`struct` · `arrow_ipc::gen::File::Block` · arrow-ipc 59.3.0

```rust
struct Block
```

Source: `src/gen/File.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c13149eb66cc11a524521dad"></a>
## 0

`struct_field` · `arrow_ipc::gen::File::Block::0` · arrow-ipc 59.3.0

```rust
0: [u8; 24]
```

Source: `src/gen/File.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b04d3def2f33e3560a21f084"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::File::Block::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [54, 2], "filename": "src/gen/File.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/File.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c621d5625aa429ab80700aa6"></a>
## Output

`assoc_type` · `arrow_ipc::gen::File::Block::Output` · arrow-ipc 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [78, 2], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/File.rs:63`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e104a84db30d3688803a82"></a>
## alignment

`function` · `arrow_ipc::gen::File::Block::alignment` · arrow-ipc 59.3.0

```rust
fn alignment() -> flatbuffers::PushAlignment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [78, 2], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/File.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23f86de3d14be9de697b7422"></a>
## bodyLength

`function` · `arrow_ipc::gen::File::Block::bodyLength` · arrow-ipc 59.3.0

```rust
fn bodyLength(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [191, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:163`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Length of the data (this is aligned so there can be a gap between this and
the metadata).

<a id="op-df0c5a9b89f684d10fcd7991"></a>
## clone

`function` · `arrow_ipc::gen::File::Block::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> Block
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/File.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af1aca8c25bafdc8d1644722"></a>
## default

`function` · `arrow_ipc::gen::File::Block::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [36, 2], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/File.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd5e56f59663742dab369d67"></a>
## eq

`function` · `arrow_ipc::gen::File::Block::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &Block) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 23], "end": [30, 32], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/File.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bb1c2c346c3618a79720d0d"></a>
## fmt

`function` · `arrow_ipc::gen::File::Block::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [45, 2], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/File.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82d1eca6fa3acf6b51b11986"></a>
## follow

`function` · `arrow_ipc::gen::File::Block::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [54, 2], "filename": "src/gen/File.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/File.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-484b482f58bffee7a7b3b65b"></a>
## metaDataLength

`function` · `arrow_ipc::gen::File::Block::metaDataLength` · arrow-ipc 59.3.0

```rust
fn metaDataLength(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [191, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:132`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Length of the metadata

<a id="op-8d1019822ef874625bfcafd1"></a>
## new

`function` · `arrow_ipc::gen::File::Block::new` · arrow-ipc 59.3.0

```rust
fn new(offset: i64, metaDataLength: i32, bodyLength: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [191, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48527b8717ae5d120a2d5767"></a>
## offset

`function` · `arrow_ipc::gen::File::Block::offset` · arrow-ipc 59.3.0

```rust
fn offset(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [191, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Index to the start of the RecordBlock (note this is past the Message header)

<a id="op-eff05f6bbaa813fc1aa18e65"></a>
## push

`function` · `arrow_ipc::gen::File::Block::push` · arrow-ipc 59.3.0

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [78, 2], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/File.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3b43d8cd628205295fd7d44"></a>
## run_verifier

`function` · `arrow_ipc::gen::File::Block::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [89, 2], "filename": "src/gen/File.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/File.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e17bc16ee1f8f19223fe25f"></a>
## set_bodyLength

`function` · `arrow_ipc::gen::File::Block::set_bodyLength` · arrow-ipc 59.3.0

```rust
fn set_bodyLength(&mut self, x: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [191, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:178`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-308b1ec551725889f7a90b4f"></a>
## set_metaDataLength

`function` · `arrow_ipc::gen::File::Block::set_metaDataLength` · arrow-ipc 59.3.0

```rust
fn set_metaDataLength(&mut self, x: i32)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [191, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6397595d49eb80e6dfec3d30"></a>
## set_offset

`function` · `arrow_ipc::gen::File::Block::set_offset` · arrow-ipc 59.3.0

```rust
fn set_offset(&mut self, x: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::File::Block", "path": "Block"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [191, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
