# `parquet::file::serialized_reader::ReadOptions`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.serialized_reader.ReadOptions.json).

<a id="op-369feadda83636e9edbaf29f"></a>
## ReadOptions

`struct` · `parquet::file::serialized_reader::ReadOptions` · parquet 59.3.0

```rust
struct ReadOptions
```

Source: `src/file/serialized_reader.rs:221`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A collection of options for reading a Parquet file.

Predicates are currently only supported on row group metadata.
All predicates will be chained using 'AND' to filter the row groups.
