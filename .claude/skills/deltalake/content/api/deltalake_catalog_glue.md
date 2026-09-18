# `deltalake_catalog_glue`

Crate `deltalake-catalog-glue` · 2 public items · structured records in [`model/deltalake_catalog_glue.json`](../model/deltalake_catalog_glue.json)

## GlueError

`enum` · `deltalake_catalog_glue::GlueError`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_glue.GlueError.md)

```rust
enum GlueError
```

**Variants**: `MissingMetadata`, `AWSError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: aws_sdk_glue::Error) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## GlueDataCatalog

`struct` · `deltalake_catalog_glue::GlueDataCatalog`
[Full member contracts, output types and access classification](../operations/deltalake_catalog_glue.GlueDataCatalog.md)

```rust
struct GlueDataCatalog
```

**Implements**: `deltalake_core::data_catalog::DataCatalog`

**Derives**: Debug

**Methods** (2)

```rust
async fn from_env() -> Result<Self, GlueError>
fn with_config(config: &SdkConfig) -> Self
```

**via `deltalake_core::data_catalog::DataCatalog`**

```rust
async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, DataCatalogError>
```

A Glue Data Catalog implement of the `Catalog` trait

---
