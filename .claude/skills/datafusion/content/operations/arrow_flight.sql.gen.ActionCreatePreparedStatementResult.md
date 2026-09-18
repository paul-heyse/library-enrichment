# `arrow_flight::sql::gen::ActionCreatePreparedStatementResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.ActionCreatePreparedStatementResult.json).

<a id="op-cf81a99721af3a73f15fa2b7"></a>
## ActionCreatePreparedStatementResult

`struct` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult` · arrow-flight 59.3.0

```rust
struct ActionCreatePreparedStatementResult
```

Source: `src/sql/arrow.flight.protocol.sql.rs:470`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Wrap the result of a "CreatePreparedStatement" or "CreatePreparedSubstraitPlan" action.

The resultant PreparedStatement can be closed either:
- Manually, through the "ClosePreparedStatement" action;
- Automatically, by a server timeout.

The result should be wrapped in a google.protobuf.Any message.

<a id="op-c08d2837d924f22f9410e06b"></a>
## as_any

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1661ec3c46ab943f3020aba3"></a>
## clear

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 38], "end": [469, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c809309a27b1837281af043"></a>
## clone

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> ActionCreatePreparedStatementResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 10], "end": [469, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a86778430e208444a912553"></a>
## dataset_schema

`struct_field` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::dataset_schema` · arrow-flight 59.3.0

```rust
dataset_schema: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:480`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

If a result set generating query was provided, dataset_schema contains the
schema of the result set.  It should be an IPC-encapsulated Schema, as described in Schema.fbs.
For some queries, the schema of the results may depend on the schema of the parameters.  The server
should provide its best guess as to the schema at this point.  Clients must not assume that this
schema, if provided, will be accurate.

<a id="op-8ec9163ae3c47aace43e2dfc"></a>
## default

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 38], "end": [469, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2fc375e4579af05b53489b5"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 38], "end": [469, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dc0c765518d9c2d2a295e85"></a>
## eq

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &ActionCreatePreparedStatementResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 17], "end": [469, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2daa21872fbb0a658e7cf5ee"></a>
## fmt

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 38], "end": [469, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f8ee844a6e04778fb0ed844"></a>
## hash

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [469, 32], "end": [469, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04f15f1cc460372c53c2f4fc"></a>
## parameter_schema

`struct_field` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::parameter_schema` · arrow-flight 59.3.0

```rust
parameter_schema: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:484`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

If the query provided contained parameters, parameter_schema contains the
schema of the expected parameters.  It should be an IPC-encapsulated Schema, as described in Schema.fbs.

<a id="op-f3cb1731ab80ae3defd3b3e5"></a>
## prepared_statement_handle

`struct_field` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::prepared_statement_handle` · arrow-flight 59.3.0

```rust
prepared_statement_handle: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:473`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the prepared statement on the server.

<a id="op-886792470653e084dddbaa95"></a>
## type_url

`function` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::ActionCreatePreparedStatementResult", "path": "ActionCreatePreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
