# `arrow_flight::sql::gen::CommandStatementSubstraitPlan`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandStatementSubstraitPlan.json).

<a id="op-d2848abe3ad9bd250398f49a"></a>
## CommandStatementSubstraitPlan

`struct` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan` · arrow-flight 59.3.0

```rust
struct CommandStatementSubstraitPlan
```

Source: `src/sql/arrow.flight.protocol.sql.rs:708`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a Substrait plan. Used in the command member of FlightDescriptor
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
  - DoPut: execute the query.

<a id="op-4cdd00c771b106ffdc98b6f6"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ca3ac13e438e1cdc8fdfb29"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 38], "end": [707, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26940ea160d89fb2fba9b260"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandStatementSubstraitPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 10], "end": [707, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-918651a608f4465444ed31c2"></a>
## default

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 38], "end": [707, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-036eb891ab4987420b54dfc9"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 38], "end": [707, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df40a1d5a66367b0455f7f27"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandStatementSubstraitPlan) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 17], "end": [707, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf218b28d3ccb35b95abb9ac"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 38], "end": [707, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4be0440e35d0f824e2c8bd5"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 32], "end": [707, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50399c78c8663fe947b69413"></a>
## plan

`struct_field` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::plan` · arrow-flight 59.3.0

```rust
plan: ::core::option::Option<SubstraitPlan>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:711`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A serialized substrait.Plan

<a id="op-0b7497e151ff9225cf09ad72"></a>
## transaction_id

`struct_field` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::transaction_id` · arrow-flight 59.3.0

```rust
transaction_id: ::core::option::Option<::prost::bytes::Bytes>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:714`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Include the query as part of this transaction (if unset, the query is auto-committed).

<a id="op-b5db1f9643c388c37ed5ae28"></a>
## transaction_id

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::transaction_id` · arrow-flight 59.3.0

```rust
fn transaction_id(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 38], "end": [707, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:707`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `transaction_id`, or the default value if `transaction_id` is unset.

<a id="op-11ee970351baf3519eceb056"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementSubstraitPlan", "path": "CommandStatementSubstraitPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
