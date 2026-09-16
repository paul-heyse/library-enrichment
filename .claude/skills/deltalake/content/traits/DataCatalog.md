# DataCatalog

`deltalake_core::data_catalog::DataCatalog`

```rust
trait DataCatalog: Send + Sync + Debug
```

Also reachable as `deltalake::DataCatalog`, `deltalake::data_catalog::DataCatalog`, `deltalake_core::DataCatalog`

Prose: [`api/deltalake_core.data_catalog.md`](../api/deltalake_core.data_catalog.md#datacatalog) · records: [`model/deltalake_core.data_catalog.json`](../model/deltalake_core.data_catalog.json)

## Required

Every implementation must supply these.

```rust
async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, Self::Error>
```

## Implementors (2)

Read one before writing your own.

- `deltalake_catalog_glue::GlueDataCatalog`
- `deltalake_catalog_unity::UnityCatalog`

## Documentation

Abstractions for data catalog for the Delta table. To add support for new cloud, simply implement this trait.
