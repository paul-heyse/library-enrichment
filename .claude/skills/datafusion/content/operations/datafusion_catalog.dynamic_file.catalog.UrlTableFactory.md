# `datafusion_catalog::dynamic_file::catalog::UrlTableFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.dynamic_file.catalog.UrlTableFactory.json).

<a id="op-f86a61dc7e8f7beddfbc8d27"></a>
## UrlTableFactory

`trait` · `datafusion_catalog::dynamic_file::catalog::UrlTableFactory` · datafusion-catalog 55.1.0

```rust
trait UrlTableFactory: Debug + Sync + Send
```

Source: `src/dynamic_file/catalog.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

[UrlTableFactory](../operations/datafusion_catalog.dynamic_file.catalog.UrlTableFactory.md#op-f86a61dc7e8f7beddfbc8d27) is a factory that can create a table provider from the given url.

<a id="op-f443095c38c18a0d0c116d3b"></a>
## try_new

`function` · `datafusion_catalog::dynamic_file::catalog::UrlTableFactory::try_new` · datafusion-catalog 55.1.0

```rust
async fn try_new(&self, url: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Source: `src/dynamic_file/catalog.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

create a new table provider from the provided url
