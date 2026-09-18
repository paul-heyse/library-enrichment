# `parquet::data_type::DataType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.DataType.json).

<a id="op-7f29b486e21564dbfbfb77f4"></a>
## DataType

`trait` · `parquet::data_type::DataType` · parquet 59.3.0

```rust
trait DataType: 'static + Send
```

Source: `src/data_type.rs:1204`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Contains the Parquet physical type information as well as the Rust primitive type
presentation.

<a id="op-1914469c4459a6b214d0f35e"></a>
## T

`assoc_type` · `parquet::data_type::DataType::T` · parquet 59.3.0

```rust
T
```

Source: `src/data_type.rs:1206`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The physical type of the Parquet data type.

<a id="op-423fb20e46809b7c60f76652"></a>
## get_column_reader

`function` · `parquet::data_type::DataType::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(column_writer: ColumnReader) -> Option<ColumnReaderImpl<Self>> where Self: Sized
```

Source: `src/data_type.rs:1217`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the underlying [`ColumnReaderImpl`](../operations/parquet.column.reader.ColumnReaderImpl.md#op-4f95cd21f22fc927e88eeaa2) for the given [`ColumnReader`](../operations/parquet.column.reader.ColumnReader.md#op-8a6c8a9c55bdcd4d2b1facd6).

<a id="op-0319a1110ce5f7edbe8497a5"></a>
## get_column_writer

`function` · `parquet::data_type::DataType::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>> where Self: Sized
```

Source: `src/data_type.rs:1222`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the underlying [`ColumnWriterImpl`](../operations/parquet.column.writer.ColumnWriterImpl.md#op-69b6d900155474a075b94a16) for the given [`ColumnWriter`](../operations/parquet.column.writer.ColumnWriter.md#op-1c10b7f3464136210285624d).

<a id="op-99487680cad17ffbfe669211"></a>
## get_column_writer_mut

`function` · `parquet::data_type::DataType::get_column_writer_mut` · parquet 59.3.0

```rust
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>> where Self: Sized
```

Source: `src/data_type.rs:1234`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a mutable reference to the underlying [`ColumnWriterImpl`](../operations/parquet.column.writer.ColumnWriterImpl.md#op-69b6d900155474a075b94a16) for the given

<a id="op-7573c43638df1b5176a0dab9"></a>
## get_column_writer_ref

`function` · `parquet::data_type::DataType::get_column_writer_ref` · parquet 59.3.0

```rust
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'b ColumnWriter<'a>) -> Option<&'b ColumnWriterImpl<'a, Self>> where Self: Sized
```

Source: `src/data_type.rs:1227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the underlying [`ColumnWriterImpl`](../operations/parquet.column.writer.ColumnWriterImpl.md#op-69b6d900155474a075b94a16) for the given [`ColumnWriter`](../operations/parquet.column.writer.ColumnWriter.md#op-1c10b7f3464136210285624d).

<a id="op-0b148683cfe1722e8d6f4db3"></a>
## get_physical_type

`function` · `parquet::data_type::DataType::get_physical_type` · parquet 59.3.0

```rust
fn get_physical_type() -> Type
```

Source: `src/data_type.rs:1209`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns Parquet physical type.

<a id="op-e78fdb3196abae48f8f7c665"></a>
## get_type_size

`function` · `parquet::data_type::DataType::get_type_size` · parquet 59.3.0

```rust
fn get_type_size() -> usize
```

Source: `src/data_type.rs:1214`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns size in bytes for Rust representation of the physical type.
