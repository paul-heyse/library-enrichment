# `datafusion_datasource::schema_adapter::SchemaAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.schema_adapter.SchemaAdapter.json).

<a id="op-12d07c2e43ace7176bf97767"></a>
## SchemaAdapter

`trait` · `datafusion_datasource::schema_adapter::SchemaAdapter` · datafusion-datasource 55.1.0

```rust
trait SchemaAdapter: Send + Sync
```

Source: `src/schema_adapter.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Creates [`SchemaMapper`](../operations/datafusion_datasource.schema_adapter.SchemaMapper.md#op-2c14746d7eb0736f8edfd169)s to map file-level [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to a table schema.

This trait has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

<a id="op-ec8449d65ca8bdaea4d75a3c"></a>
## map_column_index

`function` · `datafusion_datasource::schema_adapter::SchemaAdapter::map_column_index` · datafusion-datasource 55.1.0

```rust
fn map_column_index(&self, index: usize, file_schema: &Schema) -> Option<usize>
```

Source: `src/schema_adapter.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Map a column index in the table schema to a column index in a particular file schema.

<a id="op-62f8c79f689a3b326b8ac834"></a>
## map_schema

`function` · `datafusion_datasource::schema_adapter::SchemaAdapter::map_schema` · datafusion-datasource 55.1.0

```rust
fn map_schema(&self, file_schema: &Schema) -> Result<(Arc<dyn SchemaMapper>, Vec<usize>)>
```

Source: `src/schema_adapter.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a mapping for casting columns from the file schema to the table schema.
