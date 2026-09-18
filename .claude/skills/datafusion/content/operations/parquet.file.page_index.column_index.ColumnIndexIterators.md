# `parquet::file::page_index::column_index::ColumnIndexIterators`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.page_index.column_index.ColumnIndexIterators.json).

<a id="op-4d9a333e4fe5de2f6a86244b"></a>
## ColumnIndexIterators

`trait` · `parquet::file::page_index::column_index::ColumnIndexIterators` · parquet 59.3.0

```rust
trait ColumnIndexIterators
```

Source: `src/file/page_index/column_index.rs:636`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provides iterators over min and max values of a [`ColumnIndexMetaData`](../operations/parquet.file.page_index.column_index.ColumnIndexMetaData.md#op-6700acf40da7fd4eab386f3e)

<a id="op-85c9b3ecede155b9303bd68b"></a>
## Item

`assoc_type` · `parquet::file::page_index::column_index::ColumnIndexIterators::Item` · parquet 59.3.0

```rust
Item
```

Source: `src/file/page_index/column_index.rs:639`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Can be one of `bool`, `i32`, `i64`, `Int96`, `f32`, `f64`, [`ByteArray`](../operations/parquet.data_type.ByteArray.md#op-fff27d3428efa4abd643a59e),
or [`FixedLenByteArray`](../operations/parquet.data_type.FixedLenByteArray.md#op-35c24c1502f9d5f953e2357c)

<a id="op-225efa4c0050c777078eafca"></a>
## max_values_iter

`function` · `parquet::file::page_index::column_index::ColumnIndexIterators::max_values_iter` · parquet 59.3.0

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Source: `src/file/page_index/column_index.rs:645`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return iterator over the max values for the index

<a id="op-d29620771641c92d450eba10"></a>
## min_values_iter

`function` · `parquet::file::page_index::column_index::ColumnIndexIterators::min_values_iter` · parquet 59.3.0

```rust
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

Source: `src/file/page_index/column_index.rs:642`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return iterator over the min values for the index
