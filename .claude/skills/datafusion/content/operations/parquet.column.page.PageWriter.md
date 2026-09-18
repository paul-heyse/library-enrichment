# `parquet::column::page::PageWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.PageWriter.json).

<a id="op-1532a369b470f0e4d5dfd714"></a>
## PageWriter

`trait` · `parquet::column::page::PageWriter` · parquet 59.3.0

```rust
trait PageWriter: Send
```

Source: `src/column/page.rs:433`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

API for writing pages in a column chunk.

It is reasonable to assume that all pages will be written in the correct order, e.g.
dictionary page followed by data pages, or a set of data pages, etc.

<a id="op-d49583ab5f15c6ab0ef3cfff"></a>
## close

`function` · `parquet::column::page::PageWriter::close` · parquet 59.3.0

```rust
fn close(&mut self) -> Result<()>
```

Source: `src/column/page.rs:493`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Closes resources and flushes underlying sink.
Page writer should not be used after this method is called.

<a id="op-516f573db87510880f16caa7"></a>
## write_page

`function` · `parquet::column::page::PageWriter::write_page` · parquet 59.3.0

```rust
fn write_page(&mut self, page: CompressedPage) -> Result<PageWriteSpec>
```

Source: `src/column/page.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writes a page into the output stream/sink.
Returns `PageWriteSpec` that contains information about written page metrics,
including number of bytes, size, number of values, offset, etc.

This method is called for every compressed page we write into underlying buffer,
either data page or dictionary page.
