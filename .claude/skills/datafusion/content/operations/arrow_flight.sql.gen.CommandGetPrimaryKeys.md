# `arrow_flight::sql::gen::CommandGetPrimaryKeys`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetPrimaryKeys.json).

<a id="op-1c97b131880004e4de032120"></a>
## CommandGetPrimaryKeys

`struct` · `arrow_flight::sql::gen::CommandGetPrimaryKeys` · arrow-flight 59.3.0

```rust
struct CommandGetPrimaryKeys
```

Source: `src/sql/arrow.flight.protocol.sql.rs:248`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve the primary keys of a table on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8,
  db_schema_name: utf8,
  table_name: utf8 not null,
  column_name: utf8 not null,
  key_name: utf8,
  key_sequence: int32 not null
>
The returned data should be ordered by catalog_name, db_schema_name, table_name, key_name, then key_sequence.

<a id="op-f021379983ee1c6bf8998270"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a3917ece3aa3b8fa70d692b"></a>
## catalog

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::catalog` · arrow-flight 59.3.0

```rust
fn catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 38], "end": [247, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `catalog`, or the default value if `catalog` is unset.

<a id="op-d954b1b8d4ca0ea4e12cc3d3"></a>
## catalog

`struct_field` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::catalog` · arrow-flight 59.3.0

```rust
catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the catalog to search for the table.
An empty string retrieves those without a catalog.
If omitted the catalog name should not be used to narrow the search.

<a id="op-eb479ac256af83b2754ce8bf"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 38], "end": [247, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aefd187d5ee29057e8d0512b"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetPrimaryKeys
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 10], "end": [247, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5016759c0afaf63a4df14496"></a>
## db_schema

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::db_schema` · arrow-flight 59.3.0

```rust
fn db_schema(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 38], "end": [247, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `db_schema`, or the default value if `db_schema` is unset.

<a id="op-f50798d07be4fe9e07d2cb94"></a>
## db_schema

`struct_field` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::db_schema` · arrow-flight 59.3.0

```rust
db_schema: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the schema to search for the table.
An empty string retrieves those without a schema.
If omitted the schema name should not be used to narrow the search.

<a id="op-95a960971a0ce889cdfb8425"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 38], "end": [247, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b361b66eb5983690df88cede"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 38], "end": [247, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae2e06786953448c6a1a5e4a"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetPrimaryKeys) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 17], "end": [247, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-856c1319d358ef63e5c24ff6"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 38], "end": [247, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9934831eff661d5f1966473"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 32], "end": [247, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28b602b9f9627dfb880c4b86"></a>
## table

`struct_field` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::table` · arrow-flight 59.3.0

```rust
table: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specifies the table to get the primary keys for.

<a id="op-79ae791cf63eb23787a0fa6a"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetPrimaryKeys::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetPrimaryKeys", "path": "CommandGetPrimaryKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
