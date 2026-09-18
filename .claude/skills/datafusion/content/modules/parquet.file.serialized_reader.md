# `parquet::file::serialized_reader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.serialized_reader.json).

<a id="op-968ed0d01cdfe5b0af655032"></a>
## serialized_reader

`module` · `parquet::file::serialized_reader` · parquet 59.3.0

```rust
mod serialized_reader
```

Source: `src/file/serialized_reader.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Contains implementations of the reader traits FileReader, RowGroupReader and PageReader
Also contains implementations of the ChunkReader for files (with buffering) and byte arrays (RAM)
