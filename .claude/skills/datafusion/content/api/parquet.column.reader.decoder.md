# `parquet::column::reader::decoder`

Crate `parquet` · 7 public items · structured records in [`model/parquet.column.reader.decoder.json`](../model/parquet.column.reader.decoder.json)

## ColumnValueDecoderImpl

`struct` · `parquet::column::reader::decoder::ColumnValueDecoderImpl`

```rust
struct ColumnValueDecoderImpl<T: DataType>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.reader.decoder.ColumnValueDecoderImpl.md).


An implementation of [`ColumnValueDecoder`] for `[T::T]`

---

## DefinitionLevelDecoderImpl

`struct` · `parquet::column::reader::decoder::DefinitionLevelDecoderImpl`

```rust
struct DefinitionLevelDecoderImpl
```

[Full member, field, variant and typed contracts](../operations/parquet.column.reader.decoder.DefinitionLevelDecoderImpl.md).


An implementation of [`DefinitionLevelDecoder`] for `[i16]`

---

## RepetitionLevelDecoderImpl

`struct` · `parquet::column::reader::decoder::RepetitionLevelDecoderImpl`

```rust
struct RepetitionLevelDecoderImpl
```

[Full member, field, variant and typed contracts](../operations/parquet.column.reader.decoder.RepetitionLevelDecoderImpl.md).


An implementation of [`RepetitionLevelDecoder`] for `[i16]`

---

## ColumnLevelDecoder

`trait` · `parquet::column::reader::decoder::ColumnLevelDecoder`

```rust
trait ColumnLevelDecoder
```

**Methods** (1)

```rust
fn set_data(&mut self, encoding: Encoding, data: Bytes) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.reader.decoder.ColumnLevelDecoder.md).


Decodes level data

---

## ColumnValueDecoder

`trait` · `parquet::column::reader::decoder::ColumnValueDecoder`

```rust
trait ColumnValueDecoder
```

**Methods** (5)

```rust
fn new(col: &ColumnDescPtr) -> Self
fn read(&mut self, out: &mut Self::Buffer, num_values: usize) -> Result<usize>
fn set_data(&mut self, encoding: Encoding, data: Bytes, num_levels: usize, num_values: Option<usize>) -> Result<()>
fn set_dict(&mut self, buf: Bytes, num_values: u32, encoding: Encoding, is_sorted: bool) -> Result<()>
fn skip_values(&mut self, num_values: usize) -> Result<usize>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.reader.decoder.ColumnValueDecoder.md).


Decodes value data

---

## DefinitionLevelDecoder

`trait` · `parquet::column::reader::decoder::DefinitionLevelDecoder`

```rust
trait DefinitionLevelDecoder: ColumnLevelDecoder
```

**Methods** (2)

```rust
fn read_def_levels(&mut self, out: &mut Self::Buffer, num_levels: usize) -> Result<(usize, usize)>
fn skip_def_levels(&mut self, num_levels: usize) -> Result<(usize, usize)>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.reader.decoder.DefinitionLevelDecoder.md).


---

## RepetitionLevelDecoder

`trait` · `parquet::column::reader::decoder::RepetitionLevelDecoder`

```rust
trait RepetitionLevelDecoder: ColumnLevelDecoder
```

**Methods** (3)

```rust
fn flush_partial(&mut self) -> bool
fn read_rep_levels(&mut self, out: &mut Self::Buffer, num_records: usize, num_levels: usize) -> Result<(usize, usize)>
fn skip_rep_levels(&mut self, num_records: usize, num_levels: usize) -> Result<(usize, usize)>
```

[Full member, field, variant and typed contracts](../operations/parquet.column.reader.decoder.RepetitionLevelDecoder.md).


---
