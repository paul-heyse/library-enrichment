# `arrow_flight::sql::gen::CommandGetCatalogs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetCatalogs.json).

<a id="op-79bcbfce6a9ae0588357547a"></a>
## CommandGetCatalogs

`struct` · `arrow_flight::sql::gen::CommandGetCatalogs` · arrow-flight 59.3.0

```rust
struct CommandGetCatalogs
```

Source: `src/sql/arrow.flight.protocol.sql.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve the list of catalogs on a Flight SQL enabled backend.
The definition of a catalog depends on vendor/implementation. It is usually the database itself
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8 not null
>
The returned data should be ordered by catalog_name.

<a id="op-c585f95c3861f1633e21167d"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94bc61ec264dffc0f6d1e60b"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 44], "end": [121, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98eecb0abb4b0b7cd6a245f9"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetCatalogs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59e16f3135f89ae5d1cb467d"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 44], "end": [121, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76782e0fcd3650cc1d892602"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 44], "end": [121, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a0b19e4634b707d675ba186"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetCatalogs) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 23], "end": [121, 32], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cf770abcbe764d9368213c0"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 44], "end": [121, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30b394e34745015005fc07d9"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 38], "end": [121, 42], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a00211cc6af14c632ba60503"></a>
## into_builder

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::into_builder` · arrow-flight 59.3.0

```rust
fn into_builder(self) -> GetCatalogsBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "crate::sql::CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [41, 2], "filename": "src/sql/metadata/catalogs.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/catalogs.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a builder suitable for constructing a response

<a id="op-c8f722340d5e268acb1c63c9"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetCatalogs::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCatalogs", "path": "CommandGetCatalogs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
