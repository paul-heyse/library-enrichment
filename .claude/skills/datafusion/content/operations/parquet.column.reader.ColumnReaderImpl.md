# `parquet::column::reader::ColumnReaderImpl`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.ColumnReaderImpl.json).

<a id="op-4f95cd21f22fc927e88eeaa2"></a>
## ColumnReaderImpl

`type_alias` · `parquet::column::reader::ColumnReaderImpl` · parquet 59.3.0

```rust
type ColumnReaderImpl<T> = GenericColumnReader<column::reader::decoder::RepetitionLevelDecoderImpl, column::reader::decoder::DefinitionLevelDecoderImpl, column::reader::decoder::ColumnValueDecoderImpl<T>>
```

Source: `src/column/reader.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Typed value reader for a particular primitive column.
