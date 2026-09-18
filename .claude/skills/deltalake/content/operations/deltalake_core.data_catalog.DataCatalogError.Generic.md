# `deltalake_core::data_catalog::DataCatalogError::Generic`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.data_catalog.DataCatalogError.Generic.json).

<a id="op-1e1354451d07d92b487e49b9"></a>
## catalog

`struct_field` · `deltalake_core::data_catalog::DataCatalogError::Generic::catalog` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
catalog: &'static str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L18).

Source: `crates/core/src/data_catalog/mod.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of the catalog

<a id="op-2ca0b1f0e77bd94dd3ea16ab"></a>
## source

`struct_field` · `deltalake_core::data_catalog::DataCatalogError::Generic::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L20).

Source: `crates/core/src/data_catalog/mod.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error message
