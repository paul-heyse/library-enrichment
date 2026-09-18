# `deltalake_core::kernel::snapshot::scan`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.kernel.snapshot.scan.json`](../model/deltalake_core.kernel.snapshot.scan.json)

## Scan

`struct` · `deltalake_core::kernel::snapshot::scan::Scan`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.snapshot.scan.Scan.md)

Also reachable as `deltalake::kernel::Scan`, `deltalake_core::kernel::Scan`

```rust
struct Scan
```

**Implements**: `core::convert::From`

**Derives**: Debug

**Methods** (7)

```rust
fn logical_schema(&self) -> &SchemaRef
fn physical_predicate(&self) -> Option<PredicateRef>
fn physical_schema(&self) -> &SchemaRef
fn scan_metadata(&self, engine: Arc<dyn Engine>) -> SendableScanMetadataStream
fn scan_metadata_from<T: Iterator<Item = RecordBatch> + Send + 'static>(&self, engine: Arc<dyn Engine>, existing_version: Version, existing_data: Box<T>, existing_predicate: Option<PredicateRef>) -> SendableScanMetadataStream
fn snapshot(&self) -> &SnapshotRef
fn table_root(&self) -> &Url
```

**via `core::convert::From`**

```rust
fn from(inner: Arc<KernelScan>) -> Self
fn from(inner: KernelScan) -> Self
```

A configured, executable scan over a table snapshot.

Produced by [`ScanBuilder::build`]; drives log replay to enumerate the data files (and the
statistics materialization strategy) that satisfy the scan's schema and predicate.

---

## ScanBuilder

`struct` · `deltalake_core::kernel::snapshot::scan::ScanBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.snapshot.scan.ScanBuilder.md)

Also reachable as `deltalake::kernel::ScanBuilder`, `deltalake_core::kernel::ScanBuilder`

```rust
struct ScanBuilder
```

**Derives**: Debug

**Methods** (7)

```rust
fn build(self) -> DeltaResult<Scan>
fn new(snapshot: impl Into<Arc<KernelSnapshot>>) -> Self
fn with_kernel_all_struct_stats(self) -> Self
fn with_predicate(self, predicate: impl Into<Option<PredicateRef>>) -> Self
fn with_schema(self, schema: SchemaRef) -> Self
fn with_schema_opt(self, schema_opt: Option<SchemaRef>) -> Self
fn with_skip_stats(self, skip_stats: bool) -> Self
```

Builder to scan a snapshot of a table.

---

## SendableScanMetadataStream

`type_alias` · `deltalake_core::kernel::snapshot::scan::SendableScanMetadataStream`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.snapshot.scan.SendableScanMetadataStream.md)

Also reachable as `deltalake::kernel::SendableScanMetadataStream`, `deltalake_core::kernel::SendableScanMetadataStream`

```rust
type SendableScanMetadataStream = std::pin::Pin<Box<dyn Stream<Item = DeltaResult<delta_kernel::scan::ScanMetadata>> + Send>>
```

A boxed, `Send`able stream of [`ScanMetadata`] results produced while scanning a snapshot.

---
