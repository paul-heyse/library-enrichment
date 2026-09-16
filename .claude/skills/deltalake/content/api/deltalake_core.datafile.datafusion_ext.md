# `deltalake_core::datafile::datafusion_ext`

Crate `deltalake-core` · 4 public items · structured records in [`model/deltalake_core.datafile.datafusion_ext.json`](../model/deltalake_core.datafile.datafusion_ext.json)

## DataFusionDataReader

`struct` · `deltalake_core::datafile::datafusion_ext::DataFusionDataReader`

Also reachable as `deltalake::datafile::datafusion_ext::DataFusionDataReader`

```rust
struct DataFusionDataReader
```

**Implements**: `deltalake_core::datafile::DeltaDataReader`, `deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt`

**Methods** (2)

```rust
fn new(provider: Arc<dyn TableProvider>, session: Arc<dyn Session>) -> Self
async fn try_new(table: &DeltaTable, session: Arc<dyn Session>) -> DeltaResult<Self>
```

**via `deltalake_core::datafile::DeltaDataReader`**

```rust
async fn read(&self, options: ReadOptions) -> DeltaResult<BatchStream>
```

**via `deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt`**

```rust
async fn scan(&self, session: &dyn Session, options: ScanOptions) -> DeltaResult<SendableRecordBatchStream>
```

A DataFusion-backed reader wrapping the existing `DeltaScanNext` provider.
It carries its own session so it can also satisfy [`DeltaDataReader`].

---

## ScanOptions

`struct` · `deltalake_core::datafile::datafusion_ext::ScanOptions`

Also reachable as `deltalake::datafile::datafusion_ext::ScanOptions`

```rust
struct ScanOptions
```

**Fields**: `projection`, `limit`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default

**via `core::convert::From`**

```rust
fn from(value: ReadOptions) -> Self
```

Options controlling a DataFusion-backed scan.

---

## DeltaDataReaderExt

`trait` · `deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt`

Also reachable as `deltalake::datafile::datafusion_ext::DeltaDataReaderExt`

```rust
trait DeltaDataReaderExt: DeltaDataReader
```

**Implementors** (1)

- `deltalake_core::datafile::datafusion_ext::DataFusionDataReader`

**Methods** (1)

```rust
async fn scan(&self, session: &dyn Session, options: ScanOptions) -> DeltaResult<SendableRecordBatchStream>
```

DataFusion extension to [`DeltaDataReader`]: a full scan with pushdown.

---

## DeltaDataWriterExt

`trait` · `deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt`

Also reachable as `deltalake::datafile::datafusion_ext::DeltaDataWriterExt`

```rust
trait DeltaDataWriterExt
```

**Implementors** (1)

- `deltalake_core::datafile::writer::DeltaWriter`

**Methods** (1)

```rust
async fn write_plan(Box<self>, session: &dyn Session, plan: Arc<dyn ExecutionPlan>) -> DeltaResult<Vec<Add>>
```

DataFusion extension to [`DeltaDataWriter`]: write the output of an execution plan.

---
