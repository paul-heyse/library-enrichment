# `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.command_statement_ingest.table_definition_options.TableNotExistOption.json).

<a id="op-ef0f49fc14e8d29d569cb98f"></a>
## TableNotExistOption

`enum` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption` · arrow-flight 59.3.0

```rust
enum TableNotExistOption
```

Source: `src/sql/arrow.flight.protocol.sql.rs:838`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The action to take if the target table does not exist

<a id="op-faa9d8fe999e8f6319499b96"></a>
## Create

`variant` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::Create` · arrow-flight 59.3.0

```rust
Create
```

Source: `src/sql/arrow.flight.protocol.sql.rs:842`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create the table if it does not exist

<a id="op-9f1ac57364368662d2035283"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [835, 13], "end": [835, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:835`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a5e8bb8cf7e8ca6003b007e"></a>
## Fail

`variant` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::Fail` · arrow-flight 59.3.0

```rust
Fail
```

Source: `src/sql/arrow.flight.protocol.sql.rs:844`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Fail if the table does not exist

<a id="op-4f6b754ace09d120e42acd6d"></a>
## Unspecified

`variant` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::Unspecified` · arrow-flight 59.3.0

```rust
Unspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:840`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Do not use. Servers should error if this is specified by a client.

<a id="op-4b1c4cfb7053fa2f7a83a899"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [846, 9], "end": [867, 10], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:851`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-f711baa981af8e1deecaa300"></a>
## clone

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> TableNotExistOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [827, 13], "end": [827, 18], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:827`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a105ca421aba530a0ccde365"></a>
## cmp

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &TableNotExistOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [834, 13], "end": [834, 16], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:834`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-114c72380755c5dda9473f92"></a>
## default

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::default` · arrow-flight 59.3.0

```rust
fn default() -> TableNotExistOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [835, 13], "end": [835, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:835`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47b9a422ec1db440bebf67fe"></a>
## eq

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &TableNotExistOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 13], "end": [830, 22], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:830`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30ecd543a7ca8e57926dbb16"></a>
## fmt

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [829, 13], "end": [829, 18], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:829`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7df2e7e8da0a2f1a5c435938"></a>
## from_i32

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<TableNotExistOption>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [835, 13], "end": [835, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:835`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `TableNotExistOption`, or `None` if `value` is not a valid variant.

<a id="op-faf75216c6aa3758d00f2e79"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [846, 9], "end": [867, 10], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:859`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-1b3ae00c03e143acb1a60af7"></a>
## hash

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [832, 13], "end": [832, 17], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:832`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa926f23c8c756e7a3c09206"></a>
## is_valid

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [835, 13], "end": [835, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:835`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `TableNotExistOption`.

<a id="op-d6560f5dd234711185a273f2"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &TableNotExistOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [833, 13], "end": [833, 23], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:833`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be7f834e8b977e2af5a1c38f"></a>
## try_from

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<TableNotExistOption, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption", "path": "TableNotExistOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [835, 13], "end": [835, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:835`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
