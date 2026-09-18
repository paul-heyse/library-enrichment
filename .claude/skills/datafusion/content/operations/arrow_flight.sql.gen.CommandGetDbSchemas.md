# `arrow_flight::sql::gen::CommandGetDbSchemas`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetDbSchemas.json).

<a id="op-77c2bf1bfd7f55c310b15d14"></a>
## CommandGetDbSchemas

`struct` · `arrow_flight::sql::gen::CommandGetDbSchemas` · arrow-flight 59.3.0

```rust
struct CommandGetDbSchemas
```

Source: `src/sql/arrow.flight.protocol.sql.rs:137`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve the list of database schemas on a Flight SQL enabled backend.
The definition of a database schema depends on vendor/implementation. It is usually a collection of tables.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8,
  db_schema_name: utf8 not null
>
The returned data should be ordered by catalog_name, then db_schema_name.

<a id="op-a3cd742a974f2a982843441b"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f0ae8241e36a98fc7f13d87"></a>
## catalog

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::catalog` · arrow-flight 59.3.0

```rust
fn catalog(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 38], "end": [136, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `catalog`, or the default value if `catalog` is unset.

<a id="op-f74524ef28370bd909195b9d"></a>
## catalog

`struct_field` · `arrow_flight::sql::gen::CommandGetDbSchemas::catalog` · arrow-flight 59.3.0

```rust
catalog: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the Catalog to search for the tables.
An empty string retrieves those without a catalog.
If omitted the catalog name should not be used to narrow the search.

<a id="op-6b8639b89a63a26e95199494"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 38], "end": [136, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ce62de396c4084cb6e6e3ee"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetDbSchemas
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 10], "end": [136, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-713f2608a093dc0799c3c34d"></a>
## db_schema_filter_pattern

`struct_field` · `arrow_flight::sql::gen::CommandGetDbSchemas::db_schema_filter_pattern` · arrow-flight 59.3.0

```rust
db_schema_filter_pattern: ::core::option::Option<::prost::alloc::string::String>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies a filter pattern for schemas to search for.
When no db_schema_filter_pattern is provided, the pattern will not be used to narrow the search.
In the pattern string, two special characters can be used to denote matching rules:
    - "%" means to match any substring with 0 or more characters.
    - "_" means to match any one character.

<a id="op-9255707c7575cb4ebd63b790"></a>
## db_schema_filter_pattern

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::db_schema_filter_pattern` · arrow-flight 59.3.0

```rust
fn db_schema_filter_pattern(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 38], "end": [136, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `db_schema_filter_pattern`, or the default value if `db_schema_filter_pattern` is unset.

<a id="op-495626c7f70103241fa9e482"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 38], "end": [136, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0319628200cd43838678561"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 38], "end": [136, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a9c2756b4ae62d5cea2b673"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetDbSchemas) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 17], "end": [136, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d5a2cde0a16bd4950be92f4"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 38], "end": [136, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f44335bf10fa1c0fd1fb5c9"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 32], "end": [136, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac2fa5f6ea669b7c24de74e3"></a>
## into_builder

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::into_builder` · arrow-flight 59.3.0

```rust
fn into_builder(self) -> GetDbSchemasBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "crate::sql::CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [60, 2], "filename": "src/sql/metadata/db_schemas.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/db_schemas.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a builder suitable for constructing a response

<a id="op-2c47183f2521d6b17d559a56"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetDbSchemas::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
