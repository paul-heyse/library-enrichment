# `datafusion_catalog::async::AsyncCatalogProviderList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.async.AsyncCatalogProviderList.json).

<a id="op-a116c22687950f0deda14ad3"></a>
## AsyncCatalogProviderList

`trait` · `datafusion_catalog::async::AsyncCatalogProviderList` · datafusion-catalog 55.1.0

```rust
trait AsyncCatalogProviderList: Send + Sync
```

Source: `src/async.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

A trait for catalog provider lists that must resolve catalogs asynchronously

The [`CatalogProviderList::catalog`](../operations/datafusion_session.catalog.CatalogProviderList.md#op-3923334c75215a66b2d36a73) method is synchronous because asynchronous operations should
not be used during planning.  This trait makes it easy to lookup catalog references once and cache
them for future planning use.  See [`AsyncSchemaProvider`](../operations/datafusion_catalog.async.AsyncSchemaProvider.md#op-02f401b09b70a7408425eecf) for more details on motivation.

<a id="op-68f6544196bd1d06dbe32d94"></a>
## catalog

`function` · `datafusion_catalog::async::AsyncCatalogProviderList::catalog` · datafusion-catalog 55.1.0

```rust
async fn catalog(&self, name: &str) -> Result<Option<Arc<dyn AsyncCatalogProvider>>>
```

Source: `src/async.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Lookup a catalog in the provider

<a id="op-15ab23e9241601858c2b06de"></a>
## resolve

`function` · `datafusion_catalog::async::AsyncCatalogProviderList::resolve` · datafusion-catalog 55.1.0

```rust
async fn resolve(&self, references: &[TableReference], config: &SessionConfig) -> Result<Arc<dyn CatalogProviderList>>
```

Source: `src/async.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of catalog
providers, schema providers, and table providers.  This cache will be returned as a
synchronous CatalogProvider that can be used to plan and execute a query containing the given
references.

This cache is intended to be short-lived for the execution of a single query.  There is no mechanism
for refresh or eviction of stale entries.
