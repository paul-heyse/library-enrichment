# `arrow_flight::sql::gen::CommandPreparedStatementQuery`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandPreparedStatementQuery.json).

<a id="op-fd414f3437a15ae94b40a253"></a>
## CommandPreparedStatementQuery

`struct` · `arrow_flight::sql::gen::CommandPreparedStatementQuery` · arrow-flight 59.3.0

```rust
struct CommandPreparedStatementQuery
```

Source: `src/sql/arrow.flight.protocol.sql.rs:746`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents an instance of executing a prepared statement. Used in the command member of FlightDescriptor for
the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
    Fields on this schema may contain the following metadata:
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

    If the schema is retrieved after parameter values have been bound with DoPut, then the server should account
    for the parameters when determining the schema.
  - DoPut: bind parameter values. All of the bound parameter sets will be executed as a single atomic execution.
  - GetFlightInfo: execute the prepared statement instance.

<a id="op-a50466e0c193e6510243abf4"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3af7659e3f75e524b65506f"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 38], "end": [745, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d55456c1496b45113b97049a"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandPreparedStatementQuery
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 10], "end": [745, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62a1d4e44e88d7b076454991"></a>
## default

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 38], "end": [745, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44d0702869ddfbd73ec84ad4"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 38], "end": [745, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80a4b11183b68cf475737cc8"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandPreparedStatementQuery) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 17], "end": [745, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-823229d263f95437ccfe22dd"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 38], "end": [745, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1161d4d2f0827076c70b95ae"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 32], "end": [745, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63668e8565265103afdfca3a"></a>
## prepared_statement_handle

`struct_field` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::prepared_statement_handle` · arrow-flight 59.3.0

```rust
prepared_statement_handle: ::prost::bytes::Bytes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:749`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Opaque handle for the prepared statement on the server.

<a id="op-3a2727f0cceac40de0817567"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandPreparedStatementQuery::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandPreparedStatementQuery", "path": "CommandPreparedStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
