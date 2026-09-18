# `datafusion_session::schema`

Crate `datafusion-session` · 1 public items · structured records in [`model/datafusion_session.schema.json`](../model/datafusion_session.schema.json)

## SchemaProvider

`trait` · `datafusion_session::schema::SchemaProvider`

Also reachable as `datafusion_session::SchemaProvider`, `datafusion_session::catalog::SchemaProvider`

```rust
trait SchemaProvider: Any + Debug + Sync + Send
```

**Implementors** (6)

- `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider`
- `datafusion_catalog::information_schema::InformationSchemaProvider`
- `datafusion_catalog::listing_schema::ListingSchemaProvider`
- `datafusion_catalog::memory::schema::MemorySchemaProvider`
- `datafusion_ffi::schema_provider::ForeignSchemaProvider`
- `datafusion_ffi::tests::catalog::FixedSchemaProvider`

**Methods** (7)

```rust
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
fn owner_name(&self) -> Option<&str>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
async fn table_type(&self, name: &str) -> Result<Option<TableType>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.schema.SchemaProvider.md).


Represents a schema, comprising a number of named tables.

Please see [`CatalogProvider`] for details of implementing a custom catalog.

[`CatalogProvider`]: super::CatalogProvider

---
