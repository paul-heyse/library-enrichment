# `arrow_flight::sql::gen::CommandGetTableTypes`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetTableTypes.json).

<a id="op-916ae65cbb0bc1b8b0441a53"></a>
## CommandGetTableTypes

`struct` · `arrow_flight::sql::gen::CommandGetTableTypes` · arrow-flight 59.3.0

```rust
struct CommandGetTableTypes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve the list of table types on a Flight SQL enabled backend.
The table types depend on vendor/implementation. It is usually used to separate tables from views or system tables.
TABLE, VIEW, and SYSTEM TABLE are commonly supported.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  table_type: utf8 not null
>
The returned data should be ordered by table_type.

<a id="op-7a29ba60ff03451538fbc09b"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-128d0655b54d4546ed4eec30"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 44], "end": [229, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff1ce37bbdd1953f1f01e762"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetTableTypes
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 10], "end": [229, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e28469a3cff01365fd810e8"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 44], "end": [229, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24b857525affbf03c2f2bd0e"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 44], "end": [229, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b5cf7c455b51a1ce961701b"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetTableTypes) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 23], "end": [229, 32], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2515863529c2af8cfd3ef71a"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 44], "end": [229, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d1313bbc97042449bcabfa6"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 38], "end": [229, 42], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65dd48287a7b8cdd6465a6b3"></a>
## into_builder

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::into_builder` · arrow-flight 59.3.0

```rust
fn into_builder(self) -> GetTableTypesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "crate::sql::CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [50, 2], "filename": "src/sql/metadata/table_types.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/table_types.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a builder suitable for constructing a response

<a id="op-9752906c0a89805b2d919dab"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetTableTypes::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTableTypes", "path": "CommandGetTableTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
