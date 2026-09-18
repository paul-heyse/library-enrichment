# `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.schema_adapter.DefaultSchemaAdapterFactory.json).

<a id="op-2f3a08752c90846bc952452f"></a>
## DefaultSchemaAdapterFactory

`struct` · `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory` · datafusion-datasource 55.1.0

```rust
struct DefaultSchemaAdapterFactory
```

Source: `src/schema_adapter.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Default [`SchemaAdapterFactory`](../operations/datafusion_datasource.schema_adapter.SchemaAdapterFactory.md#op-3a8d8affcaa558dad7e4798c) for mapping schemas.

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

<a id="op-05aeb0164313dcb492110e1c"></a>
## clone

`function` · `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> DefaultSchemaAdapterFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory", "path": "DefaultSchemaAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 10], "end": [136, 15], "filename": "src/schema_adapter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema_adapter.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7bc85716144a223bfd354ef"></a>
## create

`function` · `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory::create` · datafusion-datasource 55.1.0

```rust
fn create(&self, projected_table_schema: SchemaRef, _table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory", "path": "DefaultSchemaAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [149, 2], "filename": "src/schema_adapter.rs"}, "trait": {"args": null, "id": "datafusion_datasource::schema_adapter::SchemaAdapterFactory", "path": "SchemaAdapterFactory"}, "trait_path": "datafusion_datasource::schema_adapter::SchemaAdapterFactory"}`

Source: `src/schema_adapter.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bba3670495dc5765eda2229"></a>
## default

`function` · `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory::default` · datafusion-datasource 55.1.0

```rust
fn default() -> DefaultSchemaAdapterFactory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory", "path": "DefaultSchemaAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 24], "end": [136, 31], "filename": "src/schema_adapter.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/schema_adapter.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e28162320ae18e460cca9e9b"></a>
## fmt

`function` · `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory", "path": "DefaultSchemaAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 17], "end": [136, 22], "filename": "src/schema_adapter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_adapter.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3994693f28e3a6d752f8b10e"></a>
## from_schema

`function` · `datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory::from_schema` · datafusion-datasource 55.1.0

```rust
fn from_schema(table_schema: SchemaRef) -> Box<dyn SchemaAdapter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::schema_adapter::DefaultSchemaAdapterFactory", "path": "DefaultSchemaAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 1], "end": [166, 2], "filename": "src/schema_adapter.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_adapter.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Deprecated: Create a new factory for mapping batches from a file schema to a table schema.
