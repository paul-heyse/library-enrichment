# `datafusion_datasource_json::file_format::JsonFormat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.file_format.JsonFormat.json).

<a id="op-61af6052247483fce1d8d429"></a>
## JsonFormat

`struct` · `datafusion_datasource_json::file_format::JsonFormat` · datafusion-datasource-json 55.1.0

```rust
struct JsonFormat
```

Source: `src/file_format.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

JSON `FileFormat` implementation supporting both line-delimited and array formats.

# Supported Formats

## Line-Delimited JSON (default, `newline_delimited = true`)
```text
{"key1": 1, "key2": "val"}
{"key1": 2, "key2": "vals"}
```

## JSON Array Format (`newline_delimited = false`)
```text
[
    {"key1": 1, "key2": "val"},
    {"key1": 2, "key2": "vals"}
]
```

Note: JSON array format is processed using streaming conversion,
which is memory-efficient even for large files.

<a id="op-1eb05c894bd6586febcc0109"></a>
## compression_type

`function` · `datafusion_datasource_json::file_format::JsonFormat::compression_type` · datafusion-datasource-json 55.1.0

```rust
fn compression_type(&self) -> Option<FileCompressionType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c35a945175805da2b5b57372"></a>
## create_physical_plan

`function` · `datafusion_datasource_json::file_format::JsonFormat::create_physical_plan` · datafusion-datasource-json 55.1.0

```rust
async fn create_physical_plan(&self, _state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad919a85beee628459ce89bd"></a>
## create_writer_physical_plan

`function` · `datafusion_datasource_json::file_format::JsonFormat::create_writer_physical_plan` · datafusion-datasource-json 55.1.0

```rust
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, _state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a47ed60b9e04584fb96d086"></a>
## default

`function` · `datafusion_datasource_json::file_format::JsonFormat::default` · datafusion-datasource-json 55.1.0

```rust
fn default() -> JsonFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 17], "end": [151, 24], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fe3e7afdcb90b28226e018d"></a>
## file_source

`function` · `datafusion_datasource_json::file_format::JsonFormat::file_source` · datafusion-datasource-json 55.1.0

```rust
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d32863580534ce8e78610b2d"></a>
## fmt

`function` · `datafusion_datasource_json::file_format::JsonFormat::fmt` · datafusion-datasource-json 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 10], "end": [151, 15], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bddb0df58e5f48a9ff4b25be"></a>
## get_ext

`function` · `datafusion_datasource_json::file_format::JsonFormat::get_ext` · datafusion-datasource-json 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80820fac3810cd7d52a2ef98"></a>
## get_ext_with_compression

`function` · `datafusion_datasource_json::file_format::JsonFormat::get_ext_with_compression` · datafusion-datasource-json 55.1.0

```rust
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c9761da49b97933b7448b86"></a>
## infer_schema

`function` · `datafusion_datasource_json::file_format::JsonFormat::infer_schema` · datafusion-datasource-json 55.1.0

```rust
async fn infer_schema(&self, _state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c621cce94ecbbc98df85294c"></a>
## infer_stats

`function` · `datafusion_datasource_json::file_format::JsonFormat::infer_stats` · datafusion-datasource-json 55.1.0

```rust
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [381, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49e2b0c8a1de4486cae09e0a"></a>
## is_newline_delimited

`function` · `datafusion_datasource_json::file_format::JsonFormat::is_newline_delimited` · datafusion-datasource-json 55.1.0

```rust
fn is_newline_delimited(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [206, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Returns whether this format expects newline-delimited JSON.

<a id="op-c0f506ea7bcad1dea8b75d15"></a>
## options

`function` · `datafusion_datasource_json::file_format::JsonFormat::options` · datafusion-datasource-json 55.1.0

```rust
fn options(&self) -> &JsonOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [206, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Retrieve JSON options

<a id="op-51b6c8ef968682301507df93"></a>
## with_file_compression_type

`function` · `datafusion_datasource_json::file_format::JsonFormat::with_file_compression_type` · datafusion-datasource-json 55.1.0

```rust
fn with_file_compression_type(self, file_compression_type: FileCompressionType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [206, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Set a [`FileCompressionType`](../operations/datafusion_datasource.file_compression_type.FileCompressionType.md#op-bae4301dfbb4229a03c2b319) of JSON
- defaults to `FileCompressionType::UNCOMPRESSED`

<a id="op-acbe796483062f02c94fb07f"></a>
## with_newline_delimited

`function` · `datafusion_datasource_json::file_format::JsonFormat::with_newline_delimited` · datafusion-datasource-json 55.1.0

```rust
fn with_newline_delimited(self, newline_delimited: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [206, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Set whether to read as newline-delimited JSON (NDJSON).

When `true` (default), expects newline-delimited format:
```text
{"a": 1}
{"a": 2}
```

When `false`, expects JSON array format:
```text
[{"a": 1}, {"a": 2}]
```

<a id="op-615dfccf70a8a99e5a76950b"></a>
## with_options

`function` · `datafusion_datasource_json::file_format::JsonFormat::with_options` · datafusion-datasource-json 55.1.0

```rust
fn with_options(self, options: JsonOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [206, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Set JSON options

<a id="op-5dbc913c6b1b4da4b097047b"></a>
## with_schema_infer_max_rec

`function` · `datafusion_datasource_json::file_format::JsonFormat::with_schema_infer_max_rec` · datafusion-datasource-json 55.1.0

```rust
fn with_schema_infer_max_rec(self, max_rec: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::file_format::JsonFormat", "path": "JsonFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [206, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Set a limit in terms of records to scan to infer the schema
- defaults to `DEFAULT_SCHEMA_INFER_MAX_RECORD`
