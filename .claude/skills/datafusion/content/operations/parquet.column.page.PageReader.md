# `parquet::column::page::PageReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.PageReader.json).

<a id="op-7f8c0bacd485ada3bba28e40"></a>
## PageReader

`trait` · `parquet::column::page::PageReader` · parquet 59.3.0

```rust
trait PageReader: Iterator<Item = errors::Result<Page>> + Send
```

Source: `src/column/page.rs:394`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

API for reading pages from a column chunk.
This offers a iterator like API to get the next page.

<a id="op-7ad160f719253ea8c52c4ef5"></a>
## at_record_boundary

`function` · `parquet::column::page::PageReader::at_record_boundary` · parquet 59.3.0

```rust
fn at_record_boundary(&mut self) -> Result<bool>
```

Source: `src/column/page.rs:417`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if the next page can be assumed to contain the start of a new record

Prior to parquet V2 the specification was ambiguous as to whether a single record
could be split across multiple pages, and prior to [(#4327)] the Rust writer would do
this in certain situations. However, correctly interpreting the offset index relies on
this assumption holding [(#4943)], and so this mechanism is provided for a [`PageReader`](../operations/parquet.column.page.PageReader.md#op-7f8c0bacd485ada3bba28e40)
to signal this to the calling context

[(#4327)]: https://github.com/apache/arrow-rs/pull/4327
[(#4943)]: https://github.com/apache/arrow-rs/pull/4943

<a id="op-d83138d7495078f49fb68c47"></a>
## get_next_page

`function` · `parquet::column::page::PageReader::get_next_page` · parquet 59.3.0

```rust
fn get_next_page(&mut self) -> Result<Option<Page>>
```

Source: `src/column/page.rs:397`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets the next page in the column chunk associated with this reader.
Returns `None` if there are no pages left.

<a id="op-94be98bbc20caf7bea2f446e"></a>
## peek_next_page

`function` · `parquet::column::page::PageReader::peek_next_page` · parquet 59.3.0

```rust
fn peek_next_page(&mut self) -> Result<Option<PageMetadata>>
```

Source: `src/column/page.rs:401`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets metadata about the next page, returns an error if no
column index information

<a id="op-eed16bb297ce5f06f814884c"></a>
## skip_next_page

`function` · `parquet::column::page::PageReader::skip_next_page` · parquet 59.3.0

```rust
fn skip_next_page(&mut self) -> Result<()>
```

Source: `src/column/page.rs:405`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skips reading the next page, returns an error if no
column index information
