# `datafusion_catalog`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.json).

<a id="op-e3265a7460be841db22db9cd"></a>
## datafusion_catalog

`module` · `datafusion_catalog` · datafusion-catalog 55.1.0

```rust
mod datafusion_catalog
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Default implementations of catalogs and schemas.

The catalog interfaces are defined in [`datafusion_session`](../modules/datafusion_session.md#op-53ba55e8326daf52cce4b573) and re-exported
by this crate.

Implementations
* Information schema: [`information_schema`](../modules/datafusion_catalog.information_schema.md#op-70cdd2da3c6a33d1c244ed3c)
* Simple memory based catalog: [`MemoryCatalogProviderList`](../operations/datafusion_catalog.memory.catalog.MemoryCatalogProviderList.md#op-937d01547a7438b1ed33a7d0), [`MemoryCatalogProvider`](../operations/datafusion_catalog.memory.catalog.MemoryCatalogProvider.md#op-faa1b922b564ab8766a89660), [`MemorySchemaProvider`](../operations/datafusion_catalog.memory.schema.MemorySchemaProvider.md#op-faa734bbbe1c631477491cd1)
* Listing schema: [`listing_schema`](../modules/datafusion_catalog.listing_schema.md#op-782144beae3248d3e261a2e0)
