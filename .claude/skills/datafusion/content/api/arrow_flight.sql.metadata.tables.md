# `arrow_flight::sql::metadata::tables`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.sql.metadata.tables.json`](../model/arrow_flight.sql.metadata.tables.json)

## GetTablesBuilder

`struct` · `arrow_flight::sql::metadata::tables::GetTablesBuilder`

Also reachable as `arrow_flight::sql::metadata::GetTablesBuilder`

```rust
struct GetTablesBuilder
```

**Implements**: `core::convert::From`

**Methods** (5)

```rust
fn append(&mut self, catalog_name: impl AsRef<str>, schema_name: impl AsRef<str>, table_name: impl AsRef<str>, table_type: impl AsRef<str>, table_schema: &Schema) -> Result<()>
fn build(self) -> Result<RecordBatch>
fn include_schema(&self) -> bool
fn new(catalog: Option<impl Into<String>>, db_schema_filter_pattern: Option<impl Into<String>>, table_name_filter_pattern: Option<impl Into<String>>, table_types: impl IntoIterator<Item = impl Into<String>>, include_schema: bool) -> Self
fn schema(&self) -> SchemaRef
```

**via `core::convert::From`**

```rust
fn from(value: CommandGetTables) -> Self
```

A builder for a [`CommandGetTables`] response.

Builds rows like this:

* catalog_name: utf8,
* db_schema_name: utf8,
* table_name: utf8 not null,
* table_type: utf8 not null,
* (optional) table_schema: bytes not null (schema of the table as described
  in Schema.fbs::Schema it is serialized as an IPC message.)

---
