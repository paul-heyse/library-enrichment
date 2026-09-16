# `buoyant_kernel::crc::file_stats`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.crc.file_stats.json`](../model/buoyant_kernel.crc.file_stats.json)

## FileStats

`struct` · `buoyant_kernel::crc::file_stats::FileStats`

Also reachable as `buoyant_kernel::FileStats`, `buoyant_kernel::crc::FileStats`, `delta_kernel::crc::file_stats::FileStats`

```rust
struct FileStats
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn file_size_histogram(&self) -> Option<&FileSizeHistogram>
fn num_files(&self) -> i64
fn table_size_bytes(&self) -> i64
```

File-level statistics for a table version: total file count, size, and histogram.

Obtained via [`Snapshot::get_file_stats_if_present`] or [`Crc::file_stats()`]. Returns
`None` when the source CRC's `file_stats_state` is not `Complete`.

[`Snapshot::get_file_stats_if_present`]: crate::snapshot::Snapshot::get_file_stats_if_present
[`Crc::file_stats()`]: super::Crc::file_stats

---
