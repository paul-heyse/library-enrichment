# `deltalake_core::datafile::properties`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.properties.json).

<a id="op-3c3b63e943bbc0420a0cb6a5"></a>
## properties

`module` · `deltalake_core::datafile::properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod properties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/properties.rs#L1).

Source: `crates/core/src/datafile/properties.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Engine-agnostic Delta read configuration.

[`ReaderProperties`](../operations/deltalake_core.datafile.properties.ReaderProperties.md#op-5e5723840bb14ba7b2b674be) centralizes construction of DataFusion's
[`TableParquetOptions`](datafusion::config::TableParquetOptions) for Delta
scans, so read/parquet-IO config (future: per-file decryption) lives in one
place. Read-side counterpart to `WriterProperties`.

Unresolved upstream links (retained, not inferred): `datafusion::config::TableParquetOptions`.
