# `deltalake_core::data_catalog::DataCatalogResult`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.data_catalog.DataCatalogResult.json).

<a id="op-c5cc9880fd7e19629d6bd213"></a>
## DataCatalogResult

`type_alias` · `deltalake_core::data_catalog::DataCatalogResult` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DataCatalogResult<T> = Result<T, DataCatalogError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L9).

Source: `crates/core/src/data_catalog/mod.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A result type for data catalog implementations
