# `buoyant_kernel::crc::file_stats`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.file_stats.json).

<a id="op-3194afc22ae3609a7b0a92fe"></a>
## file_stats

`module` · `buoyant_kernel::crc::file_stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod file_stats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File statistics and deltas for CRC tracking.

[`FileStats`](../operations/buoyant_kernel.crc.file_stats.FileStats.md#op-0ae222f09a8ef0e02c9e6fa6) represents absolute file-level statistics (count, size, histogram) for a table
version. [`FileStatsDelta`] captures the net changes from a single commit as a single delta
[`FileSizeHistogram`](../operations/buoyant_kernel.crc.file_size_histogram.FileSizeHistogram.md#op-c8fc21baeab039dee041e99d) (adds minus removes).

[`FileStatsDelta`] captures how many files were added/removed and their total sizes. It can be
produced from either:
1. In-memory transaction data via [`FileStatsDelta::try_compute_for_txn`]
2. A parsed .json commit file

Unresolved upstream links (retained, not inferred): ``FileStatsDelta::try_compute_for_txn``, ``FileStatsDelta``.
