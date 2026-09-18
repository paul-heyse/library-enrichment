# `arrow_flight::sql::gen::ActionBeginSavepointRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionBeginSavepointRequest.json).

<a id="op-66e4a803e2e9b35b96f0dcc5"></a>
## ActionBeginSavepointRequest

`struct` · `arrow_flight::sql::gen::ActionBeginSavepointRequest` · arrow-flight 59.3.0

```rust
struct ActionBeginSavepointRequest
```

Source: `src/sql/arrow.flight.protocol.sql.rs:507`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Request message for the "BeginSavepoint" action.
Creates a savepoint within a transaction.

Only supported if FLIGHT_SQL_TRANSACTION is
FLIGHT_SQL_TRANSACTION_SUPPORT_SAVEPOINT.

<a id="op-368844b3a55f7f2b102011af"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7cea78568edda5b2d0da248"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 38], "end": [506, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bbe18c2b954a261c8aaca27"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionBeginSavepointRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 10], "end": [506, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d1302a182a16e117317c129"></a>
## default

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 38], "end": [506, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a276d5246ec26cd5588cee18"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 38], "end": [506, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e83d9ed2484c28f21e9cecc"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionBeginSavepointRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 17], "end": [506, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cf1417fd3cbbe393217d1d0"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 38], "end": [506, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ce429691aef2cb044fda8f6"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 32], "end": [506, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:506`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dafe4f5df252c3b6a8d3ef0"></a>
## name

`struct_field` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::name` · arrow-flight 59.3.0

```rust
name: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:513`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Name for the savepoint.

<a id="op-42208290a7979292768ad2a0"></a>
## transaction_id

`struct_field` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::transaction_id` · arrow-flight 59.3.0

```rust
transaction_id: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The transaction to which a savepoint belongs.

<a id="op-c59121f876d159c585339d3b"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionBeginSavepointRequest::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionBeginSavepointRequest", "path": "ActionBeginSavepointRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
