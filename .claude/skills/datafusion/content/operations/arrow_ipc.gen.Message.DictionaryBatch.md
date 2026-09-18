# `arrow_ipc::gen::Message::DictionaryBatch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.DictionaryBatch.json).

<a id="op-00bc69c5b8a8dfa1fc137e7e"></a>
## DictionaryBatch

`struct` · `arrow_ipc::gen::Message::DictionaryBatch` · arrow-ipc 59.3.0

```rust
struct DictionaryBatch<'a>
```

Source: `src/gen/Message.rs:905`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

For sending dictionary encoding information. Any Field can be
dictionary-encoded, but in this case none of its children may be
dictionary-encoded.
There is one vector / column per dictionary, but that vector / column
may be spread across multiple dictionary batches by using the isDelta
flag

<a id="op-168d8902740153a95da21406"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Message::DictionaryBatch::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [909, 1], "end": [917, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:910`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9a6287fef920d1d101b48b1"></a>
## VT_DATA

`assoc_const` · `arrow_ipc::gen::Message::DictionaryBatch::VT_DATA` · arrow-ipc 59.3.0

```rust
VT_DATA
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:921`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e41c8d8fcc25a87d6fb04ce"></a>
## VT_ID

`assoc_const` · `arrow_ipc::gen::Message::DictionaryBatch::VT_ID` · arrow-ipc 59.3.0

```rust
VT_ID
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:920`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f555e5cce73ade90c15bd4f4"></a>
## VT_ISDELTA

`assoc_const` · `arrow_ipc::gen::Message::DictionaryBatch::VT_ISDELTA` · arrow-ipc 59.3.0

```rust
VT_ISDELTA
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:922`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9b13e4705d123c58f220177"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Message::DictionaryBatch::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Message.rs:906`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72635168a4120abcff51512b"></a>
## clone

`function` · `arrow_ipc::gen::Message::DictionaryBatch::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> DictionaryBatch<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 16], "end": [897, 21], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Message.rs:897`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43d294ab40a751e07cc0a8d5"></a>
## create

`function` · `arrow_ipc::gen::Message::DictionaryBatch::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args DictionaryBatchArgs<'args>) -> flatbuffers::WIPOffset<DictionaryBatch<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:929`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cf9025715379b31bd473261"></a>
## data

`function` · `arrow_ipc::gen::Message::DictionaryBatch::data` · arrow-ipc 59.3.0

```rust
fn data(&self) -> Option<RecordBatch<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:954`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dc529780fcbf70ac5c15221"></a>
## eq

`function` · `arrow_ipc::gen::Message::DictionaryBatch::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &DictionaryBatch<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 23], "end": [897, 32], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Message.rs:897`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9095fce784ef44be33100b3c"></a>
## fmt

`function` · `arrow_ipc::gen::Message::DictionaryBatch::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1049, 1], "end": [1057, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Message.rs:1050`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66d6b475b539e9a9653e4c6a"></a>
## follow

`function` · `arrow_ipc::gen::Message::DictionaryBatch::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [909, 1], "end": [917, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:912`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af822e229c7c61a1bba36013"></a>
## id

`function` · `arrow_ipc::gen::Message::DictionaryBatch::id` · arrow-ipc 59.3.0

```rust
fn id(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:943`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec6cdbec40299ea86068a803"></a>
## init_from_table

`function` · `arrow_ipc::gen::Message::DictionaryBatch::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:925`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f3d088dd57804e787675eb5"></a>
## isDelta

`function` · `arrow_ipc::gen::Message::DictionaryBatch::isDelta` · arrow-ipc 59.3.0

```rust
fn isDelta(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [919, 1], "end": [977, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:967`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

If isDelta is true the values in the dictionary are to be appended to a
dictionary with the indicated id. If isDelta is false this dictionary
should replace the existing dictionary.

<a id="op-861ed2e25a0f951ede0fff8a"></a>
## run_verifier

`function` · `arrow_ipc::gen::Message::DictionaryBatch::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Message::DictionaryBatch", "path": "DictionaryBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 1], "end": [993, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Message.rs:981`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
