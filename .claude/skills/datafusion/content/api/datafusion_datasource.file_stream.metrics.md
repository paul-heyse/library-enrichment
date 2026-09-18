# `datafusion_datasource::file_stream::metrics`

Crate `datafusion-datasource` · 2 public items · structured records in [`model/datafusion_datasource.file_stream.metrics.json`](../model/datafusion_datasource.file_stream.metrics.json)

## FileStreamMetrics

`struct` · `datafusion_datasource::file_stream::metrics::FileStreamMetrics`

Also reachable as `datafusion_datasource::file_stream::FileStreamMetrics`

```rust
struct FileStreamMetrics
```

**Fields**: `time_opening`, `time_scanning_until_data`, `time_scanning_total`, `time_processing`, `file_open_errors`, `file_scan_errors`, `files_opened`, `files_processed`

**Methods** (1)

```rust
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_stream.metrics.FileStreamMetrics.md).


Metrics for [`FileStream`]

Note that all of these metrics are in terms of wall clock time
(not cpu time) so they include time spent waiting on I/O as well
as other operators.

[`FileStream`]: <https://github.com/apache/datafusion/blob/main/datafusion/datasource/src/file_stream.rs>

---

## StartableTime

`struct` · `datafusion_datasource::file_stream::metrics::StartableTime`

Also reachable as `datafusion_datasource::file_stream::StartableTime`

```rust
struct StartableTime
```

**Fields**: `metrics`, `start`

**Methods** (2)

```rust
fn start(&mut self)
fn stop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_stream.metrics.StartableTime.md).


A timer that can be started and stopped.

---
