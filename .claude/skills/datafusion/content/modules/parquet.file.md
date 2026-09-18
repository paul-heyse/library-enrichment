# `parquet::file`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.json).

<a id="op-32c82f677bb3901ca5a9d887"></a>
## file

`module` · `parquet::file` · parquet 59.3.0

```rust
mod file
```

Source: `src/file/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

APIs for reading parquet data.

Provides access to file and row group readers and writers, record API, metadata, etc.

# See Also:
* [`SerializedFileReader`] and [`SerializedFileWriter`] for reading / writing parquet
* [`metadata`](../modules/parquet.file.metadata.md#op-83908534150f436bfde8c42b): for working with metadata such as schema
* [`statistics`](../modules/parquet.file.statistics.md#op-50c431aef6bd8f8fb6fb3a3a): for working with statistics in metadata

[`SerializedFileReader`]: serialized_reader::SerializedFileReader
[`SerializedFileWriter`]: writer::SerializedFileWriter

# Example of writing a new file

```rust,no_run
use std::{fs, path::Path, sync::Arc};

use parquet::{
    file::{
        properties::WriterProperties,
        writer::SerializedFileWriter,
    },
    schema::parser::parse_message_type,
};

let path = Path::new("/path/to/sample.parquet");

let message_type = "
  message schema {
    REQUIRED INT32 b;
  }
";
let schema = Arc::new(parse_message_type(message_type).unwrap());
let file = fs::File::create(&path).unwrap();
let mut writer = SerializedFileWriter::new(file, schema, Default::default()).unwrap();
let mut row_group_writer = writer.next_row_group().unwrap();
while let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
    // ... write values to a column writer
    col_writer.close().unwrap()
}
row_group_writer.close().unwrap();
writer.close().unwrap();

let bytes = fs::read(&path).unwrap();
assert_eq!(&bytes[0..4], &[b'P', b'A', b'R', b'1']);
```
# Example of reading an existing file

```rust,no_run
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::{fs::File, path::Path};

let path = Path::new("/path/to/sample.parquet");
if let Ok(file) = File::open(&path) {
    let reader = SerializedFileReader::new(file).unwrap();

    let parquet_metadata = reader.metadata();
    assert_eq!(parquet_metadata.num_row_groups(), 1);

    let row_group_reader = reader.get_row_group(0).unwrap();
    assert_eq!(row_group_reader.num_columns(), 1);
}
```
# Example of reading multiple files

```rust,no_run
use parquet::file::reader::SerializedFileReader;
use std::convert::TryFrom;

let paths = vec![
    "/path/to/sample.parquet/part-1.snappy.parquet",
    "/path/to/sample.parquet/part-2.snappy.parquet"
];
// Create a reader for each file and flat map rows
let rows = paths.iter()
    .map(|p| SerializedFileReader::try_from(*p).unwrap())
    .flat_map(|r| r.into_iter());

for row in rows {
    println!("{}", row.unwrap());
}
```
