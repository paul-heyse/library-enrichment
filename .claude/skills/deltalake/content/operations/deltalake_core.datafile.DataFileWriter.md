# `deltalake_core::datafile::DataFileWriter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.DataFileWriter.json).

<a id="op-24cfc720bb0833876566d16a"></a>
## DataFileWriter

`trait` · `deltalake_core::datafile::DataFileWriter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DataFileWriter: Send
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L47).

Source: `crates/core/src/datafile/mod.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File tier: writes a single Delta data file (or size-split set for one
partition). The per-file seam where parquet `WriterProperties`/encryption
attach. Impl: [`writer::PartitionWriter`](../operations/deltalake_core.datafile.writer.PartitionWriter.md#op-4307557303e40b4caa7d4a51).

<a id="op-c3608e7d82659fb9ec55b231"></a>
## abort

`function` · `deltalake_core::datafile::DataFileWriter::abort` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn abort(Box<self>) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L60).

Source: `crates/core/src/datafile/mod.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Abandon the writer after an error, aborting its in-progress multipart
uploads. Dropping the writer instead leaks upload parts vacuum cannot
see, and `close`-ing it finalizes files for partially-written data;
error-path callers should use this.

<a id="op-861c722ed8e8ac0558affd5a"></a>
## close

`function` · `deltalake_core::datafile::DataFileWriter::close` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn close(Box<self>) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L54).

Source: `crates/core/src/datafile/mod.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Finish writing and return the uncommitted [`Add`](../operations/deltalake_core.kernel.models.actions.Add.md#op-165aeba4e3bd6a2b7f853b9a) actions for the files
that were produced.

<a id="op-1abab61f1a52810ee2fccf01"></a>
## write

`function` · `deltalake_core::datafile::DataFileWriter::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L50).

Source: `crates/core/src/datafile/mod.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Buffer a record batch, writing to one or more parquet files as needed.
The batch must match the writer's (partition-stripped) file schema.
