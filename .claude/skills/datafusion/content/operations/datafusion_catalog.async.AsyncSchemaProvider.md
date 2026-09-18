# `datafusion_catalog::async::AsyncSchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.async.AsyncSchemaProvider.json).

<a id="op-02f401b09b70a7408425eecf"></a>
## AsyncSchemaProvider

`trait` · `datafusion_catalog::async::AsyncSchemaProvider` · datafusion-catalog 55.1.0

```rust
trait AsyncSchemaProvider: Send + Sync
```

Source: `src/async.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

A trait for schema providers that must resolve tables asynchronously

The [`SchemaProvider::table`](../operations/datafusion_session.schema.SchemaProvider.md#op-2f6985f66089b074a19145db) method _is_ asynchronous.  However, this is primarily for convenience and
it is not a good idea for this method to be slow as this will cause poor planning performance.

It is a better idea to resolve the tables once and cache them in memory for the duration of
planning.  This trait helps implement that pattern.

After implementing this trait you can call the [`AsyncSchemaProvider::resolve`](../operations/datafusion_catalog.async.AsyncSchemaProvider.md#op-565aacf0f8e77dcb568c42fb) method to get an
`Arc<dyn SchemaProvider>` that contains a cached copy of the referenced tables.  The `resolve`
method can be slow and asynchronous as it is only called once, before planning.

See the [remote_catalog.rs] for an end to end example

[remote_catalog.rs]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/data_io/remote_catalog.rs

<a id="op-565aacf0f8e77dcb568c42fb"></a>
## resolve

`function` · `datafusion_catalog::async::AsyncSchemaProvider::resolve` · datafusion-catalog 55.1.0

```rust
async fn resolve(&self, references: &[TableReference], config: &SessionConfig, catalog_name: &str, schema_name: &str) -> Result<Arc<dyn SchemaProvider>>
```

Source: `src/async.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of table
providers.  This cache will be returned as a synchronous TableProvider that can be used to plan
and execute a query containing the given references.

This cache is intended to be short-lived for the execution of a single query.  There is no mechanism
for refresh or eviction of stale entries.

See the [`AsyncSchemaProvider`](../operations/datafusion_catalog.async.AsyncSchemaProvider.md#op-02f401b09b70a7408425eecf) documentation for additional details

<a id="op-c852def67d9a72edb94cad50"></a>
## table

`function` · `datafusion_catalog::async::AsyncSchemaProvider::table` · datafusion-catalog 55.1.0

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

Source: `src/async.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Lookup a table in the schema provider
