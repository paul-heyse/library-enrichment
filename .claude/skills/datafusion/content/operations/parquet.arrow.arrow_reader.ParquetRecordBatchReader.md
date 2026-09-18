# `parquet::arrow::arrow_reader::ParquetRecordBatchReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.ParquetRecordBatchReader.json).

<a id="op-d346feb61c9116e17f0cd9c3"></a>
## ParquetRecordBatchReader

`struct` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader` · parquet 59.3.0

```rust
struct ParquetRecordBatchReader
```

Source: `src/arrow/arrow_reader/mod.rs:1388`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reads Parquet data as Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es

This struct implements the [`RecordBatchReader`](../operations/arrow_array.record_batch.RecordBatchReader.md#op-e728e9cf0c3686077413de28) trait and is an
`Iterator<Item = ArrowResult<RecordBatch>>` that yields [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es.

Typically, either reads from a file or an in memory buffer [`Bytes`]

Created by [`ParquetRecordBatchReaderBuilder`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.md#op-1638827d919dceda05d4f2dd)

[`Bytes`]: bytes::Bytes

Unresolved upstream links (retained, not inferred): `bytes::Bytes`.

<a id="op-b0e5aa593b6c975587ab6a28"></a>
## Item

`assoc_type` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ParquetRecordBatchReader", "path": "ParquetRecordBatchReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1538, 1], "end": [1546, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/arrow/arrow_reader/mod.rs:1539`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04f61789892d6f78fb355c2c"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ParquetRecordBatchReader", "path": "ParquetRecordBatchReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1528, 1], "end": [1536, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/mod.rs:1529`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61a38c5bdd65d0fbef87037d"></a>
## next

`function` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader::next` · parquet 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ParquetRecordBatchReader", "path": "ParquetRecordBatchReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1538, 1], "end": [1546, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/arrow/arrow_reader/mod.rs:1541`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b2823d70f20ab79684c25c0"></a>
## schema

`function` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader::schema` · parquet 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ParquetRecordBatchReader", "path": "ParquetRecordBatchReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1617, 1], "end": [1625, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/arrow/arrow_reader/mod.rs:1622`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the projected [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a) for reading the parquet file.

Note that the schema metadata will be stripped here. See
[`ParquetRecordBatchReaderBuilder::schema`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-c408c3639fc9a3227bf40394) if the metadata is desired.

<a id="op-3e86d07eadd9fe7610aeaee2"></a>
## try_new

`function` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader::try_new` · parquet 59.3.0

```rust
fn try_new<T: ChunkReader + 'static>(reader: T, batch_size: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ParquetRecordBatchReader", "path": "ParquetRecordBatchReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1685, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1631`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3) from the provided chunk reader

See [`ParquetRecordBatchReaderBuilder`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.md#op-1638827d919dceda05d4f2dd) for more options

<a id="op-bfb3bedd7b81e1356eff4954"></a>
## try_new_with_row_groups

`function` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader::try_new_with_row_groups` · parquet 59.3.0

```rust
fn try_new_with_row_groups(levels: &FieldLevels, row_groups: &dyn RowGroups, batch_size: usize, selection: Option<RowSelection>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ParquetRecordBatchReader", "path": "ParquetRecordBatchReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1685, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1641`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3) from the provided [`RowGroups`]

Note: this is a low-level interface see [`ParquetRecordBatchReader::try_new`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-3e86d07eadd9fe7610aeaee2) for a
higher-level interface for reading parquet data from a file

Unresolved upstream links (retained, not inferred): ``RowGroups``.
