# `arrow_flight::sql::gen::CommandStatementIngest`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandStatementIngest.json).

<a id="op-5cebb99e653d3092d9549e2c"></a>
## CommandStatementIngest

`struct` · `arrow_flight::sql::gen::CommandStatementIngest` · arrow-flight 59.3.0

```rust
struct CommandStatementIngest
```

Source: `src/sql/arrow.flight.protocol.sql.rs:778`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a bulk ingestion request. Used in the command member of FlightDescriptor
for the the RPC call DoPut to cause the server load the contents of the stream's
FlightData into the target destination.

<a id="op-b707cb40213957db079acbae"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandStatementIngest::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75d1469a66de93783f7abca9"></a>
## catalog

`function` · `arrow_flight::sql::gen::CommandStatementIngest::catalog` · arrow-flight 59.3.0

```rust
fn catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 28], "end": [777, 44], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `catalog`, or the default value if `catalog` is unset.

<a id="op-7da3a5d2c4993eb6aff8719e"></a>
## catalog

`struct_field` · `arrow_flight::sql::gen::CommandStatementIngest::catalog` · arrow-flight 59.3.0

```rust
catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:792`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The catalog of the destination table to load data into. If unset, a backend-specific default may be used.

<a id="op-1fb40bf64824e512b883bd48"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandStatementIngest::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 28], "end": [777, 44], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93a99043f289314cda745604"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandStatementIngest::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandStatementIngest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 10], "end": [777, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5b473cf7fb38eec64aef87e"></a>
## default

`function` · `arrow_flight::sql::gen::CommandStatementIngest::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 28], "end": [777, 44], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef0542388599cb574ac8251b"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandStatementIngest::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 28], "end": [777, 44], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8120d8fc56e543ba395120ec"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandStatementIngest::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandStatementIngest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 17], "end": [777, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd8e94f3561e6fdb80f1b5d8"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandStatementIngest::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 28], "end": [777, 44], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e8a936a666f94a60b38863c"></a>
## options

`struct_field` · `arrow_flight::sql::gen::CommandStatementIngest::options` · arrow-flight 59.3.0

```rust
options: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:805`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Backend-specific options.

<a id="op-14b8a9c9736a113cf3ab6191"></a>
## schema

`struct_field` · `arrow_flight::sql::gen::CommandStatementIngest::schema` · arrow-flight 59.3.0

```rust
schema: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:789`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The db_schema of the destination table to load data into. If unset, a backend-specific default may be used.

<a id="op-dd2e18da12ce55d313968d27"></a>
## schema

`function` · `arrow_flight::sql::gen::CommandStatementIngest::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 28], "end": [777, 44], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `schema`, or the default value if `schema` is unset.

<a id="op-5fd39a39ed90fbfc65f61ac4"></a>
## table

`struct_field` · `arrow_flight::sql::gen::CommandStatementIngest::table` · arrow-flight 59.3.0

```rust
table: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:786`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The table to load data into.

<a id="op-d245ccec59274a3397b259de"></a>
## table_definition_options

`struct_field` · `arrow_flight::sql::gen::CommandStatementIngest::table_definition_options` · arrow-flight 59.3.0

```rust
table_definition_options: ::core::option::Option<command_statement_ingest::TableDefinitionOptions>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:781`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The behavior for handling the table definition.

<a id="op-01edc0c2826e2c5e3dc4ba1b"></a>
## temporary

`struct_field` · `arrow_flight::sql::gen::CommandStatementIngest::temporary` · arrow-flight 59.3.0

```rust
temporary: bool
```

Source: `src/sql/arrow.flight.protocol.sql.rs:799`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Store ingested data in a temporary table.
The effect of setting temporary is to place the table in a backend-defined namespace, and to drop the table at the end of the session.
The namespacing may make use of a backend-specific schema and/or catalog.
The server should return an error if an explicit choice of schema or catalog is incompatible with the server's namespacing decision.

<a id="op-65fafa1d66fa531227da3895"></a>
## transaction_id

`function` · `arrow_flight::sql::gen::CommandStatementIngest::transaction_id` · arrow-flight 59.3.0

```rust
fn transaction_id(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 28], "end": [777, 44], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `transaction_id`, or the default value if `transaction_id` is unset.

<a id="op-accd7fedcfdf1089789a4687"></a>
## transaction_id

`struct_field` · `arrow_flight::sql::gen::CommandStatementIngest::transaction_id` · arrow-flight 59.3.0

```rust
transaction_id: ::core::option::Option<::prost::bytes::Bytes>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:802`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Perform the ingestion as part of this transaction. If specified, results should not be committed in the event of an error/cancellation.

<a id="op-cf49251bd933e7938cdd863f"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandStatementIngest::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandStatementIngest", "path": "CommandStatementIngest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
