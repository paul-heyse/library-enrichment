# CatalogProviderList

`datafusion_session::catalog::CatalogProviderList`

```rust
trait CatalogProviderList: Any + Debug + Sync + Send
```

Also reachable as `datafusion_session::CatalogProviderList`

Prose: [`api/datafusion_session.catalog.md`](../api/datafusion_session.catalog.md#catalogproviderlist) · records: [`model/datafusion_session.catalog.json`](../model/datafusion_session.catalog.json)

## Required

Every implementation must supply these.

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

## Implementors (5)

Read one before writing your own.

- `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog`
- `datafusion_catalog::memory::catalog::MemoryCatalogProviderList`
- `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList`
- `datafusion_ffi::tests::catalog::FixedCatalogProviderList`
- `datafusion_session::catalog::EmptyCatalogProviderList`

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/data_io/catalog.rs`](../corpus/examples/data_io/catalog.rs)

## Documentation

Represent a list of named [`CatalogProvider`]s.

Please see the documentation on [`CatalogProvider`] for details of
implementing a custom catalog.
