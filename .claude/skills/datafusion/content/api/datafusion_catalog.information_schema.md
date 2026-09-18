# `datafusion_catalog::information_schema`

Crate `datafusion-catalog` · 5 public items · structured records in [`model/datafusion_catalog.information_schema.json`](../model/datafusion_catalog.information_schema.json)

## INFORMATION_SCHEMA

`constant` · `datafusion_catalog::information_schema::INFORMATION_SCHEMA`

```rust
const INFORMATION_SCHEMA: &str = "information_schema"
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.information_schema.INFORMATION_SCHEMA.md).


---

## INFORMATION_SCHEMA_TABLES

`constant` · `datafusion_catalog::information_schema::INFORMATION_SCHEMA_TABLES`

```rust
const INFORMATION_SCHEMA_TABLES: &[&str] = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.information_schema.INFORMATION_SCHEMA_TABLES.md).


All information schema tables

---

## schemata_schema

`function` · `datafusion_catalog::information_schema::schemata_schema`

```rust
fn schemata_schema() -> arrow::datatypes::SchemaRef
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.information_schema.schemata_schema.md).


The Arrow schema of [`information_schema.schemata`] rows.

Useful for downstream catalog implementations that want to declare a
`TableProvider` for `schemata` before populating any rows via
[`InformationSchemataBuilder`].

Columns and nullability match
<https://www.postgresql.org/docs/current/infoschema-schemata.html>.

[`information_schema.schemata`]: https://www.postgresql.org/docs/current/infoschema-schemata.html

---

## InformationSchemaProvider

`struct` · `datafusion_catalog::information_schema::InformationSchemaProvider`

```rust
struct InformationSchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug

**Methods** (2)

```rust
fn new(catalog_list: Arc<dyn CatalogProviderList>) -> Self
fn with_table_functions(self, table_functions: HashMap<String, Arc<TableFunction>>) -> Self
```

**via `datafusion_session::schema::SchemaProvider`**

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.information_schema.InformationSchemaProvider.md).


Implements the `information_schema` virtual schema and tables

The underlying tables in the `information_schema` are created on
demand. This means that if more tables are added to the underlying
providers, they will appear the next time the `information_schema`
table is queried.

---

## InformationSchemataBuilder

`struct` · `datafusion_catalog::information_schema::InformationSchemataBuilder`

```rust
struct InformationSchemataBuilder
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn add_schemata(&mut self, catalog_name: &str, schema_name: &str, schema_owner: Option<&str>)
fn finish(&mut self) -> Result<RecordBatch>
fn new() -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.information_schema.InformationSchemataBuilder.md).


Builder that produces [`RecordBatch`] values matching the schema of
`information_schema.schemata` (see [`schemata_schema`]).

Intended for downstream catalog implementations that need to emit
`schemata` rows from their own metadata source rather than going
through DataFusion's `InformationSchemaProvider`, which enumerates
schemas synchronously via `CatalogProviderList` and so is unsuitable
for catalog backends that resolve asynchronously.

---
