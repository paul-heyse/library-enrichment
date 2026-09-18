# `datafusion::datasource::listing_table_factory`

Crate `datafusion` · 1 public items · structured records in [`model/datafusion.datasource.listing_table_factory.json`](../model/datafusion.datasource.listing_table_factory.json)

## ListingTableFactory

`struct` · `datafusion::datasource::listing_table_factory::ListingTableFactory`

```rust
struct ListingTableFactory
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

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.listing_table_factory.ListingTableFactory.md).


A `TableProviderFactory` capable of creating new `ListingTable`s

---
