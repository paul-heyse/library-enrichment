# `datafusion::datasource::listing::table`

Crate `datafusion` · 1 public items · structured records in [`model/datafusion.datasource.listing.table.json`](../model/datafusion.datasource.listing.table.json)

## ListingTableConfigExt

`trait` · `datafusion::datasource::listing::table::ListingTableConfigExt`

Also reachable as `datafusion::datasource::listing::ListingTableConfigExt`

```rust
trait ListingTableConfigExt
```

**Implementors** (1)

- `datafusion_catalog_listing::config::ListingTableConfig`

**Methods** (2)

```rust
async fn infer(self, state: &dyn Session) -> datafusion_common::Result<ListingTableConfig>
async fn infer_options(self, state: &dyn Session) -> datafusion_common::Result<ListingTableConfig>
```

Extension trait for [`ListingTableConfig`] that supports inferring schemas

This trait exists because the following inference methods only
work for [`SessionState`] implementations of [`Session`].
See [`ListingTableConfig`] for the remaining inference methods.

---
