# `datafusion_datasource::file_stream::metrics::FileStreamMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.metrics.FileStreamMetrics.json).

<a id="op-91b845b9ab2e9657710b0210"></a>
## FileStreamMetrics

`struct` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics` · datafusion-datasource 55.1.0

```rust
struct FileStreamMetrics
```

Source: `src/file_stream/metrics.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Metrics for [`FileStream`]

Note that all of these metrics are in terms of wall clock time
(not cpu time) so they include time spent waiting on I/O as well
as other operators.

[`FileStream`]: <https://github.com/apache/datafusion/blob/main/datafusion/datasource/src/file_stream.rs>

<a id="op-793cb4972331ee09c03f3817"></a>
## file_open_errors

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::file_open_errors` · datafusion-datasource 55.1.0

```rust
file_open_errors: datafusion_physical_plan::metrics::Count
```

Source: `src/file_stream/metrics.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Count of errors opening file.

If using `OnError::Skip` this will provide a count of the number of files
which were skipped and will not be included in the scan results.

<a id="op-da87306d797dc6a795b32a0d"></a>
## file_scan_errors

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::file_scan_errors` · datafusion-datasource 55.1.0

```rust
file_scan_errors: datafusion_physical_plan::metrics::Count
```

Source: `src/file_stream/metrics.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Count of errors scanning file

If using `OnError::Skip` this will provide a count of the number of files
which were skipped and will not be included in the scan results.

<a id="op-f5978c7ca7da63e9f850fa0f"></a>
## files_opened

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::files_opened` · datafusion-datasource 55.1.0

```rust
files_opened: datafusion_physical_plan::metrics::Count
```

Source: `src/file_stream/metrics.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Count of files successfully opened or evaluated for processing.
At t=end (completion of a query) this is equal to `files_opened`, and both values are equal
to the total number of files in the query; unless the query itself fails.
This value will always be greater than or equal to `files_open`.
Note that this value does *not* mean the file was actually scanned.
We increment this value for any processing of a file, even if that processing is
discarding it because we hit a `LIMIT` (in this case `files_opened` and `files_processed` are both incremented at the same time).

<a id="op-8e3533475e13ada3bcd9ccff"></a>
## files_processed

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::files_processed` · datafusion-datasource 55.1.0

```rust
files_processed: datafusion_physical_plan::metrics::Count
```

Source: `src/file_stream/metrics.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Count of files completely processed / closed (opened, pruned, or skipped due to limit).
At t=0 (the beginning of a query) this is 0.
At t=end (completion of a query) this is equal to `files_opened`, and both values are equal
to the total number of files in the query; unless the query itself fails.
This value will always be less than or equal to `files_open`.
We increment this value for any processing of a file, even if that processing is
discarding it because we hit a `LIMIT` (in this case `files_opened` and `files_processed` are both incremented at the same time).

<a id="op-de25a5fa7f469acf420b8180"></a>
## new

`function` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::new` · datafusion-datasource 55.1.0

```rust
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::metrics::FileStreamMetrics", "path": "FileStreamMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [159, 2], "filename": "src/file_stream/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/metrics.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-557e44a1b121f3c41ace1d67"></a>
## time_opening

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::time_opening` · datafusion-datasource 55.1.0

```rust
time_opening: StartableTime
```

Source: `src/file_stream/metrics.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Wall clock time elapsed for file opening.

Time between when [`FileOpener::open`] is called and when the
[`FileStream`] receives a stream for reading.

[`FileStream`]: crate::file_stream::FileStream
[`FileOpener::open`]: crate::file_stream::FileOpener::open

<a id="op-f9bc84b9945a199d320b0102"></a>
## time_processing

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::time_processing` · datafusion-datasource 55.1.0

```rust
time_processing: datafusion_physical_plan::metrics::Time
```

Source: `src/file_stream/metrics.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Wall clock time elapsed for data decompression + decoding

Time spent waiting for the FileStream's input.

<a id="op-5da5598134caa37d03ac5625"></a>
## time_scanning_total

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::time_scanning_total` · datafusion-datasource 55.1.0

```rust
time_scanning_total: StartableTime
```

Source: `src/file_stream/metrics.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Total elapsed wall clock time for scanning + record batch decompression / decoding

Sum of time between when the [`FileStream`] requests data from
the stream and when a [`RecordBatch`] is produced for all
record batches in the stream. Note that this metric also
includes the time of the parent operator's execution.

[`FileStream`]: crate::file_stream::FileStream
[`RecordBatch`]: arrow::record_batch::RecordBatch

<a id="op-730e73cd9cd69315ff696477"></a>
## time_scanning_until_data

`struct_field` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics::time_scanning_until_data` · datafusion-datasource 55.1.0

```rust
time_scanning_until_data: StartableTime
```

Source: `src/file_stream/metrics.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Wall clock time elapsed for file scanning + first record batch of decompression + decoding

Time between when the [`FileStream`] requests data from the
stream and when the first [`RecordBatch`] is produced.

[`FileStream`]: crate::file_stream::FileStream
[`RecordBatch`]: arrow::record_batch::RecordBatch
