# `arrow_flight::sql::gen::CommandStatementQuery`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandStatementQuery.json).

<a id="op-27d3fede133f129d418db1cf"></a>
## CommandStatementQuery

`struct` · `arrow_flight::sql::gen::CommandStatementQuery` · arrow-flight 59.3.0

```rust
struct CommandStatementQuery
```

Source: `src/sql/arrow.flight.protocol.sql.rs:682`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a SQL query. Used in the command member of FlightDescriptor
for the following RPC calls:
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
  - GetFlightInfo: execute the query.

<a id="op-3f684c2694f3ec2639082859"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandStatementQuery::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee4bd9d0713b1b2a8eea1e74"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandStatementQuery::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 38], "end": [681, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c62c776c5780673f846e626"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandStatementQuery::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandStatementQuery
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 10], "end": [681, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d8df79c1f25f1aaaa8b8a0d"></a>
## default

`function` · `arrow_flight::sql::gen::CommandStatementQuery::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 38], "end": [681, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97fecdeae3b2aef2ec0a5064"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandStatementQuery::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 38], "end": [681, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-727f7daba7ecfa2f3590c552"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandStatementQuery::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandStatementQuery) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 17], "end": [681, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8daab81d6c044869474d26d"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandStatementQuery::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 38], "end": [681, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c7f92a34d168df6d40946d3"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandStatementQuery::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 32], "end": [681, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f1d0eff2a4acc811029fea9"></a>
## query

`struct_field` · `arrow_flight::sql::gen::CommandStatementQuery::query` · arrow-flight 59.3.0

```rust
query: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:685`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The SQL syntax.

<a id="op-b2ef1f3a746e97d866f39544"></a>
## transaction_id

`struct_field` · `arrow_flight::sql::gen::CommandStatementQuery::transaction_id` · arrow-flight 59.3.0

```rust
transaction_id: ::core::option::Option<::prost::bytes::Bytes>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:688`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Include the query as part of this transaction (if unset, the query is auto-committed).

<a id="op-e6ad52cf02cc44386bd21f69"></a>
## transaction_id

`function` · `arrow_flight::sql::gen::CommandStatementQuery::transaction_id` · arrow-flight 59.3.0

```rust
fn transaction_id(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 38], "end": [681, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `transaction_id`, or the default value if `transaction_id` is unset.

<a id="op-6b970a1f5c3a7bcccd24ff8d"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandStatementQuery::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementQuery", "path": "CommandStatementQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
