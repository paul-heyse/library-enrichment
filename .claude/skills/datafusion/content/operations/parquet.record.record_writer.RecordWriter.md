# `parquet::record::record_writer::RecordWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.record_writer.RecordWriter.json).

<a id="op-76b6c8053e0fcbea20b92e80"></a>
## RecordWriter

`trait` · `parquet::record::record_writer::RecordWriter` · parquet 59.3.0

```rust
trait RecordWriter<T>
```

Source: `src/record/record_writer.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Trait describing how to write a record (the implementator) to a row group writer.

[`parquet_derive`] crate provides a derive macro [`ParquetRecordWriter`] for this trait
for unnested structs.

The type parameter `T` is used to work around the rust orphan rule
when implementing on types such as `&[T]`.

[`parquet_derive`]: https://crates.io/crates/parquet_derive
[`ParquetRecordWriter`]: https://docs.rs/parquet_derive/53.0.0/parquet_derive/derive.ParquetRecordWriter.html

<a id="op-ae10a93954c814cfc0dd2e37"></a>
## schema

`function` · `parquet::record::record_writer::RecordWriter::schema` · parquet 59.3.0

```rust
fn schema(&self) -> Result<TypePtr, ParquetError>
```

Source: `src/record/record_writer.rs:41`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Generated schema used by `row_group_writer`

<a id="op-9dd2ef94a13506a3ff7f5a51"></a>
## write_to_row_group

`function` · `parquet::record::record_writer::RecordWriter::write_to_row_group` · parquet 59.3.0

```rust
fn write_to_row_group<W: std::io::Write + Send>(&self, row_group_writer: &mut SerializedRowGroupWriter<'_, W>) -> Result<(), ParquetError>
```

Source: `src/record/record_writer.rs:35`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writes from `self` into `row_group_writer`.
