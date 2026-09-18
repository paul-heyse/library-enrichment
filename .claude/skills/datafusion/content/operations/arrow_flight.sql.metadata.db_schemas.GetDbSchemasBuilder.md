# `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.db_schemas.GetDbSchemasBuilder.json).

<a id="op-8ce7622b04ed745b3def9de8"></a>
## GetDbSchemasBuilder

`struct` · `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder` · arrow-flight 59.3.0

```rust
struct GetDbSchemasBuilder
```

Source: `src/sql/metadata/db_schemas.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A builder for a [`CommandGetDbSchemas`](../operations/arrow_flight.sql.gen.CommandGetDbSchemas.md#op-77c2bf1bfd7f55c310b15d14) response.

Builds rows like this:

* catalog_name: utf8,
* db_schema_name: utf8 not null

<a id="op-b3626a3fc50d8bddc1402980"></a>
## append

`function` · `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder::append` · arrow-flight 59.3.0

```rust
fn append(&mut self, catalog_name: impl AsRef<str>, schema_name: impl AsRef<str>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder", "path": "GetDbSchemasBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [171, 2], "filename": "src/sql/metadata/db_schemas.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/db_schemas.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Append a row

In case the catalog should be considered as empty, pass in an empty string '""'.

<a id="op-6cd662982e162b4fb44e879d"></a>
## build

`function` · `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder::build` · arrow-flight 59.3.0

```rust
fn build(self) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder", "path": "GetDbSchemasBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [171, 2], "filename": "src/sql/metadata/db_schemas.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/db_schemas.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

builds a `RecordBatch` with the correct schema for a `CommandGetDbSchemas` response

<a id="op-7865bc19d099f6d3d39a24e1"></a>
## from

`function` · `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder::from` · arrow-flight 59.3.0

```rust
fn from(value: CommandGetDbSchemas) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder", "path": "GetDbSchemasBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [66, 2], "filename": "src/sql/metadata/db_schemas.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetDbSchemas", "path": "CommandGetDbSchemas"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sql/metadata/db_schemas.rs:63`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec4c9bd23ca6820ad996f413"></a>
## new

`function` · `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder::new` · arrow-flight 59.3.0

```rust
fn new(catalog: Option<impl Into<String>>, db_schema_filter_pattern: Option<impl Into<String>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder", "path": "GetDbSchemasBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [171, 2], "filename": "src/sql/metadata/db_schemas.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/db_schemas.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new instance of [`GetDbSchemasBuilder`](../operations/arrow_flight.sql.metadata.db_schemas.GetDbSchemasBuilder.md#op-8ce7622b04ed745b3def9de8)

# Parameters

- `catalog`:  Specifies the Catalog to search for the tables.
  - An empty string retrieves those without a catalog.
  - If omitted the catalog name is not used to narrow the search.
- `db_schema_filter_pattern`: Specifies a filter pattern for schemas to search for.
  When no pattern is provided, the pattern will not be used to narrow the search.
  In the pattern string, two special characters can be used to denote matching rules:
    - "%" means to match any substring with 0 or more characters.
    - "_" means to match any one character.

[`CommandGetDbSchemas`]: crate::sql::CommandGetDbSchemas

<a id="op-359a90c6375c4150d7a355e5"></a>
## schema

`function` · `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder", "path": "GetDbSchemasBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [171, 2], "filename": "src/sql/metadata/db_schemas.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/db_schemas.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the schema of the RecordBatch that will be returned
from [`CommandGetDbSchemas`](../operations/arrow_flight.sql.gen.CommandGetDbSchemas.md#op-77c2bf1bfd7f55c310b15d14)
