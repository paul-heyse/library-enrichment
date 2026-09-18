# `parquet::file::writer::OnCloseColumnChunk`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.writer.OnCloseColumnChunk.json).

<a id="op-23bd48e74029c61a8d7fdd40"></a>
## OnCloseColumnChunk

`type_alias` · `parquet::file::writer::OnCloseColumnChunk` · parquet 59.3.0

```rust
type OnCloseColumnChunk<'a> = Box<dyn FnOnce(column::writer::ColumnCloseResult) -> errors::Result<()> + 'a>
```

Source: `src/file/writer.rs:120`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Callback invoked on closing a column chunk
