# `parquet::record::record_reader`

Crate `parquet` · 1 public items · structured records in [`model/parquet.record.record_reader.json`](../model/parquet.record.record_reader.json)

## RecordReader

`trait` · `parquet::record::record_reader::RecordReader`

Also reachable as `parquet::record::RecordReader`

```rust
trait RecordReader<T>
```

**Methods** (1)

```rust
fn read_from_row_group(&mut self, row_group_reader: &mut dyn RowGroupReader, num_records: usize) -> Result<(), ParquetError>
```

Read up to `num_records` records from `row_group_reader` into `self`.

The type parameter `T` is used to work around the rust orphan rule
when implementing on types such as `Vec<T>`.

---
