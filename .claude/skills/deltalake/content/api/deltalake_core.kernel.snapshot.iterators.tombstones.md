# `deltalake_core::kernel::snapshot::iterators::tombstones`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.snapshot.iterators.tombstones.json`](../model/deltalake_core.kernel.snapshot.iterators.tombstones.json)

## TombstoneView

`struct` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.snapshot.iterators.tombstones.TombstoneView.md)

Also reachable as `deltalake::kernel::TombstoneView`, `deltalake_core::kernel::TombstoneView`

```rust
struct TombstoneView
```

**Derives**: Clone

**Methods** (4)

```rust
fn data_change(&self) -> bool
fn deletion_timestamp(&self) -> Option<i64>
fn path(&self) -> Cow<'_, str>
fn size(&self) -> Option<i64>
```

A lightweight, cloneable view over a single tombstone (`Remove` action) row.

Rather than materializing a `Remove` struct, this borrows into the backing
[`RecordBatch`] and decodes individual fields on demand.

---
