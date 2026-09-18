# `datafusion_datasource_csv::source::CsvOpener`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.source.CsvOpener.json).

<a id="op-7aec00729af8c1d605c18193"></a>
## CsvOpener

`struct` · `datafusion_datasource_csv::source::CsvOpener` · datafusion-datasource-csv 55.1.0

```rust
struct CsvOpener
```

Source: `src/source.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

A [`FileOpener`](../operations/datafusion_datasource.file_stream.FileOpener.md#op-de9e825a0c7367b18e03d6f1) that opens a CSV file and yields a [`FileOpenFuture`](../operations/datafusion_datasource.file_stream.FileOpenFuture.md#op-bfc3ccad5270df581bae26d3)

<a id="op-4371f1ededcccf62004cb143"></a>
## new

`function` · `datafusion_datasource_csv::source::CsvOpener::new` · datafusion-datasource-csv 55.1.0

```rust
fn new(config: Arc<CsvSource>, file_compression_type: FileCompressionType, object_store: Arc<dyn ObjectStore>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvOpener", "path": "CsvOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [235, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Returns a [`CsvOpener`](../operations/datafusion_datasource_csv.source.CsvOpener.md#op-7aec00729af8c1d605c18193)

<a id="op-40cc4a1f7a92c993f54a18d9"></a>
## open

`function` · `datafusion_datasource_csv::source::CsvOpener::open` · datafusion-datasource-csv 55.1.0

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::source::CsvOpener", "path": "CsvOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [497, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_stream::FileOpener", "path": "FileOpener"}, "trait_path": "datafusion_datasource::file_stream::FileOpener"}`

Source: `src/source.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Open a partitioned CSV file.

If `file_meta.range` is `None`, the entire file is opened.
If `file_meta.range` is `Some(FileRange {start, end})`, this signifies that the partition
corresponds to the byte range [start, end) within the file.

Note: `start` or `end` might be in the middle of some lines. In such cases, the following rules
are applied to determine which lines to read:
1. The first line of the partition is the line in which the index of the first character >= `start`.
2. The last line of the partition is the line in which the byte at position `end - 1` resides.

Examples:
Consider the following partitions enclosed by braces `{}`:

{A,1,2,3,4,5,6,7,8,9\n
 A,1,2,3,4,5,6,7,8,9\n}
 A,1,2,3,4,5,6,7,8,9\n
 The lines read would be: [0, 1]

 A,{1,2,3,4,5,6,7,8,9\n
 A,1,2,3,4,5,6,7,8,9\n
 A},1,2,3,4,5,6,7,8,9\n
 The lines read would be: [1, 2]
