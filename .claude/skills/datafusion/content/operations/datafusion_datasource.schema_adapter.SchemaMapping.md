# `datafusion_datasource::schema_adapter::SchemaMapping`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.schema_adapter.SchemaMapping.json).

<a id="op-eb52a8ea79531b16ae5ae8ed"></a>
## SchemaMapping

`struct` · `datafusion_datasource::schema_adapter::SchemaMapping` · datafusion-datasource 55.1.0

```rust
struct SchemaMapping
```

Source: `src/schema_adapter.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: The SchemaMapping struct held a mapping from the file schema to the table schema.

This struct has been removed.

Use [`PhysicalExprAdapterFactory`] instead to customize scans via
[`FileScanConfigBuilder`], i.e. if you had implemented a custom [`SchemaAdapter`](../operations/datafusion_datasource.schema_adapter.SchemaAdapter.md#op-12d07c2e43ace7176bf97767)
and passed that into [`FileScanConfigBuilder`] / [`ParquetSource`].
Use [`BatchAdapter`] if you want to map a stream of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es
between one schema and another, i.e. if you were calling [`SchemaMapper::map_batch`](../operations/datafusion_datasource.schema_adapter.SchemaMapper.md#op-fefd05e38beee0db08d907aa) manually.

See `upgrading.md` for more details.

[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory
[`FileScanConfigBuilder`]: crate::file_scan_config::FileScanConfigBuilder
[`ParquetSource`]: https://docs.rs/datafusion-datasource-parquet/latest/datafusion_datasource_parquet/source/struct.ParquetSource.html
[`BatchAdapter`]: datafusion_physical_expr_adapter::BatchAdapter

<a id="op-4714f1eef5f7f852e49ba8c8"></a>
## fmt

`function` · `datafusion_datasource::schema_adapter::SchemaMapping::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::SchemaMapping", "path": "SchemaMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 10], "end": [209, 15], "filename": "src/schema_adapter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_adapter.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e01e46615e1c3202d8901afc"></a>
## map_batch

`function` · `datafusion_datasource::schema_adapter::SchemaMapping::map_batch` · datafusion-datasource 55.1.0

```rust
fn map_batch(&self, _batch: RecordBatch) -> Result<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::SchemaMapping", "path": "SchemaMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 1], "end": [232, 2], "filename": "src/schema_adapter.rs"}, "trait": {"args": null, "id": "datafusion_datasource::schema_adapter::SchemaMapper", "path": "SchemaMapper"}, "trait_path": "datafusion_datasource::schema_adapter::SchemaMapper"}`

Source: `src/schema_adapter.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c18073835a435f9b48e687b6"></a>
## map_column_statistics

`function` · `datafusion_datasource::schema_adapter::SchemaMapping::map_column_statistics` · datafusion-datasource 55.1.0

```rust
fn map_column_statistics(&self, _file_col_statistics: &[ColumnStatistics]) -> Result<Vec<ColumnStatistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::SchemaMapping", "path": "SchemaMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 1], "end": [232, 2], "filename": "src/schema_adapter.rs"}, "trait": {"args": null, "id": "datafusion_datasource::schema_adapter::SchemaMapper", "path": "SchemaMapper"}, "trait_path": "datafusion_datasource::schema_adapter::SchemaMapper"}`

Source: `src/schema_adapter.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
