# `deltalake_core::datafile`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.json).

<a id="op-2989b24ba9c1bf9902ce82e8"></a>
## datafile

`module` · `deltalake_core::datafile` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod datafile
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L1).

Source: `crates/core/src/datafile/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Data-file read/write abstractions, in two tiers:

* **File tier** ([`DataFileWriter`](../operations/deltalake_core.datafile.DataFileWriter.md#op-24cfc720bb0833876566d16a), [`DataFileReader`](../operations/deltalake_core.datafile.DataFileReader.md#op-128d8387951106ecba58a62f)) — the per-file
  seam where parquet `WriterProperties` attach.
  Impl: [`writer::PartitionWriter`](../operations/deltalake_core.datafile.writer.PartitionWriter.md#op-4307557303e40b4caa7d4a51).
* **Dataset tier** ([`DeltaDataWriter`](../operations/deltalake_core.datafile.DeltaDataWriter.md#op-c1f5bba8aa5bb8d203fc6776), [`DeltaDataReader`](../operations/deltalake_core.datafile.DeltaDataReader.md#op-3d4fa2249fef5fac72168f8d)) — composes the
  file tier across a table. Impl: [`writer::DeltaWriter`](../operations/deltalake_core.datafile.writer.DeltaWriter.md#op-93547c21542fa50fb0da1dfb).

Both tiers operate on a DataFusion-free fallible stream of [`RecordBatch`]es.
The DataFusion-capable surface lives in the gated [`datafusion_ext`](../modules/deltalake_core.datafile.datafusion_ext.md#op-f359eba9d0d6ea6ae21ed7fc) module.

Unresolved upstream links (retained, not inferred): ``RecordBatch``.
