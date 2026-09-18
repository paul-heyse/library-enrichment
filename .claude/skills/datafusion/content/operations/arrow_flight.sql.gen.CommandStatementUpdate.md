# `arrow_flight::sql::gen::CommandStatementUpdate`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandStatementUpdate.json).

<a id="op-749109f6dbce6164ef417241"></a>
## CommandStatementUpdate

`struct` · `arrow_flight::sql::gen::CommandStatementUpdate` · arrow-flight 59.3.0

```rust
struct CommandStatementUpdate
```

Source: `src/sql/arrow.flight.protocol.sql.rs:755`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a SQL update query. Used in the command member of FlightDescriptor
for the RPC call DoPut to cause the server to execute the included SQL update.

<a id="op-e3b96e7c3dd69e8ab92630fe"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecb4fa79c4a232476d06c0fa"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 38], "end": [754, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21cece59ad2e2b571e0a7031"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandStatementUpdate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 10], "end": [754, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f092f2541e10bec8e6eb5008"></a>
## default

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 38], "end": [754, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-433e3d3f799da62a2f2be97d"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 38], "end": [754, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0bf0a256ef8225e34b0eecf"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandStatementUpdate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 17], "end": [754, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35aab86702749f2bffdfa7a3"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 38], "end": [754, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e3ada7d232f575745935f8"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 32], "end": [754, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bc6a1cf6b19c8020380a4f1"></a>
## query

`struct_field` · `arrow_flight::sql::gen::CommandStatementUpdate::query` · arrow-flight 59.3.0

```rust
query: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:758`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The SQL syntax.

<a id="op-212188b72b7757f47d8f6a53"></a>
## transaction_id

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::transaction_id` · arrow-flight 59.3.0

```rust
fn transaction_id(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 38], "end": [754, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:754`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `transaction_id`, or the default value if `transaction_id` is unset.

<a id="op-acb1c998df7a7c64a1cad0cf"></a>
## transaction_id

`struct_field` · `arrow_flight::sql::gen::CommandStatementUpdate::transaction_id` · arrow-flight 59.3.0

```rust
transaction_id: ::core::option::Option<::prost::bytes::Bytes>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:761`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Include the query as part of this transaction (if unset, the query is auto-committed).

<a id="op-148153e3ffeae931984aae27"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandStatementUpdate::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementUpdate", "path": "CommandStatementUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
