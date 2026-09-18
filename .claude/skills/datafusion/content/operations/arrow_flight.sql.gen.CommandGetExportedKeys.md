# `arrow_flight::sql::gen::CommandGetExportedKeys`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetExportedKeys.json).

<a id="op-4e5ddf8154e18e7b0a1c4ece"></a>
## CommandGetExportedKeys

`struct` · `arrow_flight::sql::gen::CommandGetExportedKeys` · arrow-flight 59.3.0

```rust
struct CommandGetExportedKeys
```

Source: `src/sql/arrow.flight.protocol.sql.rs:291`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve a description of the foreign key columns that reference the given table's
primary key columns (the foreign keys exported by a table) of a table on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  pk_catalog_name: utf8,
  pk_db_schema_name: utf8,
  pk_table_name: utf8 not null,
  pk_column_name: utf8 not null,
  fk_catalog_name: utf8,
  fk_db_schema_name: utf8,
  fk_table_name: utf8 not null,
  fk_column_name: utf8 not null,
  key_sequence: int32 not null,
  fk_key_name: utf8,
  pk_key_name: utf8,
  update_rule: uint8 not null,
  delete_rule: uint8 not null
>
The returned data should be ordered by fk_catalog_name, fk_db_schema_name, fk_table_name, fk_key_name, then key_sequence.
update_rule and delete_rule returns a byte that is equivalent to actions declared on UpdateDeleteRules enum.

<a id="op-61378c1c958308002b7d6c13"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a6169e538f1c2fad7c13967"></a>
## catalog

`struct_field` · `arrow_flight::sql::gen::CommandGetExportedKeys::catalog` · arrow-flight 59.3.0

```rust
catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:297`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the catalog to search for the foreign key table.
An empty string retrieves those without a catalog.
If omitted the catalog name should not be used to narrow the search.

<a id="op-8d9f23e5aa81d2c1a72f703e"></a>
## catalog

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::catalog` · arrow-flight 59.3.0

```rust
fn catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 38], "end": [290, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `catalog`, or the default value if `catalog` is unset.

<a id="op-37ab4045301b84ce16415160"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 38], "end": [290, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a289c4d8fb78d93aff345e"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetExportedKeys
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 10], "end": [290, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f17473d5f413e2148c42f6e"></a>
## db_schema

`struct_field` · `arrow_flight::sql::gen::CommandGetExportedKeys::db_schema` · arrow-flight 59.3.0

```rust
db_schema: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:303`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the schema to search for the foreign key table.
An empty string retrieves those without a schema.
If omitted the schema name should not be used to narrow the search.

<a id="op-9a34fbd00e0939efc44535a0"></a>
## db_schema

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::db_schema` · arrow-flight 59.3.0

```rust
fn db_schema(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 38], "end": [290, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `db_schema`, or the default value if `db_schema` is unset.

<a id="op-6cde7b888a6122087632ca5b"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 38], "end": [290, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4710cf9e662893a043e7c7b6"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 38], "end": [290, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbbbfa51ed9c8e68d638d0a2"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetExportedKeys) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 17], "end": [290, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-915ce2c6aa124208bd35e776"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 38], "end": [290, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0411200e8b61919d0b56b453"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 32], "end": [290, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a7b8d168493b325c2245f63"></a>
## table

`struct_field` · `arrow_flight::sql::gen::CommandGetExportedKeys::table` · arrow-flight 59.3.0

```rust
table: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:306`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specifies the foreign key table to get the foreign keys for.

<a id="op-f8b7a632031c4b887d1fc319"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetExportedKeys::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetExportedKeys", "path": "CommandGetExportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
