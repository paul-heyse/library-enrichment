# `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.command_statement_ingest.table_definition_options.TableExistsOption.json).

<a id="op-836bbcf1dbc9817f9abc6ecc"></a>
## TableExistsOption

`enum` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption` · arrow-flight 59.3.0

```rust
enum TableExistsOption
```

Source: `src/sql/arrow.flight.protocol.sql.rs:881`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The action to take if the target table already exists

<a id="op-c2651f7465a0cf6cc9c14a46"></a>
## Append

`variant` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::Append` · arrow-flight 59.3.0

```rust
Append
```

Source: `src/sql/arrow.flight.protocol.sql.rs:887`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Append to the table if it already exists

<a id="op-39a3db159bede45f10cfd908"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [878, 13], "end": [878, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:878`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-344870bb2e4ae700a0c5a985"></a>
## Fail

`variant` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::Fail` · arrow-flight 59.3.0

```rust
Fail
```

Source: `src/sql/arrow.flight.protocol.sql.rs:885`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Fail if the table already exists

<a id="op-a117e46423632eb7ac4ef7c4"></a>
## Replace

`variant` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::Replace` · arrow-flight 59.3.0

```rust
Replace
```

Source: `src/sql/arrow.flight.protocol.sql.rs:889`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Drop and recreate the table if it already exists

<a id="op-a8615ec206e2ae7510faa360"></a>
## Unspecified

`variant` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::Unspecified` · arrow-flight 59.3.0

```rust
Unspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:883`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Do not use. Servers should error if this is specified by a client.

<a id="op-61d703980bf2476cd0110c64"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 9], "end": [914, 10], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:896`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-4a59519de9094c678f70161d"></a>
## clone

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> TableExistsOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 13], "end": [870, 18], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:870`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5191bb4c3880e0171bfd0c9"></a>
## cmp

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &TableExistsOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 13], "end": [877, 16], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:877`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a23f504050a94546aaea1b2"></a>
## default

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::default` · arrow-flight 59.3.0

```rust
fn default() -> TableExistsOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [878, 13], "end": [878, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:878`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3faaf4da4bb9705880557a5d"></a>
## eq

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &TableExistsOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [873, 13], "end": [873, 22], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:873`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2a0a646ca089563f991114d"></a>
## fmt

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 13], "end": [872, 18], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:872`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a9d225857fea608fc17b7d6"></a>
## from_i32

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<TableExistsOption>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [878, 13], "end": [878, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:878`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `TableExistsOption`, or `None` if `value` is not a valid variant.

<a id="op-eaae3ab240ceb1714ede1dbd"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 9], "end": [914, 10], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:905`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-4898d27967a14e1f8aa1ac43"></a>
## hash

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [875, 13], "end": [875, 17], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:875`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0cc7d30ed39d298fc8927da"></a>
## is_valid

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [878, 13], "end": [878, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:878`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `TableExistsOption`.

<a id="op-1935c375acda2fe7189d292d"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &TableExistsOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [876, 13], "end": [876, 23], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:876`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75611af4c48ccfe0315a7a13"></a>
## try_from

`function` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<TableExistsOption, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption", "path": "TableExistsOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [878, 13], "end": [878, 33], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:878`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
