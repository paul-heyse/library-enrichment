# `arrow_flight::sql::gen::ActionClosePreparedStatementRequest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionClosePreparedStatementRequest.json).

<a id="op-a1d885cb48d58e33033ff5a1"></a>
## ActionClosePreparedStatementRequest

`struct` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest` · arrow-flight 59.3.0

```rust
struct ActionClosePreparedStatementRequest
```

Source: `src/sql/arrow.flight.protocol.sql.rs:490`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Request message for the "ClosePreparedStatement" action on a Flight SQL enabled backend.
Closes server resources associated with the prepared statement handle.

<a id="op-2add872276d912246436a58f"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f941668bff975066cf00a1ac"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 38], "end": [489, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d00168487fbf30ebc351e20d"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionClosePreparedStatementRequest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 10], "end": [489, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40cd5f1a49bcc9b18b73afec"></a>
## default

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 38], "end": [489, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab750f349e398c5e7000ca55"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 38], "end": [489, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6673b2004dd23393e1b3f45"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionClosePreparedStatementRequest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 17], "end": [489, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-867580ba86130e5a79e309d2"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 38], "end": [489, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bace7485723dcec4d71c4b6"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 32], "end": [489, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9774c056f7253fd4ae0ef04"></a>
## prepared_statement_handle

`struct_field` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::prepared_statement_handle` · arrow-flight 59.3.0

```rust
prepared_statement_handle: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:493`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the prepared statement on the server.

<a id="op-1d2d031760a1964a01f31a6e"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionClosePreparedStatementRequest", "path": "ActionClosePreparedStatementRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
