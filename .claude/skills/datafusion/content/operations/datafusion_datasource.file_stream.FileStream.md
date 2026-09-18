# `datafusion_datasource::file_stream::FileStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.FileStream.json).

<a id="op-ce337624786ed9c8ce06a65d"></a>
## FileStream

`struct` · `datafusion_datasource::file_stream::FileStream` · datafusion-datasource 55.1.0

```rust
struct FileStream
```

Source: `src/file_stream/mod.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A stream that iterates record batch by record batch, file over file.

<a id="op-ae364fae2cf28a84e8e1da0a"></a>
## Item

`assoc_type` · `datafusion_datasource::file_stream::FileStream::Item` · datafusion-datasource 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [128, 2], "filename": "src/file_stream/mod.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/file_stream/mod.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d917a942b4c71b12b935f245"></a>
## new

`function` · `datafusion_datasource::file_stream::FileStream::new` · datafusion-datasource 55.1.0

```rust
fn new(config: &FileScanConfig, partition: usize, file_opener: Arc<dyn FileOpener>, metrics: &ExecutionPlanMetricsSet) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [116, 2], "filename": "src/file_stream/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/mod.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new `FileStream` using the give `FileOpener` to scan underlying files

<a id="op-46b9e17ca26717e53b3ed57a"></a>
## poll_next

`function` · `datafusion_datasource::file_stream::FileStream::poll_next` · datafusion-datasource 55.1.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [128, 2], "filename": "src/file_stream/mod.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/file_stream/mod.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3264a62f9071ed184b88c4ce"></a>
## schema

`function` · `datafusion_datasource::file_stream::FileStream::schema` · datafusion-datasource 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [134, 2], "filename": "src/file_stream/mod.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/file_stream/mod.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cf53da9678c44ceeb9b7aa2"></a>
## with_on_error

`function` · `datafusion_datasource::file_stream::FileStream::with_on_error` · datafusion-datasource 55.1.0

```rust
fn with_on_error(self, on_error: OnError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [116, 2], "filename": "src/file_stream/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/mod.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Specify the behavior when an error occurs opening or scanning a file

If `OnError::Skip` the stream will skip files which encounter an error and continue
If `OnError:Fail` (default) the stream will fail and stop processing when an error occurs
