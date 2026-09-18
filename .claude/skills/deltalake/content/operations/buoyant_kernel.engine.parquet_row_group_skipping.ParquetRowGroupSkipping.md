# `buoyant_kernel::engine::parquet_row_group_skipping::ParquetRowGroupSkipping`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.parquet_row_group_skipping.ParquetRowGroupSkipping.json).

<a id="op-99bd77cc403e60524c1b1f61"></a>
## ParquetRowGroupSkipping

`trait` · `buoyant_kernel::engine::parquet_row_group_skipping::ParquetRowGroupSkipping` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ParquetRowGroupSkipping
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/parquet_row_group_skipping.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/parquet_row_group_skipping.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An extension trait for [`ArrowReaderBuilder`] that injects row group skipping capability.

Unresolved upstream links (retained, not inferred): ``ArrowReaderBuilder``.

<a id="op-8560f01e0421c7243e1c1a29"></a>
## with_checkpoint_row_group_filter

`function` · `buoyant_kernel::engine::parquet_row_group_skipping::ParquetRowGroupSkipping::with_checkpoint_row_group_filter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_checkpoint_row_group_filter(self, predicate: &Predicate, partition_columns: &HashSet<String>, row_indexes: Option<&mut RowIndexBuilder>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/parquet_row_group_skipping.rs#L48).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/parquet_row_group_skipping.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Like [`with_row_group_filter`](Self::with_row_group_filter), but for checkpoint and sidecar
parquet files where statistics are nested under `add.stats_parsed.*` and partition values
under `add.partitionValues_parsed.*`.

The `predicate` uses physical column names (e.g. `x > 10`, or `col-abc-123 > 10` under
column mapping), and the filter internally maps them to the checkpoint's nested stats
schema layout.
Statistics for data columns are null-guarded: if a stat column contains any null values
in the row group (indicating some files lack that statistic), the stat is treated as
unavailable to prevent false pruning.

<a id="op-f67c7bff769658f0bea65791"></a>
## with_row_group_filter

`function` · `buoyant_kernel::engine::parquet_row_group_skipping::ParquetRowGroupSkipping::with_row_group_filter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_row_group_filter(self, predicate: &Predicate, row_indexes: Option<&mut RowIndexBuilder>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/parquet_row_group_skipping.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/parquet_row_group_skipping.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Instructs the parquet reader to perform row group skipping, eliminating any row group whose
stats prove that none of the group's rows can satisfy the given `predicate`.

If a [`RowIndexBuilder`](../operations/buoyant_kernel.engine.arrow_utils.RowIndexBuilder.md#op-84adf2e6bdead3539ea4265f) is provided, it will be updated to only include row indices of the
row groups that survived the filter.
