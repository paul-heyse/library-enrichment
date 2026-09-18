# `datafusion_catalog::default_table_source::source_as_provider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.default_table_source.source_as_provider.json).

<a id="op-e0d1974a5220bc06709d341f"></a>
## source_as_provider

`function` · `datafusion_catalog::default_table_source::source_as_provider` · datafusion-catalog 55.1.0

```rust
fn source_as_provider(source: &std::sync::Arc<dyn TableSource>) -> datafusion_common::Result<std::sync::Arc<dyn TableProvider>>
```

Source: `src/default_table_source.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Attempt to downcast a TableSource to DefaultTableSource and access the
TableProvider. This will only work with a TableSource created by DataFusion.
