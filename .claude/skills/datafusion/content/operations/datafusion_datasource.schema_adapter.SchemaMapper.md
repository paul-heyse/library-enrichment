# `datafusion_datasource::schema_adapter::SchemaMapper`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.schema_adapter.SchemaMapper.json).

<a id="op-2c14746d7eb0736f8edfd169"></a>
## SchemaMapper

`trait` · `datafusion_datasource::schema_adapter::SchemaMapper` · datafusion-datasource 55.1.0

```rust
trait SchemaMapper: Debug + Send + Sync
```

Source: `src/schema_adapter.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Maps columns from a specific file schema to the table schema.

This trait has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

<a id="op-fefd05e38beee0db08d907aa"></a>
## map_batch

`function` · `datafusion_datasource::schema_adapter::SchemaMapper::map_batch` · datafusion-datasource 55.1.0

```rust
fn map_batch(&self, batch: RecordBatch) -> Result<RecordBatch>
```

Source: `src/schema_adapter.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Adapts a `RecordBatch` to match the `table_schema`.

<a id="op-1da323deeeabb1f28d58dd88"></a>
## map_column_statistics

`function` · `datafusion_datasource::schema_adapter::SchemaMapper::map_column_statistics` · datafusion-datasource 55.1.0

```rust
fn map_column_statistics(&self, file_col_statistics: &[ColumnStatistics]) -> Result<Vec<ColumnStatistics>>
```

Source: `src/schema_adapter.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Adapts file-level column `Statistics` to match the `table_schema`.
