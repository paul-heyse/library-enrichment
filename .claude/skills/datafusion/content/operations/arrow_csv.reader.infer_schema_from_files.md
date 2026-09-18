# `arrow_csv::reader::infer_schema_from_files`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.reader.infer_schema_from_files.json).

<a id="op-eb2cae404d6f63c1ef756752"></a>
## infer_schema_from_files

`function` · `arrow_csv::reader::infer_schema_from_files` · arrow-csv 59.3.0

```rust
fn infer_schema_from_files(files: &[String], delimiter: u8, max_read_records: Option<usize>, has_header: bool) -> Result<Schema, ArrowError>
```

Source: `src/reader/mod.rs:461`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Infer schema from a list of CSV files by reading through first n records
with `max_read_records` controlling the maximum number of records to read.

Files will be read in the given order until n records have been reached.

If `max_read_records` is not set, all files will be read fully to infer the schema.
