# `parquet::column::page_store`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page_store.json).

<a id="op-431c010416b169255c17f11c"></a>
## page_store

`module` · `parquet::column::page_store` · parquet 59.3.0

```rust
mod page_store
```

Source: `src/column/page_store.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Pluggable storage for completed, serialized page blobs.

While a row group is being written the [`ArrowWriter`] must buffer every
column's encoded pages, because Parquet requires each column chunk to be
contiguous in the file while record batches arrive with all columns interleaved.
By default that buffer lives on the heap, so the writer's peak memory grows
with the row group size. A [`PageStore`](../operations/parquet.column.page_store.PageStore.md#op-049980600cf52224c7af9631) lets the buffer live somewhere else
— a local temp file, object storage, etc. — bounding peak write memory
independently of the row group size.

[`ArrowWriter`]: crate::arrow::arrow_writer::ArrowWriter
