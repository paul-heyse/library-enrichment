# `arrow_flight::sql::metadata::db_schemas`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.sql.metadata.db_schemas.json`](../model/arrow_flight.sql.metadata.db_schemas.json)

## GetDbSchemasBuilder

`struct` · `arrow_flight::sql::metadata::db_schemas::GetDbSchemasBuilder`

Also reachable as `arrow_flight::sql::metadata::GetDbSchemasBuilder`

```rust
struct GetDbSchemasBuilder
```

**Implements**: `core::convert::From`

**Methods** (4)

```rust
fn append(&mut self, catalog_name: impl AsRef<str>, schema_name: impl AsRef<str>)
fn build(self) -> Result<RecordBatch>
fn new(catalog: Option<impl Into<String>>, db_schema_filter_pattern: Option<impl Into<String>>) -> Self
fn schema(&self) -> SchemaRef
```

**via `core::convert::From`**

```rust
fn from(value: CommandGetDbSchemas) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.metadata.db_schemas.GetDbSchemasBuilder.md).


A builder for a [`CommandGetDbSchemas`] response.

Builds rows like this:

* catalog_name: utf8,
* db_schema_name: utf8 not null

---
