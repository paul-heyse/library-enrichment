# `parquet::file::page_index::offset_index::PageLocation`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.page_index.offset_index.PageLocation.json).

<a id="op-ae2d7b02d089261641fcb9e0"></a>
## PageLocation

`struct` · `parquet::file::page_index::offset_index::PageLocation` · parquet 59.3.0

```rust
struct PageLocation
```

Source: `src/file/page_index/offset_index.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Page location information for [`OffsetIndexMetaData`](../operations/parquet.file.page_index.offset_index.OffsetIndexMetaData.md#op-c31410ae3eaaa7e7f83822c5)

<a id="op-1d7022053e0bfba12aa1c920"></a>
## clone

`function` · `parquet::file::page_index::offset_index::PageLocation::clone` · parquet 59.3.0

```rust
fn clone(&self) -> PageLocation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::PageLocation", "path": "PageLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [45, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/page_index/offset_index.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caeb2ca709d6ba9e7f2a976a"></a>
## compressed_page_size

`struct_field` · `parquet::file::page_index::offset_index::PageLocation::compressed_page_size` · parquet 59.3.0

```rust
compressed_page_size: i32
```

Source: `src/file/page_index/offset_index.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Size of the page, including header. Sum of compressed_page_size and header

<a id="op-0b146018bbf6820209de27c1"></a>
## eq

`function` · `parquet::file::page_index::offset_index::PageLocation::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &PageLocation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::PageLocation", "path": "PageLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [45, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/page_index/offset_index.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebcbc98506eaea0c49cdd98f"></a>
## first_row_index

`struct_field` · `parquet::file::page_index::offset_index::PageLocation::first_row_index` · parquet 59.3.0

```rust
first_row_index: i64
```

Source: `src/file/page_index/offset_index.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Index within the RowGroup of the first row of the page. When an
OffsetIndex is present, pages must begin on row boundaries
(repetition_level = 0).

<a id="op-b35f9108a321dca65eb06127"></a>
## fmt

`function` · `parquet::file::page_index::offset_index::PageLocation::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::PageLocation", "path": "PageLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [45, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/page_index/offset_index.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2e7d76cb832ca2eeeb244f1"></a>
## offset

`struct_field` · `parquet::file::page_index::offset_index::PageLocation::offset` · parquet 59.3.0

```rust
offset: i64
```

Source: `src/file/page_index/offset_index.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Offset of the page in the file
