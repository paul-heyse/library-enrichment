# `datafusion_catalog::async`

Crate `datafusion-catalog` · 3 public items · structured records in [`model/datafusion_catalog.async.json`](../model/datafusion_catalog.async.json)

## AsyncCatalogProvider

`trait` · `datafusion_catalog::async::AsyncCatalogProvider`

```rust
trait AsyncCatalogProvider: Send + Sync
```

**Methods** (2)

```rust
async fn resolve(&self, references: &[TableReference], config: &SessionConfig, catalog_name: &str) -> Result<Arc<dyn CatalogProvider>>
async fn schema(&self, name: &str) -> Result<Option<Arc<dyn AsyncSchemaProvider>>>
```

A trait for catalog providers that must resolve schemas asynchronously

The [`CatalogProvider::schema`] method is synchronous because asynchronous operations should
not be used during planning.  This trait makes it easy to lookup schema references once and cache
them for future planning use.  See [`AsyncSchemaProvider`] for more details on motivation.

---

## AsyncCatalogProviderList

`trait` · `datafusion_catalog::async::AsyncCatalogProviderList`

```rust
trait AsyncCatalogProviderList: Send + Sync
```

**Methods** (2)

```rust
async fn catalog(&self, name: &str) -> Result<Option<Arc<dyn AsyncCatalogProvider>>>
async fn resolve(&self, references: &[TableReference], config: &SessionConfig) -> Result<Arc<dyn CatalogProviderList>>
```

A trait for catalog provider lists that must resolve catalogs asynchronously

The [`CatalogProviderList::catalog`] method is synchronous because asynchronous operations should
not be used during planning.  This trait makes it easy to lookup catalog references once and cache
them for future planning use.  See [`AsyncSchemaProvider`] for more details on motivation.

---

## AsyncSchemaProvider

`trait` · `datafusion_catalog::async::AsyncSchemaProvider`

```rust
trait AsyncSchemaProvider: Send + Sync
```

**Methods** (2)

```rust
async fn resolve(&self, references: &[TableReference], config: &SessionConfig, catalog_name: &str, schema_name: &str) -> Result<Arc<dyn SchemaProvider>>
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

A trait for schema providers that must resolve tables asynchronously

The [`SchemaProvider::table`] method _is_ asynchronous.  However, this is primarily for convenience and
it is not a good idea for this method to be slow as this will cause poor planning performance.

It is a better idea to resolve the tables once and cache them in memory for the duration of
planning.  This trait helps implement that pattern.

After implementing this trait you can call the [`AsyncSchemaProvider::resolve`] method to get an
`Arc<dyn SchemaProvider>` that contains a cached copy of the referenced tables.  The `resolve`
method can be slow and asynchronous as it is only called once, before planning.

See the [remote_catalog.rs] for an end to end example

[remote_catalog.rs]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/data_io/remote_catalog.rs

---
