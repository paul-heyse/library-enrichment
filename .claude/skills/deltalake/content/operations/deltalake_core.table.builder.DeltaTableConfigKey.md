# `deltalake_core::table::builder::DeltaTableConfigKey`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.builder.DeltaTableConfigKey.json).

<a id="op-693665f1b8b020282a295264"></a>
## DeltaTableConfigKey

`enum` · `deltalake_core::table::builder::DeltaTableConfigKey` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DeltaTableConfigKey
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Enumeration of recognized configuration keys, generated from the struct fields.

<a id="op-299bbe99f8677b368fad555b"></a>
## IoRuntime

`variant` · `deltalake_core::table::builder::DeltaTableConfigKey::IoRuntime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
IoRuntime
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When a runtime handler is provided, all IO tasks are spawn in that handle

<a id="op-12a6a91b609fda504e7c3422"></a>
## LogBatchSize

`variant` · `deltalake_core::table::builder::DeltaTableConfigKey::LogBatchSize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
LogBatchSize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Control the number of records to read / process from the commit / checkpoint files
when processing record batches.

<a id="op-16bacd05a8ccb1779e9560ef"></a>
## LogBufferSize

`variant` · `deltalake_core::table::builder::DeltaTableConfigKey::LogBufferSize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
LogBufferSize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Controls how many files to buffer from the commit log when updating the table.
This defaults to 4 * number of cpus

Setting a value greater than 1 results in concurrent calls to the storage api.
This can decrease latency if there are many files in the log since the
last checkpoint, but will also increase memory usage. Possible rate limits of the storage backend should
also be considered for optimal performance.

<a id="op-6c106b14d3965d853d25bbe9"></a>
## RequireFiles

`variant` · `deltalake_core::table::builder::DeltaTableConfigKey::RequireFiles` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RequireFiles
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Indicates whether DeltaTable should track files.
This defaults to `true`

Some append-only applications might have no need of tracking any files.
Hence, DeltaTable will be loaded with significant memory reduction.

<a id="op-0446960c69f274da24b01285"></a>
## SkipStats

`variant` · `deltalake_core::table::builder::DeltaTableConfigKey::SkipStats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SkipStats
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L33).

Source: `crates/core/src/table/builder.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Skip parsing file statistics while opening the table.
This defaults to `false`.

Use this option for maintenance and append workflows that do not need file pruning.
Queries with predicates scan each file because the kernel disables statistics and
partition pruning.
