# `parquet::schema::printer`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.printer.json).

<a id="op-ef16b096e16b2f14cb27aae8"></a>
## printer

`module` · `parquet::schema::printer` · parquet 59.3.0

```rust
mod printer
```

Source: `src/schema/printer.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet schema printer.
Provides methods to print Parquet file schema and list file metadata.

# Example

```rust
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    schema::printer::{print_file_metadata, print_parquet_metadata, print_schema},
};
use std::{fs::File, path::Path};

// Open a file
let path = Path::new("test.parquet");
if let Ok(file) = File::open(&path) {
    let reader = SerializedFileReader::new(file).unwrap();
    let parquet_metadata = reader.metadata();

    print_parquet_metadata(&mut std::io::stdout(), &parquet_metadata);
    print_file_metadata(&mut std::io::stdout(), &parquet_metadata.file_metadata());

    print_schema(
        &mut std::io::stdout(),
        &parquet_metadata.file_metadata().schema(),
    );
}
```
