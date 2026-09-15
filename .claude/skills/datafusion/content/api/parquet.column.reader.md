# `parquet::column::reader`

Crate `parquet` · 5 public items · structured records in [`model/parquet.column.reader.json`](../model/parquet.column.reader.json)

## ColumnReader

`enum` · `parquet::column::reader::ColumnReader`

```rust
enum ColumnReader
```

**Variants**: `BoolColumnReader`, `Int32ColumnReader`, `Int64ColumnReader`, `Int96ColumnReader`, `FloatColumnReader`, `DoubleColumnReader`, `ByteArrayColumnReader`, `FixedLenByteArrayColumnReader`

Column reader for a Parquet type.

---

## get_column_reader

`function` · `parquet::column::reader::get_column_reader`

```rust
fn get_column_reader(col_descr: schema::types::ColumnDescPtr, col_page_reader: Box<dyn PageReader>) -> ColumnReader
```

Gets a specific column reader corresponding to column descriptor `col_descr`. The
column reader will read from pages in `col_page_reader`.

---

## get_typed_column_reader

`function` · `parquet::column::reader::get_typed_column_reader`

```rust
fn get_typed_column_reader<T: DataType>(col_reader: ColumnReader) -> ColumnReaderImpl<T>
```

Gets a typed column reader for the specific type `T`, by "up-casting" `col_reader` of
non-generic type to a generic column reader type `ColumnReaderImpl`.

Panics if actual enum value for `col_reader` does not match the type `T`.

---

## GenericColumnReader

`struct` · `parquet::column::reader::GenericColumnReader`

```rust
struct GenericColumnReader<R, D, V>
```

**Methods** (3)

```rust
fn new(descr: ColumnDescPtr, page_reader: Box<dyn PageReader>) -> Self
fn read_records(&mut self, max_records: usize, def_levels: Option<&mut D::Buffer>, rep_levels: Option<&mut R::Buffer>, values: &mut V::Buffer) -> Result<(usize, usize, usize)>
fn skip_records(&mut self, num_records: usize) -> Result<usize>
```

Reads data for a given column chunk, using the provided decoders:

- R: `ColumnLevelDecoder` used to decode repetition levels
- D: `ColumnLevelDecoder` used to decode definition levels
- V: `ColumnValueDecoder` used to decode value data

---

## ColumnReaderImpl

`type_alias` · `parquet::column::reader::ColumnReaderImpl`

```rust
type ColumnReaderImpl<T> = GenericColumnReader<column::reader::decoder::RepetitionLevelDecoderImpl, column::reader::decoder::DefinitionLevelDecoderImpl, column::reader::decoder::ColumnValueDecoderImpl<T>>
```

Typed value reader for a particular primitive column.

---
