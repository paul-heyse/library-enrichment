# `deltalake_core::data_catalog::DataCatalogError::RequestError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.data_catalog.DataCatalogError.RequestError.json).

<a id="op-5e929d188b1d58f4e06c6ef6"></a>
## source

`struct_field` · `deltalake_core::data_catalog::DataCatalogError::RequestError::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L44).

Source: `crates/core/src/data_catalog/mod.rs:44`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The underlying transport or service error that caused the request to fail.
