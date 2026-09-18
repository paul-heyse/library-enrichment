# `datafusion::test_util::parquet::TestParquetFile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.parquet.TestParquetFile.json).

<a id="op-73191b974a0825772f995e97"></a>
## TestParquetFile

`struct` · `datafusion::test_util::parquet::TestParquetFile` · datafusion 55.1.0

```rust
struct TestParquetFile
```

Source: `src/test_util/parquet.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

a ParquetFile that has been created for testing.

<a id="op-e6edb26fd290a15701f5968b"></a>
## create_scan

`function` · `datafusion::test_util::parquet::TestParquetFile::create_scan` · datafusion 55.1.0

```rust
fn create_scan(&self, ctx: &SessionContext, maybe_filter: Option<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::TestParquetFile", "path": "TestParquetFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [229, 2], "filename": "src/test_util/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/parquet.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a `DataSourceExec` with the specified options.

If `maybe_filter` is non-None, the DataSourceExec will be filtered using
the given expression, and this method will return the same plan that DataFusion
will make with a pushed down predicate followed by a filter:

```text
(FilterExec)
  (DataSourceExec)
```

Otherwise if `maybe_filter` is None, return just a `DataSourceExec`

<a id="op-5eaa6e71429e91a4dddad862"></a>
## parquet_metrics

`function` · `datafusion::test_util::parquet::TestParquetFile::parquet_metrics` · datafusion 55.1.0

```rust
fn parquet_metrics(plan: &Arc<dyn ExecutionPlan>) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::TestParquetFile", "path": "TestParquetFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [229, 2], "filename": "src/test_util/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/parquet.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Retrieve metrics from the parquet exec returned from `create_scan`

Recursively searches for DataSourceExec and returns the metrics
on the first one it finds

<a id="op-5075d33b4541f1a0c55e491e"></a>
## path

`function` · `datafusion::test_util::parquet::TestParquetFile::path` · datafusion 55.1.0

```rust
fn path(&self) -> &std::path::Path
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::TestParquetFile", "path": "TestParquetFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [229, 2], "filename": "src/test_util/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/parquet.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The path to the parquet file

<a id="op-2dd13b6e8ec5c6a5425c4eb8"></a>
## schema

`function` · `datafusion::test_util::parquet::TestParquetFile::schema` · datafusion 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::TestParquetFile", "path": "TestParquetFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [229, 2], "filename": "src/test_util/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/parquet.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The schema of this parquet file

<a id="op-080ffd5b1bd74dab9b83ca3f"></a>
## try_new

`function` · `datafusion::test_util::parquet::TestParquetFile::try_new` · datafusion 55.1.0

```rust
fn try_new(path: PathBuf, props: WriterProperties, batches: impl IntoIterator<Item = RecordBatch>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::TestParquetFile", "path": "TestParquetFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [138, 2], "filename": "src/test_util/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/parquet.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new parquet file at the specified location with the
given properties
