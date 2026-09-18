# `deltalake_core::datafile::writer::WriterConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.writer.WriterConfig.json).

<a id="op-8e3e579398563326f436ff9e"></a>
## WriterConfig

`struct` · `deltalake_core::datafile::writer::WriterConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct WriterConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L201).

Source: `crates/core/src/datafile/writer.rs:201`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration to write data into Delta tables

<a id="op-781def2ead336efdf56c2f04"></a>
## clone

`function` · `deltalake_core::datafile::writer::WriterConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> WriterConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L200).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::WriterConfig", "path": "WriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 17], "end": [200, 22], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/writer.rs:200`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2320cce6e372ad120217cd35"></a>
## file_schema

`function` · `deltalake_core::datafile::writer::WriterConfig::file_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::WriterConfig", "path": "WriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [261, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:258`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Schema of files written to disk

<a id="op-caceeab6dd3686226a730de5"></a>
## fmt

`function` · `deltalake_core::datafile::writer::WriterConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L200).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::WriterConfig", "path": "WriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 10], "end": [200, 15], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/writer.rs:200`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c94302de48405ce3869f0703"></a>
## new

`function` · `deltalake_core::datafile::writer::WriterConfig::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(table_schema: ArrowSchemaRef, partition_columns: Vec<String>, writer_properties: Option<WriterProperties>, target_file_size: Option<NonZeroU64>, write_batch_size: Option<usize>, num_indexed_cols: DataSkippingNumIndexedCols, stats_columns: Option<Vec<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L225).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::WriterConfig", "path": "WriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [261, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:225`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new instance of [WriterConfig](../operations/deltalake_core.datafile.writer.WriterConfig.md#op-8e3e579398563326f436ff9e).

<a id="op-64bda8170a62ea049e49cd0a"></a>
## with_random_prefix_length

`function` · `deltalake_core::datafile::writer::WriterConfig::with_random_prefix_length` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_random_prefix_length(self, length: Option<usize>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L252).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::WriterConfig", "path": "WriterConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [261, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/writer.rs:252`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write data files under a random prefix of `length` chars instead of Hive-style dirs
(column-mapped tables); `None` keeps the Hive layout.

<a id="op-fe4b07f5fe2b235b623b7913"></a>
## num_indexed_cols

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::num_indexed_cols` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_indexed_cols: delta_kernel::table_properties::DataSkippingNumIndexedCols
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L215).

Source: `crates/core/src/datafile/writer.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Num index cols to collect stats for

<a id="op-b6017bd088b62d5e1b571ee6"></a>
## partition_columns

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::partition_columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_columns: Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L205).

Source: `crates/core/src/datafile/writer.rs:205`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Column names for columns the table is partitioned by

<a id="op-1968a8eeeca15798c5d599fe"></a>
## random_prefix_length

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::random_prefix_length` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
random_prefix_length: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L220).

Source: `crates/core/src/datafile/writer.rs:220`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When set, write data files under a random prefix directory of this length instead of
Hive-style partition dirs — keeps physical (UUID) column names out of paths under CM.

<a id="op-d5f8aabfdee81bb5188a9aa1"></a>
## stats_columns

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::stats_columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats_columns: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L217).

Source: `crates/core/src/datafile/writer.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stats columns, specific columns to collect stats from, takes precedence over num_indexed_cols

<a id="op-0edc927b3fe2355a90bf2f21"></a>
## table_schema

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::table_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_schema: arrow_schema::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L203).

Source: `crates/core/src/datafile/writer.rs:203`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Schema of the delta table

<a id="op-118927424df1e23634e7a71a"></a>
## target_file_size

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::target_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_file_size: Option<std::num::NonZeroU64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L210).

Source: `crates/core/src/datafile/writer.rs:210`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Size above which we will write a buffered parquet file to disk.
If None, the writer will not create a new file until the writer is closed.

<a id="op-46a3ebfb3f932a1c70c6ddc2"></a>
## write_batch_size

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::write_batch_size` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
write_batch_size: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L213).

Source: `crates/core/src/datafile/writer.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Row chunks passed to parquet writer. This and the internal parquet writer settings
determine how fine granular we can track / control the size of resulting files.

<a id="op-05e64350f51983604c99b9ba"></a>
## writer_properties

`struct_field` · `deltalake_core::datafile::writer::WriterConfig::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: parquet::file::properties::WriterProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L207).

Source: `crates/core/src/datafile/writer.rs:207`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties passed to underlying parquet writer
