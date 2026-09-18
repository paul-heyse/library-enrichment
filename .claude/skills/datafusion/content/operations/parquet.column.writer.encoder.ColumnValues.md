# `parquet::column::writer::encoder::ColumnValues`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.encoder.ColumnValues.json).

<a id="op-4b43aa8227e0e0406d01ce32"></a>
## ColumnValues

`trait` · `parquet::column::writer::encoder::ColumnValues` · parquet 59.3.0

```rust
trait ColumnValues
```

Source: `src/column/writer/encoder.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A collection of [`ParquetValueType`] encoded by a [`ColumnValueEncoder`]

<a id="op-1d64787459ad6ef2674bf449"></a>
## len

`function` · `parquet::column::writer::encoder::ColumnValues::len` · parquet 59.3.0

```rust
fn len(&self) -> usize
```

Source: `src/column/writer/encoder.rs:38`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of values in this collection
