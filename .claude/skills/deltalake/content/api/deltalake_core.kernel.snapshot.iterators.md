# `deltalake_core::kernel::snapshot::iterators`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.snapshot.iterators.json`](../model/deltalake_core.kernel.snapshot.iterators.json)

## LogicalFileView

`struct` · `deltalake_core::kernel::snapshot::iterators::LogicalFileView`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.snapshot.iterators.LogicalFileView.md)

Also reachable as `deltalake::kernel::LogicalFileView`, `deltalake_core::kernel::LogicalFileView`

```rust
struct LogicalFileView
```

**Derives**: Clone

**Methods** (15)

```rust
fn add_action(&self) -> Add
fn deletion_vector_descriptor(&self) -> Option<DeletionVectorDescriptor>
fn max_values(&self) -> Option<Scalar>
fn min_values(&self) -> Option<Scalar>
fn modification_datetime(&self) -> DeltaResult<chrono::DateTime<Utc>>
fn modification_time(&self) -> i64
fn null_counts(&self) -> Option<Scalar>
fn num_records(&self) -> Option<usize>
fn object_store_path(&self) -> Path
fn partition_values(&self) -> Option<StructData>
fn partition_values_map(&self) -> HashMap<String, Option<String>>
fn path(&self) -> Cow<'_, str>
fn remove_action(&self, data_change: bool) -> Remove
fn size(&self) -> i64
fn stats(&self) -> Option<String>
```

Provides semantic, typed access to file metadata from Delta log replay.

This struct wraps a RecordBatch containing file data and provides zero-copy
access to individual file entries through an index. It serves as a view into
the kernel's log replay results, offering convenient methods to extract
file properties without unnecessary data copies.

---
