# `arrow_flight::sql::gen::ActionBeginTransactionResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionBeginTransactionResult.json).

<a id="op-978828b3fb0d208652f0d648"></a>
## ActionBeginTransactionResult

`struct` · `arrow_flight::sql::gen::ActionBeginTransactionResult` · arrow-flight 59.3.0

```rust
struct ActionBeginTransactionResult
```

Source: `src/sql/arrow.flight.protocol.sql.rs:524`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The result of a "BeginTransaction" action.

The transaction can be manipulated with the "EndTransaction" action, or
automatically via server timeout. If the transaction times out, then it is
automatically rolled back.

The result should be wrapped in a google.protobuf.Any message.

<a id="op-35f7f2b1b58cc6e511366900"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57c45f3b9f64dee7f73904ba"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 38], "end": [523, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86e9a6ec1707940d224da411"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionBeginTransactionResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 10], "end": [523, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1544fa3045989c428169da49"></a>
## default

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 38], "end": [523, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f283e27c0d8d665627ff6b8c"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 38], "end": [523, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae29fc46f2e89a3a75bdb63d"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionBeginTransactionResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 17], "end": [523, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-978aff6ae451ba92f276ddb8"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 38], "end": [523, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3b5cd6b898ccb139bad488d"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 32], "end": [523, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:523`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9702538f200fa9fa85c928f"></a>
## transaction_id

`struct_field` · `arrow_flight::sql::gen::ActionBeginTransactionResult::transaction_id` · arrow-flight 59.3.0

```rust
transaction_id: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:527`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the transaction on the server.

<a id="op-770ed347f9146129e3d81570"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionBeginTransactionResult::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionResult", "path": "ActionBeginTransactionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
