# `parquet::column::writer::ColumnWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.ColumnWriter.json).

<a id="op-1c10b7f3464136210285624d"></a>
## ColumnWriter

`enum` · `parquet::column::writer::ColumnWriter` · parquet 59.3.0

```rust
enum ColumnWriter<'a>
```

Source: `src/column/writer/mod.rs:75`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for a Parquet type.

See [`get_column_writer`](../operations/parquet.column.writer.get_column_writer.md#op-e0f41296e3e94d003a567aec) to create instances of this type

<a id="op-be2cc7dbb557c8373723f976"></a>
## BoolColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::BoolColumnWriter` · parquet 59.3.0

```rust
BoolColumnWriter
```

Source: `src/column/writer/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for boolean type

<a id="op-aa472748697fdd78fd3d2f37"></a>
## ByteArrayColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::ByteArrayColumnWriter` · parquet 59.3.0

```rust
ByteArrayColumnWriter
```

Source: `src/column/writer/mod.rs:89`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for byte array type

<a id="op-309208444bbc80dfcf9f1614"></a>
## DoubleColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::DoubleColumnWriter` · parquet 59.3.0

```rust
DoubleColumnWriter
```

Source: `src/column/writer/mod.rs:87`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for double type

<a id="op-2fd7668bce4b5063270b0455"></a>
## FixedLenByteArrayColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::FixedLenByteArrayColumnWriter` · parquet 59.3.0

```rust
FixedLenByteArrayColumnWriter
```

Source: `src/column/writer/mod.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for fixed length byte array type

<a id="op-b79ebed4052379af0172277a"></a>
## FloatColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::FloatColumnWriter` · parquet 59.3.0

```rust
FloatColumnWriter
```

Source: `src/column/writer/mod.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for float type

<a id="op-d4f8cb46533cfb4943f2f530"></a>
## Int32ColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::Int32ColumnWriter` · parquet 59.3.0

```rust
Int32ColumnWriter
```

Source: `src/column/writer/mod.rs:79`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for int32 type

<a id="op-bd454ab1491323145b55857a"></a>
## Int64ColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::Int64ColumnWriter` · parquet 59.3.0

```rust
Int64ColumnWriter
```

Source: `src/column/writer/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for int64 type

<a id="op-0f522ab6a894ad8bb7cfe80b"></a>
## Int96ColumnWriter

`variant` · `parquet::column::writer::ColumnWriter::Int96ColumnWriter` · parquet 59.3.0

```rust
Int96ColumnWriter
```

Source: `src/column/writer/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column writer for int96 (timestamp) type

<a id="op-2ee4edafe71da5d822f26275"></a>
## close

`function` · `parquet::column::writer::ColumnWriter::close` · parquet 59.3.0

```rust
fn close(self) -> Result<ColumnCloseResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet::column::writer::ColumnWriter", "path": "ColumnWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [120, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close this [`ColumnWriter`](../operations/parquet.column.writer.ColumnWriter.md#op-1c10b7f3464136210285624d), returning the metadata for the column chunk.
