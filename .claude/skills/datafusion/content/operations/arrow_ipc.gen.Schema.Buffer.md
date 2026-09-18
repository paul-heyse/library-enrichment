# `arrow_ipc::gen::Schema::Buffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.Buffer.json).

<a id="op-dda8da0cbeb942e3483be66b"></a>
## Buffer

`struct` · `arrow_ipc::gen::Schema::Buffer` · arrow-ipc 59.3.0

```rust
struct Buffer
```

Source: `src/gen/Schema.rs:1138`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

----------------------------------------------------------------------
A Buffer represents a single contiguous memory segment

<a id="op-77634dc16503ff2a9fca288a"></a>
## 0

`struct_field` · `arrow_ipc::gen::Schema::Buffer::0` · arrow-ipc 59.3.0

```rust
0: [u8; 16]
```

Source: `src/gen/Schema.rs:1138`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bb783105f2dd2a66b42289b"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::Buffer::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1154, 1], "end": [1160, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:1155`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49b1bf849e1272cbf97e30cb"></a>
## Output

`assoc_type` · `arrow_ipc::gen::Schema::Buffer::Output` · arrow-ipc 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1168, 1], "end": [1184, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Schema.rs:1169`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d0164423d1821ad6a444c5a"></a>
## alignment

`function` · `arrow_ipc::gen::Schema::Buffer::alignment` · arrow-ipc 59.3.0

```rust
fn alignment() -> flatbuffers::PushAlignment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1168, 1], "end": [1184, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Schema.rs:1181`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-204d7cc065d7b5f833e49cac"></a>
## clone

`function` · `arrow_ipc::gen::Schema::Buffer::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 10], "end": [1137, 15], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:1137`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa938eb737e6694737001a58"></a>
## default

`function` · `arrow_ipc::gen::Schema::Buffer::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1139, 1], "end": [1143, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Schema.rs:1140`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6d113218fb0932eb8f7c329"></a>
## eq

`function` · `arrow_ipc::gen::Schema::Buffer::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &Buffer) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 23], "end": [1137, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:1137`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19a0133319b5c2acdff9b999"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::Buffer::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1144, 1], "end": [1151, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:1145`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b3f6d8e4d680e664b1ecbc0"></a>
## follow

`function` · `arrow_ipc::gen::Schema::Buffer::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1154, 1], "end": [1160, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:1157`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ddce7c8bf8b2526390093f7"></a>
## length

`function` · `arrow_ipc::gen::Schema::Buffer::length` · arrow-ipc 59.3.0

```rust
fn length(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1197, 1], "end": [1270, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1242`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The absolute length (in bytes) of the memory buffer. The memory is found
from offset (inclusive) to offset + length (non-inclusive). When building
messages using the encapsulated IPC message, padding bytes may be written
after a buffer, but such padding bytes do not need to be accounted for in
the size here.

<a id="op-a28299f30ec47d2cffa06c84"></a>
## new

`function` · `arrow_ipc::gen::Schema::Buffer::new` · arrow-ipc 59.3.0

```rust
fn new(offset: i64, length: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1197, 1], "end": [1270, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1199`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28369c1f842461e01cbfac2d"></a>
## offset

`function` · `arrow_ipc::gen::Schema::Buffer::offset` · arrow-ipc 59.3.0

```rust
fn offset(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1197, 1], "end": [1270, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1208`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The relative offset into the shared memory page where the bytes for this
buffer starts

<a id="op-b6ada900247dfdffb6a43302"></a>
## push

`function` · `arrow_ipc::gen::Schema::Buffer::push` · arrow-ipc 59.3.0

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1168, 1], "end": [1184, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Schema.rs:1171`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30fd203f7a0df130b5e854ea"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::Buffer::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1186, 1], "end": [1195, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:1188`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-944316028e350830ef8f3799"></a>
## set_length

`function` · `arrow_ipc::gen::Schema::Buffer::set_length` · arrow-ipc 59.3.0

```rust
fn set_length(&mut self, x: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1197, 1], "end": [1270, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1257`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa0015eaa70d1c62f72f2748"></a>
## set_offset

`function` · `arrow_ipc::gen::Schema::Buffer::set_offset` · arrow-ipc 59.3.0

```rust
fn set_offset(&mut self, x: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Schema::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1197, 1], "end": [1270, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:1223`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
