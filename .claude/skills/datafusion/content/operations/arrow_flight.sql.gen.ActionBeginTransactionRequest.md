# `arrow_flight::sql::gen::ActionBeginTransactionRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionBeginTransactionRequest.json).

<a id="op-52ecd4a4f511455fc53be304"></a>
## ActionBeginTransactionRequest

`struct` · `arrow_flight::sql::gen::ActionBeginTransactionRequest` · arrow-flight 59.3.0

```rust
struct ActionBeginTransactionRequest
```

Source: `src/sql/arrow.flight.protocol.sql.rs:499`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Request message for the "BeginTransaction" action.
Begins a transaction.

<a id="op-b076373b610f50a44431a87e"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91facbe46dc86e4bcbe2035b"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 44], "end": [498, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4b3fdd142f9b200aabacd9e"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionBeginTransactionRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 10], "end": [498, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c17bad731c976a54cf6e0f17"></a>
## default

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 44], "end": [498, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-629f37c20807284fc090fadf"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 44], "end": [498, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3da684ca5f5471885573ac5"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionBeginTransactionRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 23], "end": [498, 32], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2accd6ac0b9cb27e6e78d2bd"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 44], "end": [498, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9f2d38e69c4e1fc810047a0"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 38], "end": [498, 42], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cef5a64258707c721119735"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionBeginTransactionRequest::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginTransactionRequest", "path": "ActionBeginTransactionRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
