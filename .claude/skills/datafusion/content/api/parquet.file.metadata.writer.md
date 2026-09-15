# `parquet::file::metadata::writer`

Crate `parquet` · 1 public items · structured records in [`model/parquet.file.metadata.writer.json`](../model/parquet.file.metadata.writer.json)

## ParquetMetaDataWriter

`struct` · `parquet::file::metadata::writer::ParquetMetaDataWriter`

Also reachable as `parquet::file::metadata::ParquetMetaDataWriter`

```rust
struct ParquetMetaDataWriter<'a, W: Write>
```

**Methods** (4)

```rust
fn finish(self) -> Result<()>
fn new(buf: W, metadata: &'a ParquetMetaData) -> Self
fn new_with_tracked(buf: TrackedWrite<W>, metadata: &'a ParquetMetaData) -> Self
fn with_write_path_in_schema(self, val: bool) -> Self
```

Writes [`ParquetMetaData`] to a byte stream

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

---
