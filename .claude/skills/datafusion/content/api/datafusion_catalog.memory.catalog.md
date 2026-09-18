# `datafusion_catalog::memory::catalog`

Crate `datafusion-catalog` · 2 public items · structured records in [`model/datafusion_catalog.memory.catalog.json`](../model/datafusion_catalog.memory.catalog.json)

## MemoryCatalogProvider

`struct` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider`

Also reachable as `datafusion::catalog::MemoryCatalogProvider`, `datafusion_catalog::MemoryCatalogProvider`

```rust
struct MemoryCatalogProvider
```

**Implements**: `datafusion_session::catalog::CatalogProvider`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::catalog::CatalogProvider`**

```rust
fn deregister_schema(&self, name: &str, cascade: bool) -> datafusion_common::Result<Option<Arc<dyn SchemaProvider>>>
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> datafusion_common::Result<Option<Arc<dyn SchemaProvider>>>
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
fn schema_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.memory.catalog.MemoryCatalogProvider.md).


Simple in-memory implementation of a catalog.

---

## MemoryCatalogProviderList

`struct` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList`

Also reachable as `datafusion::catalog::MemoryCatalogProviderList`, `datafusion_catalog::MemoryCatalogProviderList`

```rust
struct MemoryCatalogProviderList
```

**Fields**: `catalogs`

**Implements**: `datafusion_session::catalog::CatalogProviderList`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::catalog::CatalogProviderList`**

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.memory.catalog.MemoryCatalogProviderList.md).


Simple in-memory list of catalogs

---
