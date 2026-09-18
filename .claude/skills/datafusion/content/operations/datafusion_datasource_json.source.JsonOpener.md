# `datafusion_datasource_json::source::JsonOpener`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.source.JsonOpener.json).

<a id="op-8ccb97bdf780cc0f97aafd27"></a>
## JsonOpener

`struct` · `datafusion_datasource_json::source::JsonOpener` · datafusion-datasource-json 55.1.0

```rust
struct JsonOpener
```

Source: `src/source.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

A [`FileOpener`](../operations/datafusion_datasource.file_stream.FileOpener.md#op-de9e825a0c7367b18e03d6f1) that opens a JSON file and yields a [`FileOpenFuture`](../operations/datafusion_datasource.file_stream.FileOpenFuture.md#op-bfc3ccad5270df581bae26d3)

<a id="op-f03ff64e31903b7242d998b8"></a>
## new

`function` · `datafusion_datasource_json::source::JsonOpener::new` · datafusion-datasource-json 55.1.0

```rust
fn new(batch_size: usize, projected_schema: SchemaRef, file_compression_type: FileCompressionType, object_store: Arc<dyn ObjectStore>, newline_delimited: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonOpener", "path": "JsonOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [127, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Returns a [`JsonOpener`](../operations/datafusion_datasource_json.source.JsonOpener.md#op-8ccb97bdf780cc0f97aafd27)

<a id="op-55cc9904761f780724d68378"></a>
## open

`function` · `datafusion_datasource_json::source::JsonOpener::open` · datafusion-datasource-json 55.1.0

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::source::JsonOpener", "path": "JsonOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [523, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_stream::FileOpener", "path": "FileOpener"}, "trait_path": "datafusion_datasource::file_stream::FileOpener"}`

Source: `src/source.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Open a partitioned JSON file.

If `file_meta.range` is `None`, the entire file is opened.
Else `file_meta.range` is `Some(FileRange{start, end})`, which corresponds to the byte range [start, end) within the file.

Note: `start` or `end` might be in the middle of some lines. In such cases, the following rules
are applied to determine which lines to read:
1. The first line of the partition is the line in which the index of the first character >= `start`.
2. The last line of the partition is the line in which the byte at position `end - 1` resides.

Note: JSON array format does not support range-based scanning.
