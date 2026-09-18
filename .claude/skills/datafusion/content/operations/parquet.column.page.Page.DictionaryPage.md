# `parquet::column::page::Page::DictionaryPage`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.Page.DictionaryPage.json).

<a id="op-7ff85baf762384994e53ad81"></a>
## buf

`struct_field` · `parquet::column::page::Page::DictionaryPage::buf` · parquet 59.3.0

```rust
buf: bytes::Bytes
```

Source: `src/column/page.rs:75`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The underlying data buffer

<a id="op-63e1c3a7c2ebeabe5972901c"></a>
## encoding

`struct_field` · `parquet::column::page::Page::DictionaryPage::encoding` · parquet 59.3.0

```rust
encoding: basic::Encoding
```

Source: `src/column/page.rs:79`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encoding for values in this page

<a id="op-63186c3a994e11d3bc2593de"></a>
## is_sorted

`struct_field` · `parquet::column::page::Page::DictionaryPage::is_sorted` · parquet 59.3.0

```rust
is_sorted: bool
```

Source: `src/column/page.rs:81`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Is dictionary page sorted

<a id="op-5a58c8707fce351da216072f"></a>
## num_values

`struct_field` · `parquet::column::page::Page::DictionaryPage::num_values` · parquet 59.3.0

```rust
num_values: u32
```

Source: `src/column/page.rs:77`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of values in this page
