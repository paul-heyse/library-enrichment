# `parquet::column::writer::ColumnWriterImpl`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.ColumnWriterImpl.json).

<a id="op-69b6d900155474a075b94a16"></a>
## ColumnWriterImpl

`type_alias` · `parquet::column::writer::ColumnWriterImpl` · parquet 59.3.0

```rust
type ColumnWriterImpl<'a, T> = GenericColumnWriter<'a, column::writer::encoder::ColumnValueEncoderImpl<T>>
```

Source: `src/column/writer/mod.rs:439`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Typed column writer for a primitive column.
