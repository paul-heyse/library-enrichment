# `deltalake_core::datafile::DataFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.DataFileReader.json).

<a id="op-128d8387951106ecba58a62f"></a>
## DataFileReader

`trait` · `deltalake_core::datafile::DataFileReader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DataFileReader: Send + Sync
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L66).

Source: `crates/core/src/datafile/mod.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File tier: reads a single parquet data file (the per-file decryption seam,
mirroring [`DataFileWriter`](../operations/deltalake_core.datafile.DataFileWriter.md#op-24cfc720bb0833876566d16a)). Impl: [`reader::ParquetFileReader`](../operations/deltalake_core.datafile.reader.ParquetFileReader.md#op-7c43616e51bb54d462d7ce6a).

<a id="op-caf4d5f3c235a02642840ad1"></a>
## read_file

`function` · `deltalake_core::datafile::DataFileReader::read_file` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn read_file(&self, path: object_store::path::Path) -> DeltaResult<BatchStream>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L68).

Source: `crates/core/src/datafile/mod.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Read the parquet data file at `path` into a stream of record batches.
