# `datafusion_catalog::dynamic_file::catalog`

Crate `datafusion-catalog` · 3 public items · structured records in [`model/datafusion_catalog.dynamic_file.catalog.json`](../model/datafusion_catalog.dynamic_file.catalog.json)

## DynamicFileCatalog

`struct` · `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog`

```rust
struct DynamicFileCatalog
```

**Implements**: `datafusion_session::catalog::CatalogProviderList`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn CatalogProviderList>, factory: Arc<dyn UrlTableFactory>) -> Self
```

**via `datafusion_session::catalog::CatalogProviderList`**

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Wrap another catalog provider list

---

## DynamicFileSchemaProvider

`struct` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider`

```rust
struct DynamicFileSchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn SchemaProvider>, factory: Arc<dyn UrlTableFactory>) -> Self
```

**via `datafusion_session::schema::SchemaProvider`**

```rust
fn deregister_table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
async fn table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

Implements the [DynamicFileSchemaProvider] that can create tables provider from the file path.

The provider will try to create a table provider from the file path if the table provider
isn't exist in the inner schema provider.

---

## UrlTableFactory

`trait` · `datafusion_catalog::dynamic_file::catalog::UrlTableFactory`

```rust
trait UrlTableFactory: Debug + Sync + Send
```

**Implementors** (1)

- `datafusion::datasource::dynamic_file::DynamicListTableFactory`

**Methods** (1)

```rust
async fn try_new(&self, url: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

[UrlTableFactory] is a factory that can create a table provider from the given url.

---
