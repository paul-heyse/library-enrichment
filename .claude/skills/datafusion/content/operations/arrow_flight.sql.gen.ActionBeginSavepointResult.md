# `arrow_flight::sql::gen::ActionBeginSavepointResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionBeginSavepointResult.json).

<a id="op-f5571387aeaa3d1fde722891"></a>
## ActionBeginSavepointResult

`struct` · `arrow_flight::sql::gen::ActionBeginSavepointResult` · arrow-flight 59.3.0

```rust
struct ActionBeginSavepointResult
```

Source: `src/sql/arrow.flight.protocol.sql.rs:538`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The result of a "BeginSavepoint" action.

The transaction can be manipulated with the "EndSavepoint" action.
If the associated transaction is committed, rolled back, or times
out, then the savepoint is also invalidated.

The result should be wrapped in a google.protobuf.Any message.

<a id="op-5f0401d165f76ad17077c7d5"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fe4acc4698b9a335ad1c2f2"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 38], "end": [537, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21cc1f3d4b4c11f111e55f8a"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionBeginSavepointResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 10], "end": [537, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ac99162bb4f321c212ba46"></a>
## default

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 38], "end": [537, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70ff10ce4c90cd96eef96bf9"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 38], "end": [537, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb3268c1f2d53cbc7f8861c2"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionBeginSavepointResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 17], "end": [537, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2320af3eccd2ebb8ba47f994"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 38], "end": [537, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40eae56782162244b93ec1c2"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 32], "end": [537, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-353c55d53ce6c5dfb508bd25"></a>
## savepoint_id

`struct_field` · `arrow_flight::sql::gen::ActionBeginSavepointResult::savepoint_id` · arrow-flight 59.3.0

```rust
savepoint_id: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:541`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the savepoint on the server.

<a id="op-f6869f3fb16850a9d8679ee9"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionBeginSavepointResult::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointResult", "path": "ActionBeginSavepointResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
