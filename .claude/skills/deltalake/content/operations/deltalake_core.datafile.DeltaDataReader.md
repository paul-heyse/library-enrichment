# `deltalake_core::datafile::DeltaDataReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.DeltaDataReader.json).

<a id="op-3d4fa2249fef5fac72168f8d"></a>
## DeltaDataReader

`trait` · `deltalake_core::datafile::DeltaDataReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DeltaDataReader: Send + Sync
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L110).

Source: `crates/core/src/datafile/mod.rs:110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Dataset tier: a DataFusion-free reader that composes the file tier
([`DataFileReader`](../operations/deltalake_core.datafile.DataFileReader.md#op-128d8387951106ecba58a62f)) across a table's data files, applying deletion
vectors, partition values, and column-mapping transforms.

<a id="op-b9a2d2d7e5d1fc0543355aac"></a>
## read

`function` · `deltalake_core::datafile::DeltaDataReader::read` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read(&self, options: ReadOptions) -> DeltaResult<BatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L112).

Source: `crates/core/src/datafile/mod.rs:112`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Read the selected data as a batch stream.
