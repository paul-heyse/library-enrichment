# `datafusion_datasource_parquet::file_format::ParquetFormat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.file_format.ParquetFormat.json).

<a id="op-730d8a0235d3096826b9a46a"></a>
## ParquetFormat

`struct` · `datafusion_datasource_parquet::file_format::ParquetFormat` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetFormat
```

Source: `src/file_format.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

The Apache Parquet `FileFormat` implementation

<a id="op-73c64ab27a3c8bfdb027f628"></a>
## binary_as_string

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::binary_as_string` · datafusion-datasource-parquet 55.1.0

```rust
fn binary_as_string(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return `true` if binary types will be read as strings.

If this returns true, DataFusion will instruct the parquet reader
to read binary columns such as `Binary` or `BinaryView` as the
corresponding string type such as `Utf8` or `LargeUtf8`.
The parquet reader has special optimizations for `Utf8` and `LargeUtf8`
validation, and such queries are significantly faster than reading
binary columns and then casting to string columns.

<a id="op-97b8a77041b696a682c5b3b9"></a>
## coerce_int96

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::coerce_int96` · datafusion-datasource-parquet 55.1.0

```rust
fn coerce_int96(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5126c68f8dedeeb20a7ab604"></a>
## compression_type

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::compression_type` · datafusion-datasource-parquet 55.1.0

```rust
fn compression_type(&self) -> Option<FileCompressionType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78e71915c62c4726182b3081"></a>
## create_physical_plan

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::create_physical_plan` · datafusion-datasource-parquet 55.1.0

```rust
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebcde7d8038b505086d202a5"></a>
## create_writer_physical_plan

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::create_writer_physical_plan` · datafusion-datasource-parquet 55.1.0

```rust
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, _state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:524`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a97698e4827fedc0968bb0ad"></a>
## default

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::default` · datafusion-datasource-parquet 55.1.0

```rust
fn default() -> ParquetFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 17], "end": [144, 24], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70dff05ccda38646d6bb507b"></a>
## enable_pruning

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::enable_pruning` · datafusion-datasource-parquet 55.1.0

```rust
fn enable_pruning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return `true` if pruning is enabled

<a id="op-ca231a639f97a600cfea1f1d"></a>
## file_source

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::file_source` · datafusion-datasource-parquet 55.1.0

```rust
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:562`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68dcb8b9dc113878b067b315"></a>
## fmt

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 10], "end": [144, 15], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91822f655c2f1edb7e6769fd"></a>
## force_view_types

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::force_view_types` · datafusion-datasource-parquet 55.1.0

```rust
fn force_view_types(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return `true` if should use view types.

If this returns true, DataFusion will instruct the parquet reader
to read string / binary columns using view `StringView` or `BinaryView`
if the table schema specifies those types, regardless of any embedded metadata
that may specify an alternate Arrow type. The parquet reader is optimized
for reading `StringView` and `BinaryView` and such queries are significantly faster.

If this returns false, the parquet reader will read the columns according to the
defaults or any embedded Arrow type information. This may result in reading
`StringArrays` and then casting to `StringViewArray` which is less efficient.

<a id="op-712d46fbb8b4c3f1ef916866"></a>
## get_ext

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::get_ext` · datafusion-datasource-parquet 55.1.0

```rust
fn get_ext(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f764c69bb015a6813e41e82b"></a>
## get_ext_with_compression

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::get_ext_with_compression` · datafusion-datasource-parquet 55.1.0

```rust
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a2f6fdc858c440fbebf66dd"></a>
## infer_ordering

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::infer_ordering` · datafusion-datasource-parquet 55.1.0

```rust
async fn infer_ordering(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<Option<LexOrdering>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74c3044811df4c521899cf50"></a>
## infer_schema

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::infer_schema` · datafusion-datasource-parquet 55.1.0

```rust
async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f286f9e83db4bcbc5dd57fc1"></a>
## infer_stats

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::infer_stats` · datafusion-datasource-parquet 55.1.0

```rust
async fn infer_stats(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9544d3d248dfc8dce121bd6"></a>
## infer_stats_and_ordering

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::infer_stats_and_ordering` · datafusion-datasource-parquet 55.1.0

```rust
async fn infer_stats_and_ordering(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<datafusion_datasource::file_format::FileMeta>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [568, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_format::FileFormat", "path": "FileFormat"}, "trait_path": "datafusion_datasource::file_format::FileFormat"}`

Source: `src/file_format.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d149b190b7aa5009cda768a8"></a>
## metadata_size_hint

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::metadata_size_hint` · datafusion-datasource-parquet 55.1.0

```rust
fn metadata_size_hint(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return the metadata size hint if set

<a id="op-bcbe34b5703705b3b35d78e5"></a>
## new

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Construct a new Format with no local overrides

<a id="op-0ed6abf92cf3edb0df3bb480"></a>
## options

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::options` · datafusion-datasource-parquet 55.1.0

```rust
fn options(&self) -> &TableParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Parquet options

<a id="op-2f052391cacde89b80e11f26"></a>
## skip_metadata

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::skip_metadata` · datafusion-datasource-parquet 55.1.0

```rust
fn skip_metadata(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Returns `true` if schema metadata will be cleared prior to
schema merging.

<a id="op-e5ac4ef32101e3a9fb2bc0b0"></a>
## with_binary_as_string

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::with_binary_as_string` · datafusion-datasource-parquet 55.1.0

```rust
fn with_binary_as_string(self, binary_as_string: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

If true, will read binary types as strings. See [`Self::binary_as_string`](../operations/datafusion_datasource_parquet.file_format.ParquetFormat.md#op-73c64ab27a3c8bfdb027f628) for details

<a id="op-68775c8c311d4aee430433c6"></a>
## with_coerce_int96

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::with_coerce_int96` · datafusion-datasource-parquet 55.1.0

```rust
fn with_coerce_int96(self, time_unit: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6010fd2346f3097ff7f8b6d4"></a>
## with_enable_pruning

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::with_enable_pruning` · datafusion-datasource-parquet 55.1.0

```rust
fn with_enable_pruning(self, enable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Activate statistics based row group level pruning
- If `None`, defaults to value on `config_options`

<a id="op-fb90be2145060afb381c6d35"></a>
## with_force_view_types

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::with_force_view_types` · datafusion-datasource-parquet 55.1.0

```rust
fn with_force_view_types(self, use_views: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

If true, will use view types. See [`Self::force_view_types`](../operations/datafusion_datasource_parquet.file_format.ParquetFormat.md#op-91822f655c2f1edb7e6769fd) for details

<a id="op-6671764e4e707039b11934c6"></a>
## with_metadata_size_hint

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::with_metadata_size_hint` · datafusion-datasource-parquet 55.1.0

```rust
fn with_metadata_size_hint(self, size_hint: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Provide a hint to the size of the file metadata. If a hint is provided
the reader will try and fetch the last `size_hint` bytes of the parquet file optimistically.
Without a hint, two read are required. One read to fetch the 8-byte parquet footer and then
another read to fetch the metadata length encoded in the footer.

- If `None`, defaults to value on `config_options`

<a id="op-fb8cf015881fbecdf6d77144"></a>
## with_options

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::with_options` · datafusion-datasource-parquet 55.1.0

```rust
fn with_options(self, options: TableParquetOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set Parquet options for the ParquetFormat

<a id="op-486d5523cb522ac62d24069d"></a>
## with_skip_metadata

`function` · `datafusion_datasource_parquet::file_format::ParquetFormat::with_skip_metadata` · datafusion-datasource-parquet 55.1.0

```rust
fn with_skip_metadata(self, skip_metadata: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::file_format::ParquetFormat", "path": "ParquetFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [257, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Tell the parquet reader to skip any metadata that may be in
the file Schema. This can help avoid schema conflicts due to
metadata.

- If `None`, defaults to value on `config_options`
