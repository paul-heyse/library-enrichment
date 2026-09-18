# `parquet::record::record_reader::RecordReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.record_reader.RecordReader.json).

<a id="op-01ece7ab2343618f0463233b"></a>
## RecordReader

`trait` · `parquet::record::record_reader::RecordReader` · parquet 59.3.0

```rust
trait RecordReader<T>
```

Source: `src/record/record_reader.rs:25`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read up to `num_records` records from `row_group_reader` into `self`.

The type parameter `T` is used to work around the rust orphan rule
when implementing on types such as `Vec<T>`.

<a id="op-eb6b1a728f60c0f89ee4c89d"></a>
## read_from_row_group

`function` · `parquet::record::record_reader::RecordReader::read_from_row_group` · parquet 59.3.0

```rust
fn read_from_row_group(&mut self, row_group_reader: &mut dyn RowGroupReader, num_records: usize) -> Result<(), ParquetError>
```

Source: `src/record/record_reader.rs:27`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read up to `num_records` records from `row_group_reader` into `self`.
