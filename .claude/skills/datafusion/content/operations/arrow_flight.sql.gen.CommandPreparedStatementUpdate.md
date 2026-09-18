# `arrow_flight::sql::gen::CommandPreparedStatementUpdate`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandPreparedStatementUpdate.json).

<a id="op-59be283aaa97bf50bd28d514"></a>
## CommandPreparedStatementUpdate

`struct` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate` · arrow-flight 59.3.0

```rust
struct CommandPreparedStatementUpdate
```

Source: `src/sql/arrow.flight.protocol.sql.rs:768`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a SQL update query. Used in the command member of FlightDescriptor
for the RPC call DoPut to cause the server to execute the included
prepared statement handle as an update.

<a id="op-0416c1ce182b24e2e2ac068b"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84fe608a25135367c2d29067"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 38], "end": [767, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-350da7db9107be733ea9f0ef"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandPreparedStatementUpdate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 10], "end": [767, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cf05060754630917d192afe"></a>
## default

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 38], "end": [767, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d47cd8836a9902e0ee87402"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 38], "end": [767, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fe4ac872a3570cf451304ea"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandPreparedStatementUpdate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 17], "end": [767, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29209e65919185be54b67dbc"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 38], "end": [767, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-930f40a3dd81f641859076ad"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 32], "end": [767, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d098ed7a9b3990f8f3b2e78"></a>
## prepared_statement_handle

`struct_field` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::prepared_statement_handle` · arrow-flight 59.3.0

```rust
prepared_statement_handle: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:771`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the prepared statement on the server.

<a id="op-d7773b80f207b94091e6b33c"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementUpdate", "path": "CommandPreparedStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
