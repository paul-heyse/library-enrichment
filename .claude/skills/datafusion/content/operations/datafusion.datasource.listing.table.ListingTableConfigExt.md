# `datafusion::datasource::listing::table::ListingTableConfigExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.listing.table.ListingTableConfigExt.json).

<a id="op-4b90c6459684f9a10392f8ed"></a>
## ListingTableConfigExt

`trait` · `datafusion::datasource::listing::table::ListingTableConfigExt` · datafusion 55.1.0

```rust
trait ListingTableConfigExt
```

Source: `src/datasource/listing/table.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Extension trait for [`ListingTableConfig`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-4acbb7e9df4c811e49fabab5) that supports inferring schemas

This trait exists because the following inference methods only
work for [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) implementations of [`Session`](../operations/datafusion_session.session.Session.md#op-75302dfa669885e17a5c9093).
See [`ListingTableConfig`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-4acbb7e9df4c811e49fabab5) for the remaining inference methods.

<a id="op-47c4944fa5fc6a1bb81e0e0b"></a>
## infer

`function` · `datafusion::datasource::listing::table::ListingTableConfigExt::infer` · datafusion 55.1.0

```rust
async fn infer(self, state: &dyn Session) -> datafusion_common::Result<ListingTableConfig>
```

Source: `src/datasource/listing/table.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Convenience method to call both [`Self::infer_options`](../operations/datafusion.datasource.listing.table.ListingTableConfigExt.md#op-0dcd5ef93667e9e367dceccf) and [`ListingTableConfig::infer_schema`]

Unresolved upstream links (retained, not inferred): ``ListingTableConfig::infer_schema``.

<a id="op-0dcd5ef93667e9e367dceccf"></a>
## infer_options

`function` · `datafusion::datasource::listing::table::ListingTableConfigExt::infer_options` · datafusion 55.1.0

```rust
async fn infer_options(self, state: &dyn Session) -> datafusion_common::Result<ListingTableConfig>
```

Source: `src/datasource/listing/table.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Infer `ListingOptions` based on `table_path` and file suffix.

The format is inferred based on the first `table_path`.
