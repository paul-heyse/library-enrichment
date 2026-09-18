# `arrow_flight::sql::gen::CommandGetImportedKeys`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetImportedKeys.json).

<a id="op-ea55b91e0fa616b9a2bd2d4b"></a>
## CommandGetImportedKeys

`struct` · `arrow_flight::sql::gen::CommandGetImportedKeys` · arrow-flight 59.3.0

```rust
struct CommandGetImportedKeys
```

Source: `src/sql/arrow.flight.protocol.sql.rs:338`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve the foreign keys of a table on a Flight SQL enabled backend.
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
The returned data should be ordered by pk_catalog_name, pk_db_schema_name, pk_table_name, pk_key_name, then key_sequence.
update_rule and delete_rule returns a byte that is equivalent to actions:
    - 0 = CASCADE
    - 1 = RESTRICT
    - 2 = SET NULL
    - 3 = NO ACTION
    - 4 = SET DEFAULT

<a id="op-43b9e667e6ea932860c05b8f"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6738226668281cc80c51803a"></a>
## catalog

`struct_field` · `arrow_flight::sql::gen::CommandGetImportedKeys::catalog` · arrow-flight 59.3.0

```rust
catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:344`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the catalog to search for the primary key table.
An empty string retrieves those without a catalog.
If omitted the catalog name should not be used to narrow the search.

<a id="op-ee62032d0bef7f2c71b57e91"></a>
## catalog

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::catalog` · arrow-flight 59.3.0

```rust
fn catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 38], "end": [337, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `catalog`, or the default value if `catalog` is unset.

<a id="op-c1e93c8da337ff8401d260bf"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 38], "end": [337, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-535bc03f9735849cb7230e35"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetImportedKeys
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 10], "end": [337, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4717d6e441af1b2dfe7db119"></a>
## db_schema

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::db_schema` · arrow-flight 59.3.0

```rust
fn db_schema(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 38], "end": [337, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `db_schema`, or the default value if `db_schema` is unset.

<a id="op-5b701618c1fef6e543196b75"></a>
## db_schema

`struct_field` · `arrow_flight::sql::gen::CommandGetImportedKeys::db_schema` · arrow-flight 59.3.0

```rust
db_schema: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the schema to search for the primary key table.
An empty string retrieves those without a schema.
If omitted the schema name should not be used to narrow the search.

<a id="op-b14307967117df4fafad8b6c"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 38], "end": [337, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-763ae80b4e99178e92ed4d7b"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 38], "end": [337, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28016661cb89c86ea39655bc"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetImportedKeys) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 17], "end": [337, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7352d30a0fecb0c4943610c6"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 38], "end": [337, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3137fb44a6536e585e1a5b9d"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [337, 32], "end": [337, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ddcb0e3629e51d6a380d239"></a>
## table

`struct_field` · `arrow_flight::sql::gen::CommandGetImportedKeys::table` · arrow-flight 59.3.0

```rust
table: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:353`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specifies the primary key table to get the foreign keys for.

<a id="op-413cd8f6a2553ae3c378bfb0"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetImportedKeys::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetImportedKeys", "path": "CommandGetImportedKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
