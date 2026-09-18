# `parquet::record::api::RowFormatter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.RowFormatter.json).

<a id="op-aae81814eb0c83cc7d2b37e9"></a>
## RowFormatter

`trait` · `parquet::record::api::RowFormatter` · parquet 59.3.0

```rust
trait RowFormatter
```

Source: `src/record/api.rs:205`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Trait for formatting fields within a Row.

# Examples

```
use std::fs::File;
use std::path::Path;
use parquet::record::Row;
use parquet::record::RowFormatter;
use parquet::file::reader::{FileReader, SerializedFileReader};

if let Ok(file) = File::open(&Path::new("test.parquet")) {
    let reader = SerializedFileReader::new(file).unwrap();
    let row = reader.get_row_iter(None).unwrap().next().unwrap().unwrap();
    println!("column 0: {}, column 1: {}", row.fmt(0), row.fmt(1));
}
```


<a id="op-92a8b2df8477913b01f7a407"></a>
## fmt

`function` · `parquet::record::api::RowFormatter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, i: usize) -> &dyn fmt::Display
```

Source: `src/record/api.rs:207`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The method to format a field at the given index.
