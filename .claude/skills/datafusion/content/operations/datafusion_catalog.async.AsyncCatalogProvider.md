# `datafusion_catalog::async::AsyncCatalogProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.async.AsyncCatalogProvider.json).

<a id="op-27d8089bf0f83ec92e94ce09"></a>
## AsyncCatalogProvider

`trait` · `datafusion_catalog::async::AsyncCatalogProvider` · datafusion-catalog 55.1.0

```rust
trait AsyncCatalogProvider: Send + Sync
```

Source: `src/async.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

A trait for catalog providers that must resolve schemas asynchronously

The [`CatalogProvider::schema`](../operations/datafusion_session.catalog.CatalogProvider.md#op-b79abc665424f2077d1417a5) method is synchronous because asynchronous operations should
not be used during planning.  This trait makes it easy to lookup schema references once and cache
them for future planning use.  See [`AsyncSchemaProvider`](../operations/datafusion_catalog.async.AsyncSchemaProvider.md#op-02f401b09b70a7408425eecf) for more details on motivation.

<a id="op-74fb45a267b99af3a546c74f"></a>
## resolve

`function` · `datafusion_catalog::async::AsyncCatalogProvider::resolve` · datafusion-catalog 55.1.0

```rust
async fn resolve(&self, references: &[TableReference], config: &SessionConfig, catalog_name: &str) -> Result<Arc<dyn CatalogProvider>>
```

Source: `src/async.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of schema
providers (each with their own cache of table providers).  This cache will be returned as a
synchronous CatalogProvider that can be used to plan and execute a query containing the given
references.

This cache is intended to be short-lived for the execution of a single query.  There is no mechanism
for refresh or eviction of stale entries.

<a id="op-4ad1abfdde169549f844bfa8"></a>
## schema

`function` · `datafusion_catalog::async::AsyncCatalogProvider::schema` · datafusion-catalog 55.1.0

```rust
async fn schema(&self, name: &str) -> Result<Option<Arc<dyn AsyncSchemaProvider>>>
```

Source: `src/async.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Lookup a schema in the provider
