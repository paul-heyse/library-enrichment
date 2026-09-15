# `parquet::column::writer::encoder`

Crate `parquet` · 5 public items · structured records in [`model/parquet.column.writer.encoder.json`](../model/parquet.column.writer.encoder.json)

## ColumnValueEncoderImpl

`struct` · `parquet::column::writer::encoder::ColumnValueEncoderImpl`

```rust
struct ColumnValueEncoderImpl<T: DataType>
```

---

## DataPageValues

`struct` · `parquet::column::writer::encoder::DataPageValues`

```rust
struct DataPageValues<T>
```

**Fields**: `buf`, `num_values`, `encoding`, `min_value`, `max_value`, `variable_length_bytes`

The encoded values for a data page, with optional statistics

---

## DictionaryPage

`struct` · `parquet::column::writer::encoder::DictionaryPage`

```rust
struct DictionaryPage
```

**Fields**: `buf`, `num_values`, `is_sorted`

The encoded data for a dictionary page

---

## ColumnValueEncoder

`trait` · `parquet::column::writer::encoder::ColumnValueEncoder`

```rust
trait ColumnValueEncoder
```

**Methods** (15)

```rust
fn compresses_against_previous_value(&self) -> bool
fn count_values_within_byte_budget(_values: &Self::Values, _offset: usize, _len: usize, _byte_budget: usize) -> Option<usize>
fn count_values_within_byte_budget_gather(_values: &Self::Values, _indices: &[usize], _byte_budget: usize) -> Option<usize>
fn estimated_data_page_size(&self) -> usize
fn estimated_dict_page_size(&self) -> Option<usize>
fn estimated_memory_size(&self) -> usize
fn flush_bloom_filter(&mut self) -> Option<Sbbf>
fn flush_data_page(&mut self) -> Result<DataPageValues<Self::T>>
fn flush_dict_page(&mut self) -> Result<Option<DictionaryPage>>
fn flush_geospatial_statistics(&mut self) -> Option<Box<GeospatialStatistics>>
fn has_dictionary(&self) -> bool
fn num_values(&self) -> usize
fn try_new(descr: &ColumnDescPtr, props: &WriterProperties) -> Result<Self> where Self: Sized
fn write(&mut self, values: &Self::Values, offset: usize, len: usize) -> Result<()>
fn write_gather(&mut self, values: &Self::Values, indices: &[usize]) -> Result<()>
```

A generic encoder of [`ColumnValues`] to data and dictionary pages used by
[super::GenericColumnWriter`]

---

## ColumnValues

`trait` · `parquet::column::writer::encoder::ColumnValues`

```rust
trait ColumnValues
```

**Methods** (1)

```rust
fn len(&self) -> usize
```

A collection of [`ParquetValueType`] encoded by a [`ColumnValueEncoder`]

---
