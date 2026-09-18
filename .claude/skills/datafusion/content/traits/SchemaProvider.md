# SchemaProvider

`datafusion_session::schema::SchemaProvider`

```rust
trait SchemaProvider: Any + Debug + Sync + Send
```

Also reachable as `datafusion_session::SchemaProvider`, `datafusion_session::catalog::SchemaProvider`

Prose: [`api/datafusion_session.schema.md`](../api/datafusion_session.schema.md#schemaprovider) · records: [`model/datafusion_session.schema.json`](../model/datafusion_session.schema.json)

## Required

Every implementation must supply these.

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
fn owner_name(&self) -> Option<&str>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
async fn table_type(&self, name: &str) -> Result<Option<TableType>>
```

## Implementors (6)

Read one before writing your own.

- `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider`
- `datafusion_catalog::information_schema::InformationSchemaProvider`
- `datafusion_catalog::listing_schema::ListingSchemaProvider`
- `datafusion_catalog::memory::schema::MemorySchemaProvider`
- `datafusion_ffi::schema_provider::ForeignSchemaProvider`
- `datafusion_ffi::tests::catalog::FixedSchemaProvider`

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/data_io/catalog.rs`](../corpus/examples/data_io/catalog.rs)

## Documentation

Represents a schema, comprising a number of named tables.

Please see [`CatalogProvider`] for details of implementing a custom catalog.

[`CatalogProvider`]: super::CatalogProvider
