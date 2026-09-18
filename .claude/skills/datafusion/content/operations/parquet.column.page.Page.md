# `parquet::column::page::Page`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.Page.json).

<a id="op-4dfead4d60e4e48ca124b381"></a>
## Page

`enum` · `parquet::column::page::Page` · parquet 59.3.0

```rust
enum Page
```

Source: `src/column/page.rs:35`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet Page definition.

List of supported pages.
These are 1-to-1 mapped from the equivalent Thrift definitions, except `buf` which
used to store uncompressed bytes of the page.

<a id="op-cbda2c5dee705edcfa1bbaf2"></a>
## DataPage

`variant` · `parquet::column::page::Page::DataPage` · parquet 59.3.0

```rust
DataPage
```

Source: `src/column/page.rs:37`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Data page Parquet format v1.

<a id="op-f82c9820f968a8ec038c75f3"></a>
## DataPageV2

`variant` · `parquet::column::page::Page::DataPageV2` · parquet 59.3.0

```rust
DataPageV2
```

Source: `src/column/page.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Data page Parquet format v2.

<a id="op-1263ff17be94d75f57c651ac"></a>
## DictionaryPage

`variant` · `parquet::column::page::Page::DictionaryPage` · parquet 59.3.0

```rust
DictionaryPage
```

Source: `src/column/page.rs:73`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Dictionary page.

<a id="op-421a3ed3febd6ecd4838e5d8"></a>
## buffer

`function` · `parquet::column::page::Page::buffer` · parquet 59.3.0

```rust
fn buffer(&self) -> &Bytes
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [140, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:106`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns internal byte buffer reference for this page.

<a id="op-98a8f88592f35a6d01c32cca"></a>
## clone

`function` · `parquet::column::page::Page::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Page
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/column/page.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/column/page.rs:34`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e7c2d411b3b38241e289148"></a>
## encoding

`function` · `parquet::column::page::Page::encoding` · parquet 59.3.0

```rust
fn encoding(&self) -> Encoding
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [140, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:124`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns this page [`Encoding`](../operations/parquet.basic.Encoding.md#op-ea6de82a506bce95510cae40).

<a id="op-c503afb279f986ef7e7eaa9f"></a>
## fmt

`function` · `parquet::column::page::Page::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 22], "filename": "src/column/page.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/column/page.rs:34`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38f9ac35f303eaf4b16ac18a"></a>
## is_data_page

`function` · `parquet::column::page::Page::is_data_page` · parquet 59.3.0

```rust
fn is_data_page(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [140, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:96`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether this page is any version of a data page

<a id="op-258a1724ee74db6cebf7caf2"></a>
## is_dictionary_page

`function` · `parquet::column::page::Page::is_dictionary_page` · parquet 59.3.0

```rust
fn is_dictionary_page(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [140, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:101`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether this page is a dictionary page

<a id="op-6a0c16348d1135c8e680ee42"></a>
## num_values

`function` · `parquet::column::page::Page::num_values` · parquet 59.3.0

```rust
fn num_values(&self) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [140, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:115`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns number of values in this page.

<a id="op-3bee4387e7fbce7b13575942"></a>
## page_type

`function` · `parquet::column::page::Page::page_type` · parquet 59.3.0

```rust
fn page_type(&self) -> PageType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [140, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:87`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`PageType`](../operations/parquet.basic.PageType.md#op-9932999a245d509642ceeb04) for this page.

<a id="op-6353f7566b97eade919f48ab"></a>
## statistics

`function` · `parquet::column::page::Page::statistics` · parquet 59.3.0

```rust
fn statistics(&self) -> Option<&Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::Page", "path": "Page"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [140, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:133`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns optional [`Statistics`](../operations/parquet.file.statistics.Statistics.md#op-ba51f82bfe4dce01512b0440).
