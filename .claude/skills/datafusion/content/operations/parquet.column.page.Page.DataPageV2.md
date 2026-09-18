# `parquet::column::page::Page::DataPageV2`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.Page.DataPageV2.json).

<a id="op-5bc7cb9b67419cb8997a93d5"></a>
## buf

`struct_field` · `parquet::column::page::Page::DataPageV2::buf` · parquet 59.3.0

```rust
buf: bytes::Bytes
```

Source: `src/column/page.rs:54`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The underlying data buffer

<a id="op-0a5d49d9a9d052f33cda2243"></a>
## def_levels_byte_len

`struct_field` · `parquet::column::page::Page::DataPageV2::def_levels_byte_len` · parquet 59.3.0

```rust
def_levels_byte_len: u32
```

Source: `src/column/page.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Length of definition levels

<a id="op-e2e6cdf4d79e597a96db88a4"></a>
## encoding

`struct_field` · `parquet::column::page::Page::DataPageV2::encoding` · parquet 59.3.0

```rust
encoding: basic::Encoding
```

Source: `src/column/page.rs:58`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encoding for values in this page

<a id="op-b3a5a59ff97cc3b352929607"></a>
## is_compressed

`struct_field` · `parquet::column::page::Page::DataPageV2::is_compressed` · parquet 59.3.0

```rust
is_compressed: bool
```

Source: `src/column/page.rs:68`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Is this page compressed

<a id="op-7b7b51f33b116f5728367dc0"></a>
## num_nulls

`struct_field` · `parquet::column::page::Page::DataPageV2::num_nulls` · parquet 59.3.0

```rust
num_nulls: u32
```

Source: `src/column/page.rs:60`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of null values in this page

<a id="op-9b6a2912506340c66af1d8d2"></a>
## num_rows

`struct_field` · `parquet::column::page::Page::DataPageV2::num_rows` · parquet 59.3.0

```rust
num_rows: u32
```

Source: `src/column/page.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of rows in this page

<a id="op-9ce007c9426a88217fc1ddec"></a>
## num_values

`struct_field` · `parquet::column::page::Page::DataPageV2::num_values` · parquet 59.3.0

```rust
num_values: u32
```

Source: `src/column/page.rs:56`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of values in this page

<a id="op-49b638d39e30227e6195c057"></a>
## rep_levels_byte_len

`struct_field` · `parquet::column::page::Page::DataPageV2::rep_levels_byte_len` · parquet 59.3.0

```rust
rep_levels_byte_len: u32
```

Source: `src/column/page.rs:66`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Length of repetition levels

<a id="op-0ee55f4c9ec9d7a5ad97ce1c"></a>
## statistics

`struct_field` · `parquet::column::page::Page::DataPageV2::statistics` · parquet 59.3.0

```rust
statistics: Option<file::statistics::Statistics>
```

Source: `src/column/page.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional statistics for this page
