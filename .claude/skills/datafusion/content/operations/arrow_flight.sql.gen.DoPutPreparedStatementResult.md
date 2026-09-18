# `arrow_flight::sql::gen::DoPutPreparedStatementResult`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.DoPutPreparedStatementResult.json).

<a id="op-e83016700d32fd3b42153771"></a>
## DoPutPreparedStatementResult

`struct` · `arrow_flight::sql::gen::DoPutPreparedStatementResult` · arrow-flight 59.3.0

```rust
struct DoPutPreparedStatementResult
```

Source: `src/sql/arrow.flight.protocol.sql.rs:934`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

An *optional* response returned when `DoPut` is called with `CommandPreparedStatementQuery`.

*Note on legacy behavior*: previous versions of the protocol did not return any result for
this command, and that behavior should still be supported by clients. In that case, the client
can continue as though the fields in this message were not provided or set to sensible default values.

<a id="op-6d0a9496b68e3d47822aa2c9"></a>
## as_any

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eed69fd0d8fc2c10e184f4eb"></a>
## clear

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 38], "end": [933, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16a0b71e96ee1f9fa5d340b4"></a>
## clone

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> DoPutPreparedStatementResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 10], "end": [933, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9ead0a625b315dbebae7e4b"></a>
## default

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 38], "end": [933, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28aa815c25411194299b4858"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 38], "end": [933, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17f1583a9bff6c41694e9110"></a>
## eq

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &DoPutPreparedStatementResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 17], "end": [933, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c15336266964c47845febd89"></a>
## fmt

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 38], "end": [933, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30e2e9e880d879016ac2138b"></a>
## hash

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 32], "end": [933, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38bd55fe92497cfb22cc7c29"></a>
## prepared_statement_handle

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::prepared_statement_handle` · arrow-flight 59.3.0

```rust
fn prepared_statement_handle(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 38], "end": [933, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `prepared_statement_handle`, or the default value if `prepared_statement_handle` is unset.

<a id="op-3fc37e58c7c7cc27d0917362"></a>
## prepared_statement_handle

`struct_field` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::prepared_statement_handle` · arrow-flight 59.3.0

```rust
prepared_statement_handle: ::core::option::Option<::prost::bytes::Bytes>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:944`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Represents a (potentially updated) opaque handle for the prepared statement on the server.
Because the handle could potentially be updated, any previous handles for this prepared
statement should be considered invalid, and all subsequent requests for this prepared
statement must use this new handle.
The updated handle allows implementing query parameters with stateless services.

When an updated handle is not provided by the server, clients should contiue
using the previous handle provided by `ActionCreatePreparedStatementResonse`.

<a id="op-78da7e48432b1123d97ad3ab"></a>
## type_url

`function` · `arrow_flight::sql::gen::DoPutPreparedStatementResult::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::DoPutPreparedStatementResult", "path": "DoPutPreparedStatementResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
