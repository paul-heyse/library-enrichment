# `datafusion_datasource::schema_adapter::SchemaAdapterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.schema_adapter.SchemaAdapterFactory.json).

<a id="op-3a8d8affcaa558dad7e4798c"></a>
## SchemaAdapterFactory

`trait` · `datafusion_datasource::schema_adapter::SchemaAdapterFactory` · datafusion-datasource 55.1.0

```rust
trait SchemaAdapterFactory: Debug + Send + Sync + 'static
```

Source: `src/schema_adapter.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Factory for creating [`SchemaAdapter`](../operations/datafusion_datasource.schema_adapter.SchemaAdapter.md#op-12d07c2e43ace7176bf97767).

This trait has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

<a id="op-5022263fe3fca2f7fadc646f"></a>
## create

`function` · `datafusion_datasource::schema_adapter::SchemaAdapterFactory::create` · datafusion-datasource 55.1.0

```rust
fn create(&self, projected_table_schema: SchemaRef, table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
```

Source: `src/schema_adapter.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a [`SchemaAdapter`](../operations/datafusion_datasource.schema_adapter.SchemaAdapter.md#op-12d07c2e43ace7176bf97767)

<a id="op-49d62eb673d438dd6649bd65"></a>
## create_with_projected_schema

`function` · `datafusion_datasource::schema_adapter::SchemaAdapterFactory::create_with_projected_schema` · datafusion-datasource 55.1.0

```rust
fn create_with_projected_schema(&self, projected_table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
```

Source: `src/schema_adapter.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a [`SchemaAdapter`](../operations/datafusion_datasource.schema_adapter.SchemaAdapter.md#op-12d07c2e43ace7176bf97767) using only the projected table schema.
