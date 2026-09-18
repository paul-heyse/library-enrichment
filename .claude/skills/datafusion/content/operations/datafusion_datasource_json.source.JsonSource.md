# `datafusion_datasource_json::source::JsonSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.source.JsonSource.json).

<a id="op-f357c01d00fa1183db25b574"></a>
## JsonSource

`struct` · `datafusion_datasource_json::source::JsonSource` · datafusion-datasource-json 55.1.0

```rust
struct JsonSource
```

Source: `src/source.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

JsonSource holds the extra configuration that is necessary for [`JsonOpener`](../operations/datafusion_datasource_json.source.JsonOpener.md#op-8ccb97bdf780cc0f97aafd27)

<a id="op-262c4efc76e2202420d5e9f3"></a>
## apply_expressions

`function` · `datafusion_datasource_json::source::JsonSource::apply_expressions` · datafusion-datasource-json 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31af52b169ac8f2e07f28aca"></a>
## clone

`function` · `datafusion_datasource_json::source::JsonSource::clone` · datafusion-datasource-json 55.1.0

```rust
fn clone(&self) -> JsonSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 10], "end": [130, 15], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/source.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-418d57aae516d10fae5f9e93"></a>
## create_file_opener

`function` · `datafusion_datasource_json::source::JsonSource::create_file_opener` · datafusion-datasource-json 55.1.0

```rust
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, _partition: usize) -> Result<Arc<dyn FileOpener>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71e22090442f6f6e7c69240d"></a>
## file_type

`function` · `datafusion_datasource_json::source::JsonSource::file_type` · datafusion-datasource-json 55.1.0

```rust
fn file_type(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9f956c646ab21576f1e8374"></a>
## metrics

`function` · `datafusion_datasource_json::source::JsonSource::metrics` · datafusion-datasource-json 55.1.0

```rust
fn metrics(&self) -> &ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5395b0a288cb20e200dd65d"></a>
## new

`function` · `datafusion_datasource_json::source::JsonSource::new` · datafusion-datasource-json 55.1.0

```rust
fn new(table_schema: impl Into<datafusion_datasource::TableSchema>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [162, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Initialize a JsonSource with the provided schema

<a id="op-6ad3ad4d155b855ffe86e5ba"></a>
## projection

`function` · `datafusion_datasource_json::source::JsonSource::projection` · datafusion-datasource-json 55.1.0

```rust
fn projection(&self) -> Option<&ProjectionExprs>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dad9c552cfc30b5a7a6eefd"></a>
## table_schema

`function` · `datafusion_datasource_json::source::JsonSource::table_schema` · datafusion-datasource-json 55.1.0

```rust
fn table_schema(&self) -> &datafusion_datasource::TableSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7f888bbdb80a7a82eab6d97"></a>
## try_from_proto

`function` · `datafusion_datasource_json::source::JsonSource::try_from_proto` · datafusion-datasource-json 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [298, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Reconstructs a `DataSourceExec` from a protobuf `JsonScan`.

Defaults to newline-delimited JSON because protobuf does not encode the mode.

<a id="op-8769565049a452727ef9970e"></a>
## try_pushdown_projection

`function` · `datafusion_datasource_json::source::JsonSource::try_pushdown_projection` · datafusion-datasource-json 55.1.0

```rust
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2afa5f9a936bcfaf2240dc6d"></a>
## try_to_proto

`function` · `datafusion_datasource_json::source::JsonSource::try_to_proto` · datafusion-datasource-json 55.1.0

```rust
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Emit a `JsonScan` node wrapping the shared base config.

<a id="op-9907aaa10a43445a60ac1d99"></a>
## with_batch_size

`function` · `datafusion_datasource_json::source::JsonSource::with_batch_size` · datafusion-datasource-json 55.1.0

```rust
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [262, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab310a1f38a667291333d8e1"></a>
## with_newline_delimited

`function` · `datafusion_datasource_json::source::JsonSource::with_newline_delimited` · datafusion-datasource-json 55.1.0

```rust
fn with_newline_delimited(self, newline_delimited: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonSource", "path": "JsonSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [162, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Set whether to read as newline-delimited JSON.

When `true` (default), expects newline-delimited format.
When `false`, expects JSON array format `[{...}, {...}]`.
