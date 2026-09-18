# `datafusion_datasource::schema_adapter`

Crate `datafusion-datasource` · 6 public items · structured records in [`model/datafusion_datasource.schema_adapter.json`](../model/datafusion_datasource.schema_adapter.json)

## DefaultSchemaAdapterFactory

`struct` · `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory`

> **Deprecated** — since 52.0.0: DefaultSchemaAdapterFactory has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.

Also reachable as `datafusion::datasource::schema_adapter::DefaultSchemaAdapterFactory`

```rust
struct DefaultSchemaAdapterFactory
```

**Implements**: `datafusion_datasource::schema_adapter::SchemaAdapterFactory`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn from_schema(table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
```

**via `datafusion_datasource::schema_adapter::SchemaAdapterFactory`**

```rust
fn create(&self, projected_table_schema: SchemaRef, _table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.schema_adapter.DefaultSchemaAdapterFactory.md).


Deprecated: Default [`SchemaAdapterFactory`] for mapping schemas.

This struct has been removed.

Use [`PhysicalExprAdapterFactory`] instead to customize scans via
[`FileScanConfigBuilder`], i.e. if you had implemented a custom [`SchemaAdapter`]
and passed that into [`FileScanConfigBuilder`] / [`ParquetSource`].
Use [`BatchAdapter`] if you want to map a stream of [`RecordBatch`]es
between one schema and another, i.e. if you were calling [`SchemaMapper::map_batch`] manually.

See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory
[`FileScanConfigBuilder`]: crate::file_scan_config::FileScanConfigBuilder
[`ParquetSource`]: https://docs.rs/datafusion-datasource-parquet/latest/datafusion_datasource_parquet/source/struct.ParquetSource.html
[`BatchAdapter`]: datafusion_physical_expr_adapter::BatchAdapter

---

## SchemaMapping

`struct` · `datafusion_datasource::schema_adapter::SchemaMapping`

> **Deprecated** — since 52.0.0: SchemaMapping has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.

Also reachable as `datafusion::datasource::schema_adapter::SchemaMapping`

```rust
struct SchemaMapping
```

**Implements**: `datafusion_datasource::schema_adapter::SchemaMapper`

**Derives**: Debug

**via `datafusion_datasource::schema_adapter::SchemaMapper`**

```rust
fn map_batch(&self, _batch: RecordBatch) -> Result<RecordBatch>
fn map_column_statistics(&self, _file_col_statistics: &[ColumnStatistics]) -> Result<Vec<ColumnStatistics>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.schema_adapter.SchemaMapping.md).


Deprecated: The SchemaMapping struct held a mapping from the file schema to the table schema.

This struct has been removed.

Use [`PhysicalExprAdapterFactory`] instead to customize scans via
[`FileScanConfigBuilder`], i.e. if you had implemented a custom [`SchemaAdapter`]
and passed that into [`FileScanConfigBuilder`] / [`ParquetSource`].
Use [`BatchAdapter`] if you want to map a stream of [`RecordBatch`]es
between one schema and another, i.e. if you were calling [`SchemaMapper::map_batch`] manually.

See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory
[`FileScanConfigBuilder`]: crate::file_scan_config::FileScanConfigBuilder
[`ParquetSource`]: https://docs.rs/datafusion-datasource-parquet/latest/datafusion_datasource_parquet/source/struct.ParquetSource.html
[`BatchAdapter`]: datafusion_physical_expr_adapter::BatchAdapter

---

## SchemaAdapter

`trait` · `datafusion_datasource::schema_adapter::SchemaAdapter`

> **Deprecated** — since 52.0.0: SchemaAdapter has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.

Also reachable as `datafusion::datasource::schema_adapter::SchemaAdapter`

```rust
trait SchemaAdapter: Send + Sync
```

**Methods** (2)

```rust
fn map_column_index(&self, index: usize, file_schema: &Schema) -> Option<usize>
fn map_schema(&self, file_schema: &Schema) -> Result<(Arc<dyn SchemaMapper>, Vec<usize>)>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.schema_adapter.SchemaAdapter.md).


Deprecated: Creates [`SchemaMapper`]s to map file-level [`RecordBatch`]es to a table schema.

This trait has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

---

## SchemaAdapterFactory

`trait` · `datafusion_datasource::schema_adapter::SchemaAdapterFactory`

> **Deprecated** — since 52.0.0: SchemaAdapter has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.

Also reachable as `datafusion::datasource::schema_adapter::SchemaAdapterFactory`

```rust
trait SchemaAdapterFactory: Debug + Send + Sync + 'static
```

**Implementors** (1)

- `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory`

**Methods** (2)

```rust
fn create(&self, projected_table_schema: SchemaRef, table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
fn create_with_projected_schema(&self, projected_table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.schema_adapter.SchemaAdapterFactory.md).


Deprecated: Factory for creating [`SchemaAdapter`].

This trait has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

---

## SchemaMapper

`trait` · `datafusion_datasource::schema_adapter::SchemaMapper`

> **Deprecated** — since 52.0.0: SchemaMapper has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.

Also reachable as `datafusion::datasource::schema_adapter::SchemaMapper`

```rust
trait SchemaMapper: Debug + Send + Sync
```

**Implementors** (1)

- `datafusion_datasource::schema_adapter::SchemaMapping`

**Methods** (2)

```rust
fn map_batch(&self, batch: RecordBatch) -> Result<RecordBatch>
fn map_column_statistics(&self, file_col_statistics: &[ColumnStatistics]) -> Result<Vec<ColumnStatistics>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.schema_adapter.SchemaMapper.md).


Deprecated: Maps columns from a specific file schema to the table schema.

This trait has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

---

## CastColumnFn

`type_alias` · `datafusion_datasource::schema_adapter::CastColumnFn`

> **Deprecated** — since 52.0.0: SchemaAdapter has been removed. Use PhysicalExprAdapterFactory instead. See upgrading.md for more details.

Also reachable as `datafusion::datasource::schema_adapter::CastColumnFn`

```rust
type CastColumnFn = dyn Fn(&arrow::array::ArrayRef, &arrow::datatypes::Field, &arrow::compute::CastOptions<'_>) -> datafusion_common::Result<arrow::array::ArrayRef> + Send + Sync
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.schema_adapter.CastColumnFn.md).


Deprecated: Function type for casting columns.

This type has been removed. Use [`PhysicalExprAdapterFactory`] instead.
See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

---
