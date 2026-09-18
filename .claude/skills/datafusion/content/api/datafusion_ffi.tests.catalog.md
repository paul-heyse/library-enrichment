# `datafusion_ffi::tests::catalog`

Crate `datafusion-ffi` · 4 public items · structured records in [`model/datafusion_ffi.tests.catalog.json`](../model/datafusion_ffi.tests.catalog.json)

## fruit_table

`function` · `datafusion_ffi::tests::catalog::fruit_table`

```rust
fn fruit_table() -> std::sync::Arc<dyn TableProvider + 'static>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.catalog.fruit_table.md).


---

## FixedCatalogProvider

`struct` · `datafusion_ffi::tests::catalog::FixedCatalogProvider`

```rust
struct FixedCatalogProvider
```

**Implements**: `datafusion_session::catalog::CatalogProvider`

**Derives**: Debug, Default

**via `datafusion_session::catalog::CatalogProvider`**

```rust
fn deregister_schema(&self, name: &str, cascade: bool) -> Result<Option<Arc<dyn SchemaProvider>>>
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> Result<Option<Arc<dyn SchemaProvider>>>
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
fn schema_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.catalog.FixedCatalogProvider.md).


This catalog provider is intended only for unit tests. It prepopulates with one
schema and only allows for schemas named after four types of fruit.

---

## FixedCatalogProviderList

`struct` · `datafusion_ffi::tests::catalog::FixedCatalogProviderList`

```rust
struct FixedCatalogProviderList
```

**Implements**: `datafusion_session::catalog::CatalogProviderList`

**Derives**: Debug, Default

**via `datafusion_session::catalog::CatalogProviderList`**

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.catalog.FixedCatalogProviderList.md).


This catalog provider list is intended only for unit tests. It prepopulates with one
catalog and only allows for catalogs named after four colors.

---

## FixedSchemaProvider

`struct` · `datafusion_ffi::tests::catalog::FixedSchemaProvider`

```rust
struct FixedSchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug, Default

**via `datafusion_session::schema::SchemaProvider`**

```rust
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.catalog.FixedSchemaProvider.md).


This schema provider is intended only for unit tests. It prepopulates with one
table and only allows for tables named sales and purchases.

---
