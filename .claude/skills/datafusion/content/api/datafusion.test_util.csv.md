# `datafusion::test_util::csv`

Crate `datafusion` · 1 public items · structured records in [`model/datafusion.test_util.csv.json`](../model/datafusion.test_util.csv.json)

## TestCsvFile

`struct` · `datafusion::test_util::csv::TestCsvFile`

```rust
struct TestCsvFile
```

**Methods** (3)

```rust
fn path(&self) -> &std::path::Path
fn schema(&self) -> SchemaRef
fn try_new(path: PathBuf, batches: impl IntoIterator<Item = RecordBatch>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test_util.csv.TestCsvFile.md).


a CSV file that has been created for testing.

---
