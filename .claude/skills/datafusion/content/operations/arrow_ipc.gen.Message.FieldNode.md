# `arrow_ipc::gen::Message::FieldNode`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.FieldNode.json).

<a id="op-4e86652370f799e2544c5010"></a>
## FieldNode

`struct` · `arrow_ipc::gen::Message::FieldNode` · arrow-ipc 59.3.0

```rust
struct FieldNode
```

Source: `src/gen/Message.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

----------------------------------------------------------------------
Data structures for describing a table row batch (a collection of
equal-length Arrow arrays)
Metadata about a field at some level of a nested type tree (but not
its children).

For example, a `List<Int16>` with values `[[1, 2, 3], null, [4], [5, 6], null]`
would have {length: 5, null_count: 2} for its List node, and {length: 6,
null_count: 0} for its Int16 node, as separate FieldNode structs

<a id="op-6562e03021da6432decd3b33"></a>
## 0

`struct_field` · `arrow_ipc::gen::Message::FieldNode::0` · arrow-ipc 59.3.0

```rust
0: [u8; 16]
```

Source: `src/gen/Message.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1664fa7aeca4f342c9a735f1"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Message::FieldNode::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [373, 1], "end": [379, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:374`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b349afe51655b42f0ac8f7e"></a>
## Output

`assoc_type` · `arrow_ipc::gen::Message::FieldNode::Output` · arrow-ipc 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [403, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Message.rs:388`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dff84090d48eac14d9b7b4a1"></a>
## alignment

`function` · `arrow_ipc::gen::Message::FieldNode::alignment` · arrow-ipc 59.3.0

```rust
fn alignment() -> flatbuffers::PushAlignment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [403, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Message.rs:400`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aafd6a01df94ef6a1f78ccea"></a>
## clone

`function` · `arrow_ipc::gen::Message::FieldNode::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> FieldNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 10], "end": [356, 15], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Message.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23e769eaf7be60ec35d43e16"></a>
## default

`function` · `arrow_ipc::gen::Message::FieldNode::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [362, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Message.rs:359`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03099180b951a2afcbd605f3"></a>
## eq

`function` · `arrow_ipc::gen::Message::FieldNode::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &FieldNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 23], "end": [356, 32], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Message.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5e5971dde1bbd487c16b6ee"></a>
## fmt

`function` · `arrow_ipc::gen::Message::FieldNode::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [363, 1], "end": [370, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Message.rs:364`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4420c66af032deaa2266ccf2"></a>
## follow

`function` · `arrow_ipc::gen::Message::FieldNode::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [373, 1], "end": [379, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Message.rs:376`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8984b69f4f1f2d0f008f0860"></a>
## length

`function` · `arrow_ipc::gen::Message::FieldNode::length` · arrow-ipc 59.3.0

```rust
fn length(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [487, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:427`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The number of value slots in the Arrow array at this level of a nested
tree

<a id="op-85a57623e64eb58522eddd1d"></a>
## new

`function` · `arrow_ipc::gen::Message::FieldNode::new` · arrow-ipc 59.3.0

```rust
fn new(length: i64, null_count: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [487, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:418`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70d270b8d0bca7344ed1a127"></a>
## null_count

`function` · `arrow_ipc::gen::Message::FieldNode::null_count` · arrow-ipc 59.3.0

```rust
fn null_count(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [487, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The number of observed nulls. Fields with null_count == 0 may choose not
to write their physical validity bitmap out as a materialized buffer,
instead setting the length of the bitmap buffer to 0.

<a id="op-1f7e3bcd7bd78c922c8ef570"></a>
## push

`function` · `arrow_ipc::gen::Message::FieldNode::push` · arrow-ipc 59.3.0

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [403, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::push::Push", "path": "Push"}, "trait_path": "flatbuffers::push::Push"}`

Source: `src/gen/Message.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed39edb6f8bcb26371020714"></a>
## run_verifier

`function` · `arrow_ipc::gen::Message::FieldNode::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [405, 1], "end": [414, 2], "filename": "src/gen/Message.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Message.rs:407`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42e9d170bfb377c7004b8cdd"></a>
## set_length

`function` · `arrow_ipc::gen::Message::FieldNode::set_length` · arrow-ipc 59.3.0

```rust
fn set_length(&mut self, x: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [487, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:442`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3f7dbd56bd32d26de71ed5b"></a>
## set_null_count

`function` · `arrow_ipc::gen::Message::FieldNode::set_null_count` · arrow-ipc 59.3.0

```rust
fn set_null_count(&mut self, x: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::gen::Message::FieldNode", "path": "FieldNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [487, 2], "filename": "src/gen/Message.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Message.rs:474`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
