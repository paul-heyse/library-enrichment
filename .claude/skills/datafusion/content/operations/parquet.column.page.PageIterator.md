# `parquet::column::page::PageIterator`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.PageIterator.json).

<a id="op-79635d082b37355e61832b30"></a>
## PageIterator

`trait` · `parquet::column::page::PageIterator` · parquet 59.3.0

```rust
trait PageIterator: Iterator<Item = errors::Result<Box<dyn PageReader>>> + Send
```

Source: `src/column/page.rs:497`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An iterator over pages of one specific column in a parquet file.
