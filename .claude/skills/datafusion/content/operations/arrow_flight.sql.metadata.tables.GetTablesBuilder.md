# `arrow_flight::sql::metadata::tables::GetTablesBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.tables.GetTablesBuilder.json).

<a id="op-a87af5379ec74ef613b86dff"></a>
## GetTablesBuilder

`struct` · `arrow_flight::sql::metadata::tables::GetTablesBuilder` · arrow-flight 59.3.0

```rust
struct GetTablesBuilder
```

Source: `src/sql/metadata/tables.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A builder for a [`CommandGetTables`](../operations/arrow_flight.sql.gen.CommandGetTables.md#op-b356eca1c40464b8a7ecea0a) response.

Builds rows like this:

* catalog_name: utf8,
* db_schema_name: utf8,
* table_name: utf8 not null,
* table_type: utf8 not null,
* (optional) table_schema: bytes not null (schema of the table as described
  in Schema.fbs::Schema it is serialized as an IPC message.)

<a id="op-e0203f7c4da8a114f07fcc1d"></a>
## append

`function` · `arrow_flight::sql::metadata::tables::GetTablesBuilder::append` · arrow-flight 59.3.0

```rust
fn append(&mut self, catalog_name: impl AsRef<str>, schema_name: impl AsRef<str>, table_name: impl AsRef<str>, table_type: impl AsRef<str>, table_schema: &Schema) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::tables::GetTablesBuilder", "path": "GetTablesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [281, 2], "filename": "src/sql/metadata/tables.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/tables.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Append a row

<a id="op-a184032cf6bf0fb6962fd1bf"></a>
## build

`function` · `arrow_flight::sql::metadata::tables::GetTablesBuilder::build` · arrow-flight 59.3.0

```rust
fn build(self) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::tables::GetTablesBuilder", "path": "GetTablesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [281, 2], "filename": "src/sql/metadata/tables.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/tables.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

builds a `RecordBatch` for `CommandGetTables`

<a id="op-2b0d200443f9ee710e078518"></a>
## from

`function` · `arrow_flight::sql::metadata::tables::GetTablesBuilder::from` · arrow-flight 59.3.0

```rust
fn from(value: CommandGetTables) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::tables::GetTablesBuilder", "path": "GetTablesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [84, 2], "filename": "src/sql/metadata/tables.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetTables", "path": "CommandGetTables"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sql/metadata/tables.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-498ebf82dfa9a068de1dc9ff"></a>
## include_schema

`function` · `arrow_flight::sql::metadata::tables::GetTablesBuilder::include_schema` · arrow-flight 59.3.0

```rust
fn include_schema(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::tables::GetTablesBuilder", "path": "GetTablesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [281, 2], "filename": "src/sql/metadata/tables.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/tables.rs:278`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Should the "schema" column be included

<a id="op-8f1fc20959e0e4053ddd56ce"></a>
## new

`function` · `arrow_flight::sql::metadata::tables::GetTablesBuilder::new` · arrow-flight 59.3.0

```rust
fn new(catalog: Option<impl Into<String>>, db_schema_filter_pattern: Option<impl Into<String>>, table_name_filter_pattern: Option<impl Into<String>>, table_types: impl IntoIterator<Item = impl Into<String>>, include_schema: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::tables::GetTablesBuilder", "path": "GetTablesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [281, 2], "filename": "src/sql/metadata/tables.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/tables.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new instance of [`GetTablesBuilder`](../operations/arrow_flight.sql.metadata.tables.GetTablesBuilder.md#op-a87af5379ec74ef613b86dff)

# Parameters

- `catalog`:  Specifies the Catalog to search for the tables.
  - An empty string retrieves those without a catalog.
  - If omitted the catalog name is not used to narrow the search.
- `db_schema_filter_pattern`: Specifies a filter pattern for schemas to search for.
  When no pattern is provided, the pattern will not be used to narrow the search.
  In the pattern string, two special characters can be used to denote matching rules:
    - "%" means to match any substring with 0 or more characters.
    - "_" means to match any one character.
- `table_name_filter_pattern`: Specifies a filter pattern for tables to search for.
  When no pattern is provided, all tables matching other filters are searched.
  In the pattern string, two special characters can be used to denote matching rules:
    - "%" means to match any substring with 0 or more characters.
    - "_" means to match any one character.
- `table_types`:  Specifies a filter of table types which must match.
  An empy Vec matches all table types.
- `include_schema`: Specifies if the Arrow schema should be returned for found tables.

[`CommandGetTables`]: crate::sql::CommandGetTables

<a id="op-70e7552c54de9fa0ad118a96"></a>
## schema

`function` · `arrow_flight::sql::metadata::tables::GetTablesBuilder::schema` · arrow-flight 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::tables::GetTablesBuilder", "path": "GetTablesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [281, 2], "filename": "src/sql/metadata/tables.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/tables.rs:273`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the schema of the RecordBatch that will be returned from [`CommandGetTables`]

Note the schema differs based on the values of `include_schema

[`CommandGetTables`]: crate::sql::CommandGetTables
