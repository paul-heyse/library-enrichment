# `datafusion::datasource::provider`

Crate `datafusion` · 1 public items · structured records in [`model/datafusion.datasource.provider.json`](../model/datafusion.datasource.provider.json)

## DefaultTableFactory

`struct` · `datafusion::datasource::provider::DefaultTableFactory`

```rust
struct DefaultTableFactory
```

**Implements**: `datafusion_session::table::TableProviderFactory`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::table::TableProviderFactory`**

```rust
async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

The default [`TableProviderFactory`]

If [`CreateExternalTable`] is unbounded calls [`StreamTableFactory::create`],
otherwise calls [`ListingTableFactory::create`]

---
