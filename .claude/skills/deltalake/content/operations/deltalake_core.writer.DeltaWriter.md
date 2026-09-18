# `deltalake_core::writer::DeltaWriter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.DeltaWriter.json).

<a id="op-664f2195f9de70fb9633c7ad"></a>
## DeltaWriter

`trait` · `deltalake_core::writer::DeltaWriter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DeltaWriter<T>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L175).

Source: `crates/core/src/writer/mod.rs:175`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trait for writing data to Delta tables

<a id="op-7bf4105d68f9669eeadcae40"></a>
## flush

`function` · `deltalake_core::writer::DeltaWriter::flush` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn flush(&mut self) -> Result<Vec<Add>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L184).

Source: `crates/core/src/writer/mod.rs:184`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Flush the internal write buffers to files in the delta table folder structure.
The corresponding delta [`Add`](../operations/deltalake_core.kernel.models.actions.Add.md#op-165aeba4e3bd6a2b7f853b9a) actions are returned and should be committed via a transaction.

<a id="op-85f06c0b7b6353f28c49881a"></a>
## flush_and_commit

`function` · `deltalake_core::writer::DeltaWriter::flush_and_commit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn flush_and_commit(&mut self, table: &mut DeltaTable) -> Result<Version, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L188).

Source: `crates/core/src/writer/mod.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Flush the internal write buffers to files in the delta table folder structure.
and commit the changes to the Delta log, creating a new table version.

<a id="op-5cc772da91d6e0fd4292b6bf"></a>
## write

`function` · `deltalake_core::writer::DeltaWriter::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write(&mut self, values: T) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L177).

Source: `crates/core/src/writer/mod.rs:177`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write a chunk of values into the internal write buffers with the default write mode

<a id="op-17caef92f2cc09fcee3850e1"></a>
## write_with_mode

`function` · `deltalake_core::writer::DeltaWriter::write_with_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_with_mode(&mut self, values: T, mode: WriteMode) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L180).

Source: `crates/core/src/writer/mod.rs:180`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wreite a chunk of values into the internal write buffers with the specified [WriteMode](../operations/deltalake_core.writer.WriteMode.md#op-1f477263f864cc45bd49fd0e)
