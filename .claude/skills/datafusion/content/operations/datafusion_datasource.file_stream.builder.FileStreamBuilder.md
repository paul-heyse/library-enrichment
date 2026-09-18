# `datafusion_datasource::file_stream::builder::FileStreamBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.builder.FileStreamBuilder.json).

<a id="op-94246535c08d99e35c929bd7"></a>
## FileStreamBuilder

`struct` · `datafusion_datasource::file_stream::builder::FileStreamBuilder` · datafusion-datasource 55.1.0

```rust
struct FileStreamBuilder<'a>
```

Source: `src/file_stream/builder.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Builder for constructing a [`FileStream`](../operations/datafusion_datasource.file_stream.FileStream.md#op-ce337624786ed9c8ce06a65d).

<a id="op-e89c6674ed0c10e1c4ea17de"></a>
## build

`function` · `datafusion_datasource::file_stream::builder::FileStreamBuilder::build` · datafusion-datasource 55.1.0

```rust
fn build(self) -> Result<FileStream>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::file_stream::builder::FileStreamBuilder", "path": "FileStreamBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [142, 2], "filename": "src/file_stream/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/builder.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Build the configured [`FileStream`](../operations/datafusion_datasource.file_stream.FileStream.md#op-ce337624786ed9c8ce06a65d).

<a id="op-a17763d05845a193b25e8c9a"></a>
## new

`function` · `datafusion_datasource::file_stream::builder::FileStreamBuilder::new` · datafusion-datasource 55.1.0

```rust
fn new(config: &'a FileScanConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::file_stream::builder::FileStreamBuilder", "path": "FileStreamBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [142, 2], "filename": "src/file_stream/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/builder.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new builder for [`FileStream`](../operations/datafusion_datasource.file_stream.FileStream.md#op-ce337624786ed9c8ce06a65d).

<a id="op-01d2a240dd1cc9d3ea5e16a9"></a>
## with_file_opener

`function` · `datafusion_datasource::file_stream::builder::FileStreamBuilder::with_file_opener` · datafusion-datasource 55.1.0

```rust
fn with_file_opener(self, file_opener: Arc<dyn FileOpener>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::file_stream::builder::FileStreamBuilder", "path": "FileStreamBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [142, 2], "filename": "src/file_stream/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/builder.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Configure the [`FileOpener`](../operations/datafusion_datasource.file_stream.FileOpener.md#op-de9e825a0c7367b18e03d6f1) used to open files.

This will overwrite any setting from [`Self::with_morselizer`](../operations/datafusion_datasource.file_stream.builder.FileStreamBuilder.md#op-f48d4426992d478b6d06dd29)

<a id="op-32f6220ca68c431b3fa7787f"></a>
## with_metrics

`function` · `datafusion_datasource::file_stream::builder::FileStreamBuilder::with_metrics` · datafusion-datasource 55.1.0

```rust
fn with_metrics(self, metrics: &'a ExecutionPlanMetricsSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::file_stream::builder::FileStreamBuilder", "path": "FileStreamBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [142, 2], "filename": "src/file_stream/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/builder.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Configure the metrics set used by the stream.

<a id="op-f48d4426992d478b6d06dd29"></a>
## with_morselizer

`function` · `datafusion_datasource::file_stream::builder::FileStreamBuilder::with_morselizer` · datafusion-datasource 55.1.0

```rust
fn with_morselizer(self, morselizer: Box<dyn Morselizer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::file_stream::builder::FileStreamBuilder", "path": "FileStreamBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [142, 2], "filename": "src/file_stream/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/builder.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Configure the [`Morselizer`](../operations/datafusion_datasource.morsel.Morselizer.md#op-4dc8c6c12cccc0db0eec6703) used to open files.

This will overwrite any setting from [`Self::with_file_opener`](../operations/datafusion_datasource.file_stream.builder.FileStreamBuilder.md#op-01d2a240dd1cc9d3ea5e16a9)

<a id="op-1bce06df2e0e033b5c336060"></a>
## with_on_error

`function` · `datafusion_datasource::file_stream::builder::FileStreamBuilder::with_on_error` · datafusion-datasource 55.1.0

```rust
fn with_on_error(self, on_error: OnError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::file_stream::builder::FileStreamBuilder", "path": "FileStreamBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [142, 2], "filename": "src/file_stream/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/builder.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Configure the behavior when opening or scanning a file fails.

<a id="op-61d994f195bc20e59128c26c"></a>
## with_partition

`function` · `datafusion_datasource::file_stream::builder::FileStreamBuilder::with_partition` · datafusion-datasource 55.1.0

```rust
fn with_partition(self, partition: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::file_stream::builder::FileStreamBuilder", "path": "FileStreamBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [142, 2], "filename": "src/file_stream/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/builder.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Configure the partition to scan.
