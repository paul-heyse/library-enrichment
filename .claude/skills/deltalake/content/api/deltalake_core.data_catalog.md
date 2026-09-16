# `deltalake_core::data_catalog`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.data_catalog.json`](../model/deltalake_core.data_catalog.json)

## DataCatalogError

`enum` · `deltalake_core::data_catalog::DataCatalogError`

Also reachable as `deltalake::DataCatalogError`, `deltalake::data_catalog::DataCatalogError`, `deltalake_core::DataCatalogError`

```rust
enum DataCatalogError
```

**Variants**: `Generic`, `InvalidDataCatalog`, `UnknownConfigKey`, `RequestError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Error enum that represents a CatalogError.

---

## DataCatalog

`trait` · `deltalake_core::data_catalog::DataCatalog`

Also reachable as `deltalake::DataCatalog`, `deltalake::data_catalog::DataCatalog`, `deltalake_core::DataCatalog`

```rust
trait DataCatalog: Send + Sync + Debug
```

**Implementors** (2)

- `deltalake_catalog_glue::GlueDataCatalog`
- `deltalake_catalog_unity::UnityCatalog`

**Methods** (1)

```rust
async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, Self::Error>
```

Abstractions for data catalog for the Delta table. To add support for new cloud, simply implement this trait.

---

## DataCatalogResult

`type_alias` · `deltalake_core::data_catalog::DataCatalogResult`

Also reachable as `deltalake::data_catalog::DataCatalogResult`

```rust
type DataCatalogResult<T> = Result<T, DataCatalogError>
```

A result type for data catalog implementations

---
