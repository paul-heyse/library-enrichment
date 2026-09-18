# `parquet::file::metadata::writer::ParquetMetaDataWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.writer.ParquetMetaDataWriter.json).

<a id="op-897397dc43ac19f9d052fbd0"></a>
## ParquetMetaDataWriter

`struct` · `parquet::file::metadata::writer::ParquetMetaDataWriter` · parquet 59.3.0

```rust
struct ParquetMetaDataWriter<'a, W: Write>
```

Source: `src/file/metadata/writer.rs:419`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writes [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) to a byte stream

This structure handles the details of writing the various parts of Parquet
metadata into a byte stream. It is used to write the metadata into a parquet
file and can also write metadata into other locations (such as a store of
bytes).

# Discussion

The process of writing Parquet metadata is tricky because the
metadata is not stored as a single inline thrift structure. It can have
several "out of band" structures such as the [`OffsetIndex`] and
BloomFilters stored in separate structures whose locations are stored as
offsets from the beginning of the file.

Note: this writer does not directly write BloomFilters. In order to write
BloomFilters, write the bloom filters into the buffer before creating the
metadata writer. Then set the corresponding `bloom_filter_offset` and
`bloom_filter_length` on [`ColumnChunkMetaData`] passed to this writer.

# Output Format

The format of the metadata is as follows:

1. Optional [`ColumnIndex`] (thrift encoded)
2. Optional [`OffsetIndex`] (thrift encoded)
3. [`FileMetaData`] (thrift encoded)
4. Length of encoded `FileMetaData` (4 bytes, little endian)
5. Parquet Magic Bytes (4 bytes)

[`FileMetaData`]: https://github.com/apache/parquet-format/tree/master?tab=readme-ov-file#metadata
[`ColumnChunkMetaData`]: crate::file::metadata::ColumnChunkMetaData
[`ColumnIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`OffsetIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

```text
┌──────────────────────┐
│                      │
│         ...          │
│                      │
│┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐ │
│     ColumnIndex     ◀│─ ─ ─
││    (Optional)     │ │     │
│ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │
│┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐ │     │ FileMetadata
│     OffsetIndex      │       contains embedded
││    (Optional)     │◀┼ ─   │ offsets to
│ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │  │    ColumnIndex and
│╔═══════════════════╗ │     │ OffsetIndex
│║                   ║ │  │
│║                   ║ ┼ ─   │
│║   FileMetadata    ║ │
│║                   ║ ┼ ─ ─ ┘
│║                   ║ │
│╚═══════════════════╝ │
│┌───────────────────┐ │
││  metadata length  │ │ length of FileMetadata  (only)
│└───────────────────┘ │
│┌───────────────────┐ │
││      'PAR1'       │ │ Parquet Magic Bytes
│└───────────────────┘ │
└──────────────────────┘
     Output Buffer
```

# Example
```no_run
# use parquet::file::metadata::{ParquetMetaData, ParquetMetaDataWriter};
# fn get_metadata() -> ParquetMetaData { unimplemented!(); }
// write parquet metadata to an in-memory buffer
let mut buffer = vec![];
let metadata: ParquetMetaData = get_metadata();
let writer = ParquetMetaDataWriter::new(&mut buffer, &metadata);
// write the metadata to the buffer
writer.finish().unwrap();
assert!(!buffer.is_empty());
```

<a id="op-ba41393f81d0d04b861ad544"></a>
## finish

`function` · `parquet::file::metadata::writer::ParquetMetaDataWriter::finish` · parquet 59.3.0

```rust
fn finish(self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::metadata::writer::ParquetMetaDataWriter", "path": "ParquetMetaDataWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [533, 2], "filename": "src/file/metadata/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/writer.rs:461`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write the metadata to the buffer

<a id="op-e3458c9b1e32bf38ef951ae8"></a>
## new

`function` · `parquet::file::metadata::writer::ParquetMetaDataWriter::new` · parquet 59.3.0

```rust
fn new(buf: W, metadata: &'a ParquetMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::metadata::writer::ParquetMetaDataWriter", "path": "ParquetMetaDataWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [533, 2], "filename": "src/file/metadata/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/writer.rs:433`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `ParquetMetaDataWriter` to write to `buf`

Note any embedded offsets in the metadata will be written assuming the
metadata is at the start of the buffer. If the metadata is being written
to a location other than the start of the buffer, see [`Self::new_with_tracked`](../operations/parquet.file.metadata.writer.ParquetMetaDataWriter.md#op-0e519086f2e1a98216657507)

See example on the struct level documentation

<a id="op-0e519086f2e1a98216657507"></a>
## new_with_tracked

`function` · `parquet::file::metadata::writer::ParquetMetaDataWriter::new_with_tracked` · parquet 59.3.0

```rust
fn new_with_tracked(buf: TrackedWrite<W>, metadata: &'a ParquetMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::metadata::writer::ParquetMetaDataWriter", "path": "ParquetMetaDataWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [533, 2], "filename": "src/file/metadata/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/writer.rs:443`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new ParquetMetaDataWriter to write to `buf`

This method is used when the metadata is being written to a location other
than the start of the buffer.

See example on the struct level documentation

<a id="op-9d1141798bff4d33d49d5124"></a>
## with_write_path_in_schema

`function` · `parquet::file::metadata::writer::ParquetMetaDataWriter::with_write_path_in_schema` · parquet 59.3.0

```rust
fn with_write_path_in_schema(self, val: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::metadata::writer::ParquetMetaDataWriter", "path": "ParquetMetaDataWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [533, 2], "filename": "src/file/metadata/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/writer.rs:453`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set whether or not to write the `path_in_schema` field in the Thrift `ColumnMetaData`
struct.
