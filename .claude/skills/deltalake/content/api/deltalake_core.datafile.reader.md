# `deltalake_core::datafile::reader`

Crate `deltalake-core` · 4 public items · structured records in [`model/deltalake_core.datafile.reader.json`](../model/deltalake_core.datafile.reader.json)

## KernelDataFileReader

`struct` · `deltalake_core::datafile::reader::KernelDataFileReader`
[Full member contracts, output types and access classification](../operations/deltalake_core.datafile.reader.KernelDataFileReader.md)

Also reachable as `deltalake::datafile::reader::KernelDataFileReader`

```rust
struct KernelDataFileReader
```

**Implements**: `deltalake_core::datafile::DataFileReader`

**Derives**: Clone, Debug, Default

**via `deltalake_core::datafile::DataFileReader`**

```rust
async fn read_file(&self, _path: Path) -> DeltaResult<BatchStream>
```

File-tier reader backed by `delta-kernel`'s parquet handler (placeholder).

---

## KernelDataReader

`struct` · `deltalake_core::datafile::reader::KernelDataReader`
[Full member contracts, output types and access classification](../operations/deltalake_core.datafile.reader.KernelDataReader.md)

Also reachable as `deltalake::datafile::reader::KernelDataReader`

```rust
struct KernelDataReader
```

**Implements**: `deltalake_core::datafile::DeltaDataReader`

**Derives**: Clone, Debug, Default

**via `deltalake_core::datafile::DeltaDataReader`**

```rust
async fn read(&self, _options: ReadOptions) -> DeltaResult<BatchStream>
```

Dataset-tier reader backed by `delta-kernel`'s scan engine (placeholder).

Unlike [`ParquetTableReader`], this will apply deletion vectors, partition
values, and column-mapping transforms — the full Delta read semantics.

---

## ParquetFileReader

`struct` · `deltalake_core::datafile::reader::ParquetFileReader`
[Full member contracts, output types and access classification](../operations/deltalake_core.datafile.reader.ParquetFileReader.md)

Also reachable as `deltalake::datafile::reader::ParquetFileReader`

```rust
struct ParquetFileReader
```

**Implements**: `deltalake_core::datafile::DataFileReader`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(store: Arc<dyn ObjectStore>) -> Self
```

**via `deltalake_core::datafile::DataFileReader`**

```rust
async fn read_file(&self, path: Path) -> DeltaResult<BatchStream>
```

File-tier reader that reads a single parquet data file directly from object
storage, with no DataFusion. A concrete [`DataFileReader`] — the per-file read
seam where parquet decryption will later attach, mirroring the write side.

---

## ParquetTableReader

`struct` · `deltalake_core::datafile::reader::ParquetTableReader`
[Full member contracts, output types and access classification](../operations/deltalake_core.datafile.reader.ParquetTableReader.md)

Also reachable as `deltalake::datafile::reader::ParquetTableReader`

```rust
struct ParquetTableReader
```

**Implements**: `deltalake_core::datafile::DeltaDataReader`

**Derives**: Clone, Debug

**Methods** (1)

```rust
async fn try_new(table: &DeltaTable) -> DeltaResult<Self>
```

**via `deltalake_core::datafile::DeltaDataReader`**

```rust
async fn read(&self, options: ReadOptions) -> DeltaResult<BatchStream>
```

Dataset-tier reader that reads all of a table's parquet data files directly —
no DataFusion, no predicate — composing [`ParquetFileReader`] across the table.

Minimal by design: it reads raw parquet, so [`ParquetTableReader::try_new`]
rejects tables that use deletion vectors, column mapping, or partition columns
(those are the kernel-backed [`KernelDataReader`]'s job), and it does not unify
schemas across files — returned batches reflect each file's physical schema as
written. On a table whose schema evolved, older and newer files yield batches
with differing schemas, and the caller must reconcile them (e.g. cast to a
common schema before `concat`).

---
