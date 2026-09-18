# `parquet::file::writer::SerializedColumnWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.writer.SerializedColumnWriter.json).

<a id="op-9dfcc6fa1c68565d8942c8b8"></a>
## SerializedColumnWriter

`struct` · `parquet::file::writer::SerializedColumnWriter` · parquet 59.3.0

```rust
struct SerializedColumnWriter<'a>
```

Source: `src/file/writer.rs:974`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A wrapper around a [`ColumnWriter`](../operations/parquet.column.writer.ColumnWriter.md#op-1c10b7f3464136210285624d) that invokes a callback on [`Self::close`](../operations/parquet.file.writer.SerializedColumnWriter.md#op-d37e2d153a26ec37449b5c48)

<a id="op-d37e2d153a26ec37449b5c48"></a>
## close

`function` · `parquet::file::writer::SerializedColumnWriter::close` · parquet 59.3.0

```rust
fn close(self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::file::writer::SerializedColumnWriter", "path": "SerializedColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 1], "end": [1005, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:997`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close this [`SerializedColumnWriter`](../operations/parquet.file.writer.SerializedColumnWriter.md#op-9dfcc6fa1c68565d8942c8b8)

<a id="op-14afd6f3b3316872f6e56058"></a>
## new

`function` · `parquet::file::writer::SerializedColumnWriter::new` · parquet 59.3.0

```rust
fn new(inner: ColumnWriter<'a>, on_close: Option<OnCloseColumnChunk<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::file::writer::SerializedColumnWriter", "path": "SerializedColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 1], "end": [1005, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:982`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`SerializedColumnWriter`](../operations/parquet.file.writer.SerializedColumnWriter.md#op-9dfcc6fa1c68565d8942c8b8) from a [`ColumnWriter`](../operations/parquet.column.writer.ColumnWriter.md#op-1c10b7f3464136210285624d) and an
optional callback to be invoked on [`Self::close`](../operations/parquet.file.writer.SerializedColumnWriter.md#op-d37e2d153a26ec37449b5c48)

<a id="op-8b31ff6d5e0cea8531edada9"></a>
## typed

`function` · `parquet::file::writer::SerializedColumnWriter::typed` · parquet 59.3.0

```rust
fn typed<T: DataType>(&mut self) -> &mut ColumnWriterImpl<'a, T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::file::writer::SerializedColumnWriter", "path": "SerializedColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 1], "end": [1005, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:992`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to a typed [`ColumnWriterImpl`](../operations/parquet.column.writer.ColumnWriterImpl.md#op-69b6d900155474a075b94a16)

<a id="op-2431a2a4111027d3d168eb20"></a>
## untyped

`function` · `parquet::file::writer::SerializedColumnWriter::untyped` · parquet 59.3.0

```rust
fn untyped(&mut self) -> &mut ColumnWriter<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::file::writer::SerializedColumnWriter", "path": "SerializedColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 1], "end": [1005, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:987`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to an untyped [`ColumnWriter`](../operations/parquet.column.writer.ColumnWriter.md#op-1c10b7f3464136210285624d)
