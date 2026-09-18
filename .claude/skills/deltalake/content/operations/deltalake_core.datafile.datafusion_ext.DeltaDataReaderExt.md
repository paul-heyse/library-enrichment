# `deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.datafusion_ext.DeltaDataReaderExt.json).

<a id="op-1bf8e4be6ddc21b15f13b89a"></a>
## DeltaDataReaderExt

`trait` · `deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DeltaDataReaderExt: DeltaDataReader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L87).

Source: `crates/core/src/datafile/datafusion_ext.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

DataFusion extension to [`DeltaDataReader`](../operations/deltalake_core.datafile.DeltaDataReader.md#op-3d4fa2249fef5fac72168f8d): a full scan with pushdown.

<a id="op-989fc8799501cd6b1024c47d"></a>
## scan

`function` · `deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt::scan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn scan(&self, session: &dyn Session, options: ScanOptions) -> DeltaResult<SendableRecordBatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L98).

Source: `crates/core/src/datafile/datafusion_ext.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Scan the table through the DataFusion `DeltaScanNext` provider, returning
a coalesced single-partition stream.

`session` must have the table's object store registered — for
[`DataFusionDataReader`](../operations/deltalake_core.datafile.datafusion_ext.DataFusionDataReader.md#op-ec999f6bbf3a136a67732883), the session passed to
[`try_new`](DataFusionDataReader::try_new) (which registers it) or one
sharing that session's `RuntimeEnv`. A reader built via
[`new`](DataFusionDataReader::new) registers nothing, and scanning with
an unrelated session fails at execution with an
"object store not registered" error.
