# `arrow_array::record_batch::RecordBatchReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.record_batch.RecordBatchReader.json).

<a id="op-e728e9cf0c3686077413de28"></a>
## RecordBatchReader

`trait` · `arrow_array::record_batch::RecordBatchReader` · arrow-array 59.3.0

```rust
trait RecordBatchReader: Iterator<Item = Result<RecordBatch, arrow_schema::ArrowError>>
```

Source: `src/record_batch.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Trait for types that can read `RecordBatch`'s.

To create from an iterator, see [RecordBatchIterator](../operations/arrow_array.record_batch.RecordBatchIterator.md#op-d3fc556920f24888e8206871).

<a id="op-710930d6e99a6661965c0dbe"></a>
## schema

`function` · `arrow_array::record_batch::RecordBatchReader::schema` · arrow-array 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Source: `src/record_batch.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the schema of this `RecordBatchReader`.

Implementation of this trait should guarantee that all `RecordBatch`'s returned by this
reader should have the same schema as returned from this method.
