# `deltalake_catalog_unity::datafusion`

Crate `deltalake-catalog-unity` · 3 public items · structured records in [`model/deltalake_catalog_unity.datafusion.json`](../model/deltalake_catalog_unity.datafusion.json)

## UnityCatalogList

`struct` · `deltalake_catalog_unity::datafusion::UnityCatalogList`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.datafusion.UnityCatalogList.md)

Also reachable as `deltalake_catalog_unity::prelude::UnityCatalogList`

```rust
struct UnityCatalogList
```

**Fields**: `catalogs`

**Implements**: `datafusion_session::catalog::CatalogProviderList`

**Derives**: Debug

**Methods** (1)

```rust
async fn try_new(client: Arc<UnityCatalog>) -> DataCatalogResult<Self>
```

**via `datafusion_session::catalog::CatalogProviderList`**

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

In-memory list of catalogs populated by unity catalog

---

## UnityCatalogProvider

`struct` · `deltalake_catalog_unity::datafusion::UnityCatalogProvider`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.datafusion.UnityCatalogProvider.md)

Also reachable as `deltalake_catalog_unity::prelude::UnityCatalogProvider`

```rust
struct UnityCatalogProvider
```

**Fields**: `schemas`

**Implements**: `datafusion_session::catalog::CatalogProvider`

**Derives**: Debug

**Methods** (1)

```rust
async fn try_new(client: Arc<UnityCatalog>, catalog_name: impl Into<String>) -> DataCatalogResult<Self>
```

**via `datafusion_session::catalog::CatalogProvider`**

```rust
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
fn schema_names(&self) -> Vec<String>
```

A datafusion [`CatalogProvider`] backed by Databricks UnityCatalog

---

## UnitySchemaProvider

`struct` · `deltalake_catalog_unity::datafusion::UnitySchemaProvider`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_unity.datafusion.UnitySchemaProvider.md)

Also reachable as `deltalake_catalog_unity::prelude::UnitySchemaProvider`

```rust
struct UnitySchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug

**Methods** (1)

```rust
async fn try_new(client: Arc<UnityCatalog>, catalog_name: impl Into<String>, schema_name: impl Into<String>) -> DataCatalogResult<Self>
```

**via `datafusion_session::schema::SchemaProvider`**

```rust
async fn table(&self, name: &str) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

A datafusion [`SchemaProvider`] backed by Databricks UnityCatalog

---
