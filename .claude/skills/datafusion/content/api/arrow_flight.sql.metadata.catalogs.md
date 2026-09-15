# `arrow_flight::sql::metadata::catalogs`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.sql.metadata.catalogs.json`](../model/arrow_flight.sql.metadata.catalogs.json)

## GetCatalogsBuilder

`struct` · `arrow_flight::sql::metadata::catalogs::GetCatalogsBuilder`

Also reachable as `arrow_flight::sql::metadata::GetCatalogsBuilder`

```rust
struct GetCatalogsBuilder
```

**Implements**: `core::convert::From`

**Derives**: Default

**Methods** (4)

```rust
fn append(&mut self, catalog_name: impl Into<String>)
fn build(self) -> Result<RecordBatch>
fn new() -> Self
fn schema(&self) -> SchemaRef
```

**via `core::convert::From`**

```rust
fn from(_: CommandGetCatalogs) -> Self
```

A builder for a [`CommandGetCatalogs`] response.

Builds rows like this:

* catalog_name: utf8,

---
