# `parquet::file::writer::OnCloseRowGroup`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.writer.OnCloseRowGroup.json).

<a id="op-9f1befc70dbd929ba19528a6"></a>
## OnCloseRowGroup

`type_alias` · `parquet::file::writer::OnCloseRowGroup` · parquet 59.3.0

```rust
type OnCloseRowGroup<'a, W> = Box<dyn FnOnce(&'a mut TrackedWrite<W>, RowGroupMetaData, Vec<Option<bloom_filter::Sbbf>>, Vec<Option<file::page_index::column_index::ColumnIndexMetaData>>, Vec<Option<file::page_index::offset_index::OffsetIndexMetaData>>) -> errors::Result<()> + Send + 'a>
```

Source: `src/file/writer.rs:127`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Callback invoked on closing a row group, arguments are:

- the row group metadata
- the column index for each column chunk
- the offset index for each column chunk
