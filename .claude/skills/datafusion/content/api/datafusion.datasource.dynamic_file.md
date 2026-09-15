# `datafusion::datasource::dynamic_file`

Crate `datafusion` · 1 public items · structured records in [`model/datafusion.datasource.dynamic_file.json`](../model/datafusion.datasource.dynamic_file.json)

## DynamicListTableFactory

`struct` · `datafusion::datasource::dynamic_file::DynamicListTableFactory`

```rust
struct DynamicListTableFactory
```

**Implements**: `datafusion_catalog::dynamic_file::catalog::UrlTableFactory`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new(session_store: SessionStore) -> Self
fn session_store(&self) -> &SessionStore
```

**via `datafusion_catalog::dynamic_file::catalog::UrlTableFactory`**

```rust
async fn try_new(&self, url: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

[DynamicListTableFactory] is a factory that can create a [ListingTable] from the given url.

---
