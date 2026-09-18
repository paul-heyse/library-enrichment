# `parquet::column::page::CompressedPage`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.CompressedPage.json).

<a id="op-386e74bc0c85babee32290fb"></a>
## CompressedPage

`struct` · `parquet::column::page::CompressedPage` · parquet 59.3.0

```rust
struct CompressedPage
```

Source: `src/column/page.rs:147`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Helper struct to represent pages with potentially compressed buffer (data page v1) or
compressed and concatenated buffer (def levels + rep levels + compressed values for
data page v2).

The difference with `Page` is that `Page` buffer is always uncompressed.

<a id="op-d451aaf11a9cf10fce24b10e"></a>
## compressed_page

`function` · `parquet::column::page::CompressedPage::compressed_page` · parquet 59.3.0

```rust
fn compressed_page(&self) -> &Page
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:168`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns underlying page with potentially compressed buffer.

<a id="op-365d5cdfcd218a8ccdacd365"></a>
## compressed_size

`function` · `parquet::column::page::CompressedPage::compressed_size` · parquet 59.3.0

```rust
fn compressed_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:181`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns compressed size in bytes.

Note that it is assumed that buffer is compressed, but it may not be. In this
case compressed size will be equal to uncompressed size.

<a id="op-64cc576cafcb4c0edd34eeed"></a>
## data

`function` · `parquet::column::page::CompressedPage::data` · parquet 59.3.0

```rust
fn data(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:196`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of compressed buffer in the page.

<a id="op-7d91c3518d04849d44f91fd1"></a>
## encoding

`function` · `parquet::column::page::CompressedPage::encoding` · parquet 59.3.0

```rust
fn encoding(&self) -> Encoding
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:191`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns encoding for values in page.

<a id="op-9ad04fca3d81ca50ec7269a8"></a>
## memory_usage

`function` · `parquet::column::page::CompressedPage::memory_usage` · parquet 59.3.0

```rust
fn memory_usage(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:205`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of heap bytes this page currently holds.

This is the page's compressed buffer (the embedded [`Bytes`]); use it to
account for a buffered page's memory footprint rather than reaching for
`data().len()` at each call site.

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-0bfd6e34b9e055a19d4cef0d"></a>
## new

`function` · `parquet::column::page::CompressedPage::new` · parquet 59.3.0

```rust
fn new(compressed_page: Page, uncompressed_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:155`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates `CompressedPage` from a page with potentially compressed buffer and
uncompressed size.

<a id="op-4f733f7d95d4a8531cf32c44"></a>
## num_values

`function` · `parquet::column::page::CompressedPage::num_values` · parquet 59.3.0

```rust
fn num_values(&self) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:186`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of values in page.

<a id="op-dcde292f9ce3e3079548ae7c"></a>
## page_type

`function` · `parquet::column::page::CompressedPage::page_type` · parquet 59.3.0

```rust
fn page_type(&self) -> PageType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:163`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns page type.

<a id="op-5b8a0e640428f23f1ce4ed3c"></a>
## uncompressed_size

`function` · `parquet::column::page::CompressedPage::uncompressed_size` · parquet 59.3.0

```rust
fn uncompressed_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::CompressedPage", "path": "CompressedPage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [308, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:173`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns uncompressed size in bytes.
