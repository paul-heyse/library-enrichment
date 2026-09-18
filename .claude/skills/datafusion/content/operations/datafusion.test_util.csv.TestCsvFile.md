# `datafusion::test_util::csv::TestCsvFile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.csv.TestCsvFile.json).

<a id="op-ae335fcf96af29c210420a15"></a>
## TestCsvFile

`struct` · `datafusion::test_util::csv::TestCsvFile` · datafusion 55.1.0

```rust
struct TestCsvFile
```

Source: `src/test_util/csv.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

a CSV file that has been created for testing.

<a id="op-27983145a266e801c6b49a9d"></a>
## path

`function` · `datafusion::test_util::csv::TestCsvFile::path` · datafusion 55.1.0

```rust
fn path(&self) -> &std::path::Path
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::csv::TestCsvFile", "path": "TestCsvFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [69, 2], "filename": "src/test_util/csv.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/csv.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The path to the csv file

<a id="op-f1d2a78471594e117f3f7175"></a>
## schema

`function` · `datafusion::test_util::csv::TestCsvFile::schema` · datafusion 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::csv::TestCsvFile", "path": "TestCsvFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [69, 2], "filename": "src/test_util/csv.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/csv.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The schema of this csv file

<a id="op-a91068e2353582e79eddbda7"></a>
## try_new

`function` · `datafusion::test_util::csv::TestCsvFile::try_new` · datafusion 55.1.0

```rust
fn try_new(path: PathBuf, batches: impl IntoIterator<Item = RecordBatch>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::csv::TestCsvFile", "path": "TestCsvFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [69, 2], "filename": "src/test_util/csv.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/csv.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new csv file at the specified location
