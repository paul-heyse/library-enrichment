# `arrow_flight::sql::gen::CommandGetCrossReference`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetCrossReference.json).

<a id="op-3513e5ab2cbfca5bf7b8666f"></a>
## CommandGetCrossReference

`struct` · `arrow_flight::sql::gen::CommandGetCrossReference` · arrow-flight 59.3.0

```rust
struct CommandGetCrossReference
```

Source: `src/sql/arrow.flight.protocol.sql.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve a description of the foreign key columns in the given foreign key table that
reference the primary key or the columns representing a unique constraint of the parent table (could be the same
or a different table) on a Flight SQL enabled backend.
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

<a id="op-9f9f524f8b1ea3bd68673e69"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ba1b2b5ef0be08a40813794"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa3513152911b39ce1de0739"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetCrossReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 10], "end": [386, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7f737c0872429d465c3c706"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26f1bbfeee530bdf4492e52d"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbf6dede5291d654550c2fa0"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetCrossReference) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 17], "end": [386, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f2907c44afab95bd6aff6db"></a>
## fk_catalog

`struct_field` · `arrow_flight::sql::gen::CommandGetCrossReference::fk_catalog` · arrow-flight 59.3.0

```rust
fk_catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:409`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
The catalog name where the foreign table is.
An empty string retrieves those without a catalog.
If omitted the catalog name should not be used to narrow the search.

<a id="op-464fd8fde19ea7b099a91b64"></a>
## fk_catalog

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::fk_catalog` · arrow-flight 59.3.0

```rust
fn fk_catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `fk_catalog`, or the default value if `fk_catalog` is unset.

<a id="op-3c15f852f05bcf4e338bc75f"></a>
## fk_db_schema

`struct_field` · `arrow_flight::sql::gen::CommandGetCrossReference::fk_db_schema` · arrow-flight 59.3.0

```rust
fk_db_schema: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:415`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
The schema name where the foreign table is.
An empty string retrieves those without a schema.
If omitted the schema name should not be used to narrow the search.

<a id="op-9257b1f7779e21260e76854c"></a>
## fk_db_schema

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::fk_db_schema` · arrow-flight 59.3.0

```rust
fn fk_db_schema(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `fk_db_schema`, or the default value if `fk_db_schema` is unset.

<a id="op-0a96dafbd65153b0c5200c3f"></a>
## fk_table

`struct_field` · `arrow_flight::sql::gen::CommandGetCrossReference::fk_table` · arrow-flight 59.3.0

```rust
fk_table: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:419`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
The foreign table name. It cannot be null.

<a id="op-97ad7affea4786a2c7300540"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f2c933cd189627ede367d9d"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 32], "end": [386, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-343b2571ad31be54691ff725"></a>
## pk_catalog

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::pk_catalog` · arrow-flight 59.3.0

```rust
fn pk_catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `pk_catalog`, or the default value if `pk_catalog` is unset.

<a id="op-d6b8b2948af6842e3197bfba"></a>
## pk_catalog

`struct_field` · `arrow_flight::sql::gen::CommandGetCrossReference::pk_catalog` · arrow-flight 59.3.0

```rust
pk_catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:393`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
The catalog name where the parent table is.
An empty string retrieves those without a catalog.
If omitted the catalog name should not be used to narrow the search.

<a id="op-1ead4ba720116ff09e4efa64"></a>
## pk_db_schema

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::pk_db_schema` · arrow-flight 59.3.0

```rust
fn pk_db_schema(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 38], "end": [386, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `pk_db_schema`, or the default value if `pk_db_schema` is unset.

<a id="op-d53d98e97f386efff15cc70e"></a>
## pk_db_schema

`struct_field` · `arrow_flight::sql::gen::CommandGetCrossReference::pk_db_schema` · arrow-flight 59.3.0

```rust
pk_db_schema: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
The Schema name where the parent table is.
An empty string retrieves those without a schema.
If omitted the schema name should not be used to narrow the search.

<a id="op-ad6ab86f05bf04dcdd2eb08a"></a>
## pk_table

`struct_field` · `arrow_flight::sql::gen::CommandGetCrossReference::pk_table` · arrow-flight 59.3.0

```rust
pk_table: ::prost::alloc::string::String
```

Source: `src/sql/arrow.flight.protocol.sql.rs:403`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

*
The parent table name. It cannot be null.

<a id="op-2d8931c004f60582ec69e7b2"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetCrossReference::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetCrossReference", "path": "CommandGetCrossReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
