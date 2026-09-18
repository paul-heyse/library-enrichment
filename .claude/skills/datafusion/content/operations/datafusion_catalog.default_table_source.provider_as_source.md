# `datafusion_catalog::default_table_source::provider_as_source`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.default_table_source.provider_as_source.json).

<a id="op-59baf2201b4c1eb7841f7abe"></a>
## provider_as_source

`function` · `datafusion_catalog::default_table_source::provider_as_source` · datafusion-catalog 55.1.0

```rust
fn provider_as_source(table_provider: std::sync::Arc<dyn TableProvider>) -> std::sync::Arc<dyn TableSource>
```

Source: `src/default_table_source.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Wrap TableProvider in TableSource
