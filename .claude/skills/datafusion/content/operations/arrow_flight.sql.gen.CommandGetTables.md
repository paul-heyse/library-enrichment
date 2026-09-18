# `arrow_flight::sql::gen::CommandGetTables`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetTables.json).

<a id="op-b356eca1c40464b8a7ecea0a"></a>
## CommandGetTables

`struct` · `arrow_flight::sql::gen::CommandGetTables` · arrow-flight 59.3.0

```rust
struct CommandGetTables
```

Source: `src/sql/arrow.flight.protocol.sql.rs:181`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve the list of tables, and optionally their schemas, on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8,
  db_schema_name: utf8,
  table_name: utf8 not null,
  table_type: utf8 not null,
  \[optional\] table_schema: bytes not null (schema of the table as described in Schema.fbs::Schema,
                                           it is serialized as an IPC message.)
>
Fields on table_schema may contain the following metadata:
  - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
  - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
  - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
  - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
  - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
  - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
  - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
  - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
  - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
  - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
The returned data should be ordered by catalog_name, db_schema_name, table_name, then table_type, followed by table_schema if requested.

<a id="op-a59f05b23085101285d75e21"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetTables::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5512ec1d9d4d241ff37a69c1"></a>
## catalog

`struct_field` · `arrow_flight::sql::gen::CommandGetTables::catalog` · arrow-flight 59.3.0

```rust
catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the Catalog to search for the tables.
An empty string retrieves those without a catalog.
If omitted the catalog name should not be used to narrow the search.

<a id="op-568767e319a5fbe98f74beb9"></a>
## catalog

`function` · `arrow_flight::sql::gen::CommandGetTables::catalog` · arrow-flight 59.3.0

```rust
fn catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 38], "end": [180, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `catalog`, or the default value if `catalog` is unset.

<a id="op-994e9fe7941548903ffdf008"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetTables::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 38], "end": [180, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fdba71289d189c2a66c76e4"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetTables::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetTables
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 10], "end": [180, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cca099c9250821b843a87b1"></a>
## db_schema_filter_pattern

`struct_field` · `arrow_flight::sql::gen::CommandGetTables::db_schema_filter_pattern` · arrow-flight 59.3.0

```rust
db_schema_filter_pattern: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:195`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies a filter pattern for schemas to search for.
When no db_schema_filter_pattern is provided, all schemas matching other filters are searched.
In the pattern string, two special characters can be used to denote matching rules:
    - "%" means to match any substring with 0 or more characters.
    - "_" means to match any one character.

<a id="op-b258d7dc1515a945af2a7c5c"></a>
## db_schema_filter_pattern

`function` · `arrow_flight::sql::gen::CommandGetTables::db_schema_filter_pattern` · arrow-flight 59.3.0

```rust
fn db_schema_filter_pattern(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 38], "end": [180, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `db_schema_filter_pattern`, or the default value if `db_schema_filter_pattern` is unset.

<a id="op-e0ae326e3b8f55c8e9c20315"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetTables::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 38], "end": [180, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6e7cd233ffc69ecd191caec"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetTables::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 38], "end": [180, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6290f0bede69d82299389504"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetTables::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetTables) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 17], "end": [180, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e172088ac566e136bc1fe331"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetTables::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 38], "end": [180, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10c3b167ddb1c1dc67214937"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetTables::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 32], "end": [180, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a899173e1bb608a0100d8ecf"></a>
## include_schema

`struct_field` · `arrow_flight::sql::gen::CommandGetTables::include_schema` · arrow-flight 59.3.0

```rust
include_schema: bool
```

Source: `src/sql/arrow.flight.protocol.sql.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specifies if the Arrow schema should be returned for found tables.

<a id="op-0c83aefc67a1c5e70a86113d"></a>
## into_builder

`function` · `arrow_flight::sql::gen::CommandGetTables::into_builder` · arrow-flight 59.3.0

```rust
fn into_builder(self) -> GetTablesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "crate::sql::CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [72, 2], "filename": "src/sql/metadata/tables.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/tables.rs:69`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a builder suitable for constructing a response

<a id="op-dfcd1b65ae15829ca3ce67b4"></a>
## table_name_filter_pattern

`struct_field` · `arrow_flight::sql::gen::CommandGetTables::table_name_filter_pattern` · arrow-flight 59.3.0

```rust
table_name_filter_pattern: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:203`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies a filter pattern for tables to search for.
When no table_name_filter_pattern is provided, all tables matching other filters are searched.
In the pattern string, two special characters can be used to denote matching rules:
    - "%" means to match any substring with 0 or more characters.
    - "_" means to match any one character.

<a id="op-eb3fd86d2f0e7c286f20d0a7"></a>
## table_name_filter_pattern

`function` · `arrow_flight::sql::gen::CommandGetTables::table_name_filter_pattern` · arrow-flight 59.3.0

```rust
fn table_name_filter_pattern(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 38], "end": [180, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `table_name_filter_pattern`, or the default value if `table_name_filter_pattern` is unset.

<a id="op-9b6af1505bd98b6b364ca560"></a>
## table_types

`struct_field` · `arrow_flight::sql::gen::CommandGetTables::table_types` · arrow-flight 59.3.0

```rust
table_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:211`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies a filter of table types which must match.
The table types depend on vendor/implementation. It is usually used to separate tables from views or system tables.
TABLE, VIEW, and SYSTEM TABLE are commonly supported.

<a id="op-6b938581add7be22b5dfe97c"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetTables::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
