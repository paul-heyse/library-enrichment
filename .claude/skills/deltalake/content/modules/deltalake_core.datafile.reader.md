# `deltalake_core::datafile::reader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.reader.json).

<a id="op-a8292f50446b2f0d91719f03"></a>
## reader

`module` · `deltalake_core::datafile::reader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod reader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/reader.rs#L1).

Source: `crates/core/src/datafile/reader.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

DataFusion-free data-file readers (file and dataset tiers).

This module hosts two kinds of reader:

* [`ParquetFileReader`](../operations/deltalake_core.datafile.reader.ParquetFileReader.md#op-7c43616e51bb54d462d7ce6a) / [`ParquetTableReader`](../operations/deltalake_core.datafile.reader.ParquetTableReader.md#op-0befbd1bf96f43c3cfdb22bf) — the first concrete
  implementations: they read raw parquet directly from object storage with no
  DataFusion, and are intentionally minimal, rejecting tables that need
  deletion-vector application, column mapping, or partition-value reconstruction.
* [`KernelDataFileReader`](../operations/deltalake_core.datafile.reader.KernelDataFileReader.md#op-fec287867c89e5c2414ac5f7) / [`KernelDataReader`](../operations/deltalake_core.datafile.reader.KernelDataReader.md#op-8db980494e2cd454b717a106) — placeholders for the
  later, full-fidelity reader backed by `delta-kernel`'s scan engine (which
  applies deletion vectors, partition values, and column-mapping transforms).

In a `datafusion` build, full reads go through
[`crate::datafile::datafusion_ext::DeltaDataReaderExt`](../operations/deltalake_core.datafile.datafusion_ext.DeltaDataReaderExt.md#op-1bf8e4be6ddc21b15f13b89a).
