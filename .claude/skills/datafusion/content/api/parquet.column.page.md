# `parquet::column::page`

Crate `parquet` · 7 public items · structured records in [`model/parquet.column.page.json`](../model/parquet.column.page.json)

## Page

`enum` · `parquet::column::page::Page`

```rust
enum Page
```

**Variants**: `DataPage`, `DataPageV2`, `DictionaryPage`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn buffer(&self) -> &Bytes
fn encoding(&self) -> Encoding
fn is_data_page(&self) -> bool
fn is_dictionary_page(&self) -> bool
fn num_values(&self) -> u32
fn page_type(&self) -> PageType
fn statistics(&self) -> Option<&Statistics>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page.Page.md).


Parquet Page definition.

List of supported pages.
These are 1-to-1 mapped from the equivalent Thrift definitions, except `buf` which
used to store uncompressed bytes of the page.

---

## CompressedPage

`struct` · `parquet::column::page::CompressedPage`

```rust
struct CompressedPage
```

**Methods** (9)

```rust
fn compressed_page(&self) -> &Page
fn compressed_size(&self) -> usize
fn data(&self) -> &[u8]
fn encoding(&self) -> Encoding
fn memory_usage(&self) -> usize
fn new(compressed_page: Page, uncompressed_size: usize) -> Self
fn num_values(&self) -> u32
fn page_type(&self) -> PageType
fn uncompressed_size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page.CompressedPage.md).


Helper struct to represent pages with potentially compressed buffer (data page v1) or
compressed and concatenated buffer (def levels + rep levels + compressed values for
data page v2).

The difference with `Page` is that `Page` buffer is always uncompressed.

---

## PageMetadata

`struct` · `parquet::column::page::PageMetadata`

```rust
struct PageMetadata
```

**Fields**: `num_rows`, `num_levels`, `is_dict`

**Derives**: Clone

[Full member, field, variant and typed contracts](../operations/parquet.column.page.PageMetadata.md).


Contains metadata for a page

---

## PageWriteSpec

`struct` · `parquet::column::page::PageWriteSpec`

```rust
struct PageWriteSpec
```

**Fields**: `page_type`, `uncompressed_size`, `compressed_size`, `num_values`, `offset`, `bytes_written`

**Derives**: Default

**Methods** (1)

```rust
fn new() -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page.PageWriteSpec.md).


Contains page write metrics.

---

## PageIterator

`trait` · `parquet::column::page::PageIterator`

```rust
trait PageIterator: Iterator<Item = errors::Result<Box<dyn PageReader>>> + Send
```

**Implementors** (1)

- `parquet::file::reader::FilePageIterator`

[Full member, field, variant and typed contracts](../operations/parquet.column.page.PageIterator.md).


An iterator over pages of one specific column in a parquet file.

---

## PageReader

`trait` · `parquet::column::page::PageReader`

```rust
trait PageReader: Iterator<Item = errors::Result<Page>> + Send
```

**Implementors** (1)

- `parquet::file::serialized_reader::SerializedPageReader`

**Methods** (4)

```rust
fn at_record_boundary(&mut self) -> Result<bool>
fn get_next_page(&mut self) -> Result<Option<Page>>
fn peek_next_page(&mut self) -> Result<Option<PageMetadata>>
fn skip_next_page(&mut self) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page.PageReader.md).


API for reading pages from a column chunk.
This offers a iterator like API to get the next page.

---

## PageWriter

`trait` · `parquet::column::page::PageWriter`

```rust
trait PageWriter: Send
```

**Implementors** (1)

- `parquet::file::writer::SerializedPageWriter`

**Methods** (2)

```rust
fn close(&mut self) -> Result<()>
fn write_page(&mut self, page: CompressedPage) -> Result<PageWriteSpec>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.page.PageWriter.md).


API for writing pages in a column chunk.

It is reasonable to assume that all pages will be written in the correct order, e.g.
dictionary page followed by data pages, or a set of data pages, etc.

---
