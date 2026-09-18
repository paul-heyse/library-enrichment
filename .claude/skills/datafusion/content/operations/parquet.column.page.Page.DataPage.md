# `parquet::column::page::Page::DataPage`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.Page.DataPage.json).

<a id="op-fc2d8a0ab8135c4b22d4d748"></a>
## buf

`struct_field` · `parquet::column::page::Page::DataPage::buf` · parquet 59.3.0

```rust
buf: bytes::Bytes
```

Source: `src/column/page.rs:39`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The underlying data buffer

<a id="op-f27bdb753e7494dcd2ddd4cf"></a>
## def_level_encoding

`struct_field` · `parquet::column::page::Page::DataPage::def_level_encoding` · parquet 59.3.0

```rust
def_level_encoding: basic::Encoding
```

Source: `src/column/page.rs:45`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Definition level encoding

<a id="op-c5534e18078f813b4f6f0b31"></a>
## encoding

`struct_field` · `parquet::column::page::Page::DataPage::encoding` · parquet 59.3.0

```rust
encoding: basic::Encoding
```

Source: `src/column/page.rs:43`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encoding for values in this page

<a id="op-a5380f7d332974a3e4130e7e"></a>
## num_values

`struct_field` · `parquet::column::page::Page::DataPage::num_values` · parquet 59.3.0

```rust
num_values: u32
```

Source: `src/column/page.rs:41`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of values in this page

<a id="op-ffae2594be30aa4ff0261c07"></a>
## rep_level_encoding

`struct_field` · `parquet::column::page::Page::DataPage::rep_level_encoding` · parquet 59.3.0

```rust
rep_level_encoding: basic::Encoding
```

Source: `src/column/page.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Repetition level encoding

<a id="op-17d7e27305e18f77b878fbf0"></a>
## statistics

`struct_field` · `parquet::column::page::Page::DataPage::statistics` · parquet 59.3.0

```rust
statistics: Option<file::statistics::Statistics>
```

Source: `src/column/page.rs:49`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional statistics for this page
