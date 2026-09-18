# `datafusion_datasource_arrow::source::ArrowSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_arrow.source.ArrowSource.json).

<a id="op-9fe2638d42ab5bc4ffa03a74"></a>
## ArrowSource

`struct` · `datafusion_datasource_arrow::source::ArrowSource` · datafusion-datasource-arrow 55.1.0

```rust
struct ArrowSource
```

Source: `src/source.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

`FileSource` for both Arrow IPC file and stream formats

<a id="op-ed74b4b78efa37e6f6ce734b"></a>
## apply_expressions

`function` · `datafusion_datasource_arrow::source::ArrowSource::apply_expressions` · datafusion-datasource-arrow 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5fbff4de5d8df4b6182dea6"></a>
## clone

`function` · `datafusion_datasource_arrow::source::ArrowSource::clone` · datafusion-datasource-arrow 55.1.0

```rust
fn clone(&self) -> ArrowSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 10], "end": [259, 15], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/source.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f014fa2d71c6a5e69d1bf79c"></a>
## create_file_opener

`function` · `datafusion_datasource_arrow::source::ArrowSource::create_file_opener` · datafusion-datasource-arrow 55.1.0

```rust
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, _base_config: &FileScanConfig, _partition: usize) -> Result<Arc<dyn FileOpener>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9c96fd564ec62dc20a7ab85"></a>
## file_type

`function` · `datafusion_datasource_arrow::source::ArrowSource::file_type` · datafusion-datasource-arrow 55.1.0

```rust
fn file_type(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-133eb1dc5ac21c11953b42b9"></a>
## metrics

`function` · `datafusion_datasource_arrow::source::ArrowSource::metrics` · datafusion-datasource-arrow 55.1.0

```rust
fn metrics(&self) -> &ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4954e91b94eac3ff9349c6b"></a>
## new_file_source

`function` · `datafusion_datasource_arrow::source::ArrowSource::new_file_source` · datafusion-datasource-arrow 55.1.0

```rust
fn new_file_source(table_schema: impl Into<TableSchema>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [267, 1], "end": [289, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Creates an [`ArrowSource`](../operations/datafusion_datasource_arrow.source.ArrowSource.md#op-9fe2638d42ab5bc4ffa03a74) for file format

<a id="op-99705b679ebe4966e9e3e16d"></a>
## new_stream_file_source

`function` · `datafusion_datasource_arrow::source::ArrowSource::new_stream_file_source` · datafusion-datasource-arrow 55.1.0

```rust
fn new_stream_file_source(table_schema: impl Into<TableSchema>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [267, 1], "end": [289, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Creates an [`ArrowSource`](../operations/datafusion_datasource_arrow.source.ArrowSource.md#op-9fe2638d42ab5bc4ffa03a74) for stream format

<a id="op-1aac33c42f76e5abbeefac30"></a>
## projection

`function` · `datafusion_datasource_arrow::source::ArrowSource::projection` · datafusion-datasource-arrow 55.1.0

```rust
fn projection(&self) -> Option<&ProjectionExprs>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5af7e96a840c0ce3f574d9a9"></a>
## repartitioned

`function` · `datafusion_datasource_arrow::source::ArrowSource::repartitioned` · datafusion-datasource-arrow 55.1.0

```rust
fn repartitioned(&self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>, config: &FileScanConfig) -> Result<Option<FileScanConfig>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3270d888b48793d28b69373"></a>
## table_schema

`function` · `datafusion_datasource_arrow::source::ArrowSource::table_schema` · datafusion-datasource-arrow 55.1.0

```rust
fn table_schema(&self) -> &TableSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46cf7068b3b454717eef559c"></a>
## try_from_proto

`function` · `datafusion_datasource_arrow::source::ArrowSource::try_from_proto` · datafusion-datasource-arrow 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [463, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Reconstructs a `DataSourceExec` from a protobuf `ArrowScan`.

Defaults to the IPC file format because protobuf does not distinguish it
from the IPC stream format.

<a id="op-1a9eb5533b5e616b1a0f1fdb"></a>
## try_pushdown_projection

`function` · `datafusion_datasource_arrow::source::ArrowSource::try_pushdown_projection` · datafusion-datasource-arrow 55.1.0

```rust
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fcd72ee6bed0466a7b3fe81"></a>
## try_to_proto

`function` · `datafusion_datasource_arrow::source::ArrowSource::try_to_proto` · datafusion-datasource-arrow 55.1.0

```rust
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Emit an `ArrowScan` node wrapping the shared base config.

Decoding defaults to the IPC file format because protobuf does not
distinguish it from the IPC stream format.

<a id="op-6b840a162114bd486b16750b"></a>
## with_batch_size

`function` · `datafusion_datasource_arrow::source::ArrowSource::with_batch_size` · datafusion-datasource-arrow 55.1.0

```rust
fn with_batch_size(&self, _batch_size: usize) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowSource", "path": "ArrowSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [427, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
