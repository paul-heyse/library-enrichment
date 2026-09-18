# `deltalake_core::data_catalog::DataCatalog`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.data_catalog.DataCatalog.json).

<a id="op-aff7d6ddc900d6c1b6ce15cd"></a>
## DataCatalog

`trait` · `deltalake_core::data_catalog::DataCatalog` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DataCatalog: Send + Sync + Debug
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L50).

Source: `crates/core/src/data_catalog/mod.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Abstractions for data catalog for the Delta table. To add support for new cloud, simply implement this trait.

<a id="op-44ca26a37d8b67de8b945038"></a>
## Error

`assoc_type` · `deltalake_core::data_catalog::DataCatalog::Error` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L52).

Source: `crates/core/src/data_catalog/mod.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error type returned by catalog operations.

<a id="op-4f3ed3f9aa68017704ebfb8b"></a>
## get_table_storage_location

`function` · `deltalake_core::data_catalog::DataCatalog::get_table_storage_location` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, Self::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L55).

Source: `crates/core/src/data_catalog/mod.rs:55`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table storage location from the Data Catalog
