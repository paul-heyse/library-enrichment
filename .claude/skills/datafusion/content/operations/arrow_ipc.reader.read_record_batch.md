# `arrow_ipc::reader::read_record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.read_record_batch.json).

<a id="op-4fe8b97ad9bab2236d7435f5"></a>
## read_record_batch

`function` · `arrow_ipc::reader::read_record_batch` · arrow-ipc 59.3.0

```rust
fn read_record_batch(buf: &arrow_buffer::Buffer, batch: RecordBatch<'_>, schema: SchemaRef, dictionaries_by_id: &std::collections::HashMap<i64, ArrayRef>, projection: Option<&[usize]>, metadata: &MetadataVersion) -> Result<RecordBatch, ArrowError>
```

Source: `src/reader.rs:767`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Creates a record batch from binary data using the `crate::RecordBatch` indexes and the `Schema`.

If `require_alignment` is true, this function will return an error if any array data in the
input `buf` is not properly aligned.
Under the hood it will use [`arrow_data::ArrayDataBuilder::build`] to construct [`arrow_data::ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

If `require_alignment` is false, this function will automatically allocate a new aligned buffer
and copy over the data if any array data in the input `buf` is not properly aligned.
(Properly aligned array data will remain zero-copy.)
Under the hood it will use [`arrow_data::ArrayDataBuilder::align_buffers`] to construct [`arrow_data::ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

Unresolved upstream links (retained, not inferred): ``arrow_data::ArrayDataBuilder::build``, ``arrow_data::ArrayDataBuilder::align_buffers``.
