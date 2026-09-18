# `datafusion_datasource_avro::source::AvroSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_avro.source.AvroSource.json).

<a id="op-14d616a10fa1f31dadde4946"></a>
## AvroSource

`struct` · `datafusion_datasource_avro::source::AvroSource` · datafusion-datasource-avro 55.1.0

```rust
struct AvroSource
```

Source: `src/source.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

AvroSource holds the extra configuration that is necessary for opening avro files

<a id="op-4f6f36cffcd117c3f55bbba0"></a>
## apply_expressions

`function` · `datafusion_datasource_avro::source::AvroSource::apply_expressions` · datafusion-datasource-avro 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-314eb9be87fbe6bd0720d1d0"></a>
## clone

`function` · `datafusion_datasource_avro::source::AvroSource::clone` · datafusion-datasource-avro 55.1.0

```rust
fn clone(&self) -> AvroSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/source.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-909e18be57312f51f13df434"></a>
## create_file_opener

`function` · `datafusion_datasource_avro::source::AvroSource::create_file_opener` · datafusion-datasource-avro 55.1.0

```rust
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, _base_config: &FileScanConfig, _partition: usize) -> Result<Arc<dyn FileOpener>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41e8fe01f5a61cb8c798ccb4"></a>
## file_type

`function` · `datafusion_datasource_avro::source::AvroSource::file_type` · datafusion-datasource-avro 55.1.0

```rust
fn file_type(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79e19519b8effdfb3b1ccc11"></a>
## metrics

`function` · `datafusion_datasource_avro::source::AvroSource::metrics` · datafusion-datasource-avro 55.1.0

```rust
fn metrics(&self) -> &ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e92c7c4ebd9fe5bcb96f86d"></a>
## new

`function` · `datafusion_datasource_avro::source::AvroSource::new` · datafusion-datasource-avro 55.1.0

```rust
fn new(table_schema: impl Into<TableSchema>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [113, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

Initialize an AvroSource with the provided schema

<a id="op-36a6639519447a7f6e8de8fd"></a>
## projection

`function` · `datafusion_datasource_avro::source::AvroSource::projection` · datafusion-datasource-avro 55.1.0

```rust
fn projection(&self) -> Option<&ProjectionExprs>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d66ebfe1629459f06b5ba2d"></a>
## supports_repartitioning

`function` · `datafusion_datasource_avro::source::AvroSource::supports_repartitioning` · datafusion-datasource-avro 55.1.0

```rust
fn supports_repartitioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eec2b75d40be2ca384f1ab20"></a>
## table_schema

`function` · `datafusion_datasource_avro::source::AvroSource::table_schema` · datafusion-datasource-avro 55.1.0

```rust
fn table_schema(&self) -> &TableSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b447d3e9a204a90774b0ebc6"></a>
## try_from_proto

`function` · `datafusion_datasource_avro::source::AvroSource::try_from_proto` · datafusion-datasource-avro 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [232, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

Reconstructs a `DataSourceExec` from a protobuf `AvroScan`.

<a id="op-0a5a2ac3ce19b6b310defc45"></a>
## try_pushdown_projection

`function` · `datafusion_datasource_avro::source::AvroSource::try_pushdown_projection` · datafusion-datasource-avro 55.1.0

```rust
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75812e59bb65703221b7e4ae"></a>
## try_to_proto

`function` · `datafusion_datasource_avro::source::AvroSource::try_to_proto` · datafusion-datasource-avro 55.1.0

```rust
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

Emit an `AvroScan` node wrapping the shared base config.

<a id="op-43cbd77a0c51f93081493c3c"></a>
## with_batch_size

`function` · `datafusion_datasource_avro::source::AvroSource::with_batch_size` · datafusion-datasource-avro 55.1.0

```rust
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_avro::source::AvroSource", "path": "AvroSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [199, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
