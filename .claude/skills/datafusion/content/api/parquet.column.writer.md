# `parquet::column::writer`

Crate `parquet` · 8 public items · structured records in [`model/parquet.column.writer.json`](../model/parquet.column.writer.json)

## ColumnWriter

`enum` · `parquet::column::writer::ColumnWriter`

```rust
enum ColumnWriter<'a>
```

**Variants**: `BoolColumnWriter`, `Int32ColumnWriter`, `Int64ColumnWriter`, `Int96ColumnWriter`, `FloatColumnWriter`, `DoubleColumnWriter`, `ByteArrayColumnWriter`, `FixedLenByteArrayColumnWriter`

**Methods** (1)

```rust
fn close(self) -> Result<ColumnCloseResult>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.ColumnWriter.md).


Column writer for a Parquet type.

See [`get_column_writer`] to create instances of this type

---

## get_column_writer

`function` · `parquet::column::writer::get_column_writer`

```rust
fn get_column_writer<'a>(descr: schema::types::ColumnDescPtr, props: file::properties::WriterPropertiesPtr, page_writer: Box<dyn PageWriter + 'a>) -> ColumnWriter<'a>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.get_column_writer.md).


Create a specific column writer corresponding to column descriptor `descr`.

---

## get_typed_column_writer

`function` · `parquet::column::writer::get_typed_column_writer`

```rust
fn get_typed_column_writer<T: DataType>(col_writer: ColumnWriter<'_>) -> ColumnWriterImpl<'_, T>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.get_typed_column_writer.md).


Gets a typed column writer for the specific type `T`, by "up-casting" `col_writer` of
non-generic type to a generic column writer type `ColumnWriterImpl`.

Panics if actual enum value for `col_writer` does not match the type `T`.

---

## get_typed_column_writer_mut

`function` · `parquet::column::writer::get_typed_column_writer_mut`

```rust
fn get_typed_column_writer_mut<'a, 'b: 'a, T: DataType>(col_writer: &'a mut ColumnWriter<'b>) -> &'a mut ColumnWriterImpl<'b, T>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.get_typed_column_writer_mut.md).


Similar to `get_typed_column_writer` but returns a reference.

---

## get_typed_column_writer_ref

`function` · `parquet::column::writer::get_typed_column_writer_ref`

```rust
fn get_typed_column_writer_ref<'a, 'b: 'a, T: DataType>(col_writer: &'b ColumnWriter<'a>) -> &'b ColumnWriterImpl<'a, T>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.get_typed_column_writer_ref.md).


Similar to `get_typed_column_writer` but returns a reference.

---

## ColumnCloseResult

`struct` · `parquet::column::writer::ColumnCloseResult`

```rust
struct ColumnCloseResult
```

**Fields**: `bytes_written`, `rows_written`, `metadata`, `bloom_filter`, `column_index`, `offset_index`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn update_dictionary_location(self, dictionary_len: usize) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.ColumnCloseResult.md).


Metadata for a column chunk of a Parquet file.

Note this structure is returned by [`ColumnWriter::close`].

---

## GenericColumnWriter

`struct` · `parquet::column::writer::GenericColumnWriter`

```rust
struct GenericColumnWriter<'a, E: ColumnValueEncoder>
```

**Methods** (7)

```rust
fn close(self) -> Result<ColumnCloseResult>
fn get_descriptor(&self) -> &ColumnDescPtr
fn get_total_bytes_written(&self) -> u64
fn get_total_rows_written(&self) -> u64
fn new(descr: ColumnDescPtr, props: WriterPropertiesPtr, page_writer: Box<dyn PageWriter + 'a>) -> Self
fn write_batch(&mut self, values: &E::Values, def_levels: Option<&[i16]>, rep_levels: Option<&[i16]>) -> Result<usize>
fn write_batch_with_statistics(&mut self, values: &E::Values, def_levels: Option<&[i16]>, rep_levels: Option<&[i16]>, min: Option<&E::T>, max: Option<&E::T>, distinct_count: Option<u64>) -> Result<usize>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.GenericColumnWriter.md).


Generic column writer for a primitive Parquet column

---

## ColumnWriterImpl

`type_alias` · `parquet::column::writer::ColumnWriterImpl`

```rust
type ColumnWriterImpl<'a, T> = GenericColumnWriter<'a, column::writer::encoder::ColumnValueEncoderImpl<T>>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.writer.ColumnWriterImpl.md).


Typed column writer for a primitive column.

---
