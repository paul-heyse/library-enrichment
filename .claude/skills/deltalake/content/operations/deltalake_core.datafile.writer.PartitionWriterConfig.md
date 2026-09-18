# `deltalake_core::datafile::writer::PartitionWriterConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.writer.PartitionWriterConfig.json).

<a id="op-a2c0d2ea1c4a230409d99d11"></a>
## PartitionWriterConfig

`struct` · `deltalake_core::datafile::writer::PartitionWriterConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PartitionWriterConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L544).

Source: `crates/core/src/datafile/writer.rs:544`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write configuration for partition writers

<a id="op-8c64224eb831367a4b7bfe14"></a>
## clone

`function` · `deltalake_core::datafile::writer::PartitionWriterConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> PartitionWriterConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L543).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriterConfig", "path": "PartitionWriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 17], "end": [543, 22], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/writer.rs:543`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e0e5f589b2d5b3bac64f336"></a>
## fmt

`function` · `deltalake_core::datafile::writer::PartitionWriterConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L543).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriterConfig", "path": "PartitionWriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 10], "end": [543, 15], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/writer.rs:543`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfb3922f85659c23e718e187"></a>
## try_new

`function` · `deltalake_core::datafile::writer::PartitionWriterConfig::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(file_schema: ArrowSchemaRef, partition_values: IndexMap<String, Scalar>, writer_properties: Option<WriterProperties>, target_file_size: Option<NonZeroU64>, write_batch_size: Option<usize>, max_concurrency_tasks: Option<usize>, prefix_override: Option<Path>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L569).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriterConfig", "path": "PartitionWriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [621, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:569`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [PartitionWriterConfig](../operations/deltalake_core.datafile.writer.PartitionWriterConfig.md#op-a2c0d2ea1c4a230409d99d11)

<a id="op-aa5b4e83637865c9fda3476a"></a>
## with_roll_on_row_group_boundary

`function` · `deltalake_core::datafile::writer::PartitionWriterConfig::with_roll_on_row_group_boundary` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_roll_on_row_group_boundary(self, roll_on_row_group_boundary: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L617).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::PartitionWriterConfig", "path": "PartitionWriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [621, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:617`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Defer the `target_file_size` file roll until the parquet writer's current row group is
complete, so no file ends in a truncated row group.

A row group cannot span files, so the plain byte roll cuts each file's last row group
wherever the target happens to land. Columnar readers that map one row group to one
in-memory segment degrade on those runt tail groups. With this enabled, every row group
is exactly the configured `max_row_group_row_count` — the single end-of-data remainder
written at close is the one exception — and a file may overshoot `target_file_size` by
up to one row group.

Only effective when the writer properties bound row groups by row count alone
(`max_row_group_row_count` set, `max_row_group_bytes` unset); byte-bounded row groups
keep the legacy roll behavior. Defaults to the `DELTARS_ROLL_ON_ROW_GROUP_BOUNDARY`
env var ("1"/"true"/"yes"), else `false`.

<a id="op-9d5897066014823e2b9d1207"></a>
## file_schema

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::file_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_schema: arrow_schema::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L546).

Source: `crates/core/src/datafile/writer.rs:546`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Schema of the data written to disk

<a id="op-c11adb6964565bdb4af65f2f"></a>
## max_concurrency_tasks

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::max_concurrency_tasks` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_concurrency_tasks: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L560).

Source: `crates/core/src/datafile/writer.rs:560`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Concurrency level for writing to object store

<a id="op-6881e5b8a1ab55577743d218"></a>
## partition_values

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_values: indexmap::IndexMap<String, delta_kernel::expressions::Scalar>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L550).

Source: `crates/core/src/datafile/writer.rs:550`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Values for all partition columns

<a id="op-9d31b44475e11dc70f039e21"></a>
## prefix

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::prefix` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
prefix: object_store::path::Path
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L548).

Source: `crates/core/src/datafile/writer.rs:548`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Prefix applied to all paths

<a id="op-b7f51660b39bf1c8c00f1450"></a>
## roll_on_row_group_boundary

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::roll_on_row_group_boundary` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
roll_on_row_group_boundary: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L564).

Source: `crates/core/src/datafile/writer.rs:564`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Defer the `target_file_size` roll until the current row group is complete, so no
file ends in a truncated row group. See
[`PartitionWriterConfig::with_roll_on_row_group_boundary`].

Unresolved upstream links (retained, not inferred): ``PartitionWriterConfig::with_roll_on_row_group_boundary``.

<a id="op-bd0a76f6ef03f4b0bc5ef086"></a>
## target_file_size

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::target_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_file_size: Option<std::num::NonZeroU64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L555).

Source: `crates/core/src/datafile/writer.rs:555`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Size above which we will write a buffered parquet file to disk.
If None, the writer will not create a new file until the writer is closed.

<a id="op-a89190ca1b79e793e48a4d09"></a>
## write_batch_size

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::write_batch_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
write_batch_size: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L558).

Source: `crates/core/src/datafile/writer.rs:558`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Row chunks passed to parquet writer. This and the internal parquet writer settings
determine how fine granular we can track / control the size of resulting files.

<a id="op-047c94a15755b88f3ae875b9"></a>
## writer_properties

`struct_field` · `deltalake_core::datafile::writer::PartitionWriterConfig::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: parquet::file::properties::WriterProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L552).

Source: `crates/core/src/datafile/writer.rs:552`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties passed to underlying parquet writer
