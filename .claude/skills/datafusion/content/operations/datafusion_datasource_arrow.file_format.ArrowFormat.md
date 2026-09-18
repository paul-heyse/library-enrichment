# `datafusion_datasource_arrow::file_format::ArrowFormat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_arrow.file_format.ArrowFormat.json).

<a id="op-b1ded2a94ee7e92e27d6b1de"></a>
## ArrowFormat

`struct` · `datafusion_datasource_arrow::file_format::ArrowFormat` · datafusion-datasource-arrow 55.1.0

```rust
struct ArrowFormat
```

Source: `src/file_format.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Arrow [`FileFormat`](../operations/datafusion_datasource.file_format.FileFormat.md#op-81da8109cfb5e6919dc4a7ee) implementation.

<a id="op-8030030b1c0af40190bef89a"></a>
## compression_type

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::compression_type` · datafusion-datasource-arrow 55.1.0

```rust
fn compression_type(&self) -> Option<FileCompressionType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b052dfaf9875b1c1fb71f5b6"></a>
## create_physical_plan

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::create_physical_plan` · datafusion-datasource-arrow 55.1.0

```rust
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c93e438418b67b8dbce96363"></a>
## create_writer_physical_plan

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::create_writer_physical_plan` · datafusion-datasource-arrow 55.1.0

```rust
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, _state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e78d798e9373c79d7e8ac309"></a>
## default

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::default` · datafusion-datasource-arrow 55.1.0

```rust
fn default() -> ArrowFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 10], "end": [110, 17], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abb4d1a25d75d7caf3999ce9"></a>
## file_source

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::file_source` · datafusion-datasource-arrow 55.1.0

```rust
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1666b8a4fa436fb468440627"></a>
## fmt

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::fmt` · datafusion-datasource-arrow 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 19], "end": [110, 24], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fb5b5093446d903b8144e85"></a>
## get_ext

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::get_ext` · datafusion-datasource-arrow 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c747025f504bc578fd061e0"></a>
## get_ext_with_compression

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::get_ext_with_compression` · datafusion-datasource-arrow 55.1.0

```rust
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c379605179b8ab770d65c28a"></a>
## infer_schema

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::infer_schema` · datafusion-datasource-arrow 55.1.0

```rust
async fn infer_schema(&self, _state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee55b33ad88fc50341ea895a"></a>
## infer_stats

`function` · `datafusion_datasource_arrow::file_format::ArrowFormat::infer_stats` · datafusion-datasource-arrow 55.1.0

```rust
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::file_format::ArrowFormat", "path": "ArrowFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [245, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
