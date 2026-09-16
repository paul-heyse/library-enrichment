# `deltalake_core::datafile`

Crate `deltalake-core` · 6 public items · structured records in [`model/deltalake_core.datafile.json`](../model/deltalake_core.datafile.json)

## ReadOptions

`struct` · `deltalake_core::datafile::ReadOptions`

Also reachable as `deltalake::datafile::ReadOptions`

```rust
struct ReadOptions
```

**Fields**: `projection`, `limit`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn with_limit(self, limit: usize) -> Self
fn with_projection(self, projection: impl Into<Vec<String>>) -> Self
```

Options for a basic (DataFusion-free) read. Richer predicate/projection
pushdown is the DataFusion extension's job ([`datafusion_ext::DeltaDataReaderExt`]).

---

## DataFileReader

`trait` · `deltalake_core::datafile::DataFileReader`

Also reachable as `deltalake::datafile::DataFileReader`

```rust
trait DataFileReader: Send + Sync
```

**Implementors** (2)

- `deltalake_core::datafile::reader::KernelDataFileReader`
- `deltalake_core::datafile::reader::ParquetFileReader`

**Methods** (1)

```rust
async fn read_file(&self, path: object_store::path::Path) -> DeltaResult<BatchStream>
```

File tier: reads a single parquet data file (the per-file decryption seam,
mirroring [`DataFileWriter`]). Impl: [`reader::ParquetFileReader`].

---

## DataFileWriter

`trait` · `deltalake_core::datafile::DataFileWriter`

Also reachable as `deltalake::datafile::DataFileWriter`

```rust
trait DataFileWriter: Send
```

**Implementors** (1)

- `deltalake_core::datafile::writer::PartitionWriter`

**Methods** (3)

```rust
async fn abort(Box<self>) -> DeltaResult<()>
async fn close(Box<self>) -> DeltaResult<Vec<Add>>
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

File tier: writes a single Delta data file (or size-split set for one
partition). The per-file seam where parquet `WriterProperties`/encryption
attach. Impl: [`writer::PartitionWriter`].

---

## DeltaDataReader

`trait` · `deltalake_core::datafile::DeltaDataReader`

Also reachable as `deltalake::datafile::DeltaDataReader`

```rust
trait DeltaDataReader: Send + Sync
```

**Implementors** (3)

- `deltalake_core::datafile::datafusion_ext::DataFusionDataReader`
- `deltalake_core::datafile::reader::KernelDataReader`
- `deltalake_core::datafile::reader::ParquetTableReader`

**Methods** (1)

```rust
async fn read(&self, options: ReadOptions) -> DeltaResult<BatchStream>
```

Dataset tier: a DataFusion-free reader that composes the file tier
([`DataFileReader`]) across a table's data files, applying deletion
vectors, partition values, and column-mapping transforms.

---

## DeltaDataWriter

`trait` · `deltalake_core::datafile::DeltaDataWriter`

Also reachable as `deltalake::datafile::DeltaDataWriter`

```rust
trait DeltaDataWriter: Send
```

**Implementors** (1)

- `deltalake_core::datafile::writer::DeltaWriter`

**Methods** (1)

```rust
async fn write_all(Box<self>, batches: BatchStream) -> DeltaResult<Vec<Add>>
```

Dataset tier: a DataFusion-free writer that drains a batch stream into a
table's data files (partitioning and composing a [`DataFileWriter`] per
partition). Batches must already conform to the table schema and constraints
(callers on the basic path validate themselves).

---

## BatchStream

`type_alias` · `deltalake_core::datafile::BatchStream`

Also reachable as `deltalake::datafile::BatchStream`

```rust
type BatchStream = futures::stream::BoxStream<'static, errors::DeltaResult<arrow_array::RecordBatch>>
```

A fallible stream of [`RecordBatch`]es — the common currency of both tiers.
Producers that parallelize (concurrent file opens, partitioned scans) do so
upstream and merge into this stream.

---
