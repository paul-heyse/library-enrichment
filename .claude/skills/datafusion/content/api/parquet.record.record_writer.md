# `parquet::record::record_writer`

Crate `parquet` · 1 public items · structured records in [`model/parquet.record.record_writer.json`](../model/parquet.record.record_writer.json)

## RecordWriter

`trait` · `parquet::record::record_writer::RecordWriter`

Also reachable as `parquet::record::RecordWriter`

```rust
trait RecordWriter<T>
```

**Methods** (2)

```rust
fn schema(&self) -> Result<TypePtr, ParquetError>
fn write_to_row_group<W: std::io::Write + Send>(&self, row_group_writer: &mut SerializedRowGroupWriter<'_, W>) -> Result<(), ParquetError>
```

Trait describing how to write a record (the implementator) to a row group writer.

[`parquet_derive`] crate provides a derive macro [`ParquetRecordWriter`] for this trait
for unnested structs.

The type parameter `T` is used to work around the rust orphan rule
when implementing on types such as `&[T]`.

[`parquet_derive`]: https://crates.io/crates/parquet_derive
[`ParquetRecordWriter`]: https://docs.rs/parquet_derive/53.0.0/parquet_derive/derive.ParquetRecordWriter.html

---
