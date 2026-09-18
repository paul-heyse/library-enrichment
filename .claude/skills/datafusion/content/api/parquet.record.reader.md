# `parquet::record::reader`

Crate `parquet` · 4 public items · structured records in [`model/parquet.record.reader.json`](../model/parquet.record.reader.json)

## Reader

`enum` · `parquet::record::reader::Reader`

```rust
enum Reader
```

**Variants**: `PrimitiveReader`, `OptionReader`, `GroupReader`, `RepeatedReader`, `KeyValueReader`

**Implements**: `core::fmt::Display`

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/parquet.record.reader.Reader.md).


Reader tree for record assembly

---

## ReaderIter

`struct` · `parquet::record::reader::ReaderIter`

```rust
struct ReaderIter
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Row>>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.reader.ReaderIter.md).


Internal iterator of [`Row`]s for a reader.

---

## RowIter

`struct` · `parquet::record::reader::RowIter`

```rust
struct RowIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (5)

```rust
fn from_file(proj: Option<Type>, reader: &'a dyn FileReader) -> Result<Self>
fn from_file_into(reader: Box<dyn FileReader>) -> Self
fn from_row_group(proj: Option<Type>, reader: &'a dyn RowGroupReader) -> Result<Self>
fn project(self, proj: Option<Type>) -> Result<Self>
fn with_batch_size(self, batch_size: usize) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Row>>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.reader.RowIter.md).


Access parquet data as an iterator of [`Row`]

# Caveats

Parquet stores data in a columnar fashion using [Dremel] encoding, and is therefore highly
optimised for reading data by column, not row. As a consequence applications concerned with
performance should prefer the columnar arrow or [ColumnReader] APIs.

Additionally the current implementation does not correctly handle repeated fields ([#2394]),
and workloads looking to handle such schema should use the other APIs.

[#2394]: https://github.com/apache/arrow-rs/issues/2394
[ColumnReader]: crate::file::reader::RowGroupReader::get_column_reader
[Dremel]: https://research.google/pubs/pub36632/

---

## TreeBuilder

`struct` · `parquet::record::reader::TreeBuilder`

```rust
struct TreeBuilder
```

**Derives**: Default

**Methods** (4)

```rust
fn as_iter(&self, descr: SchemaDescPtr, row_group_reader: &dyn RowGroupReader) -> Result<ReaderIter>
fn build(&self, descr: SchemaDescPtr, row_group_reader: &dyn RowGroupReader) -> Result<Reader>
fn new() -> Self
fn with_batch_size(self, batch_size: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.record.reader.TreeBuilder.md).


Tree builder for `Reader` enum.
Serves as a container of options for building a reader tree and a builder, and
accessing a records iterator [`RowIter`].

---
