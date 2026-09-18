# `buoyant_kernel::transaction::write_context::WriteContext`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.write_context.WriteContext.json).

<a id="op-dcfa01f4a2e15eb61c320d03"></a>
## WriteContext

`struct` · `buoyant_kernel::transaction::write_context::WriteContext` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct WriteContext
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L62).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A write context for a specific partition or an unpartitioned table. Created by
[`Transaction::partitioned_write_context`] or [`Transaction::unpartitioned_write_context`].

Note: clustered tables are unpartitioned and use `unpartitioned_write_context`.

Contains both table-wide state (shared cheaply via `Arc`) and per-partition state
(serialized partition values with physical column names as keys). How you use a
`WriteContext` depends on your engine:

- **`DefaultEngine` consumers**: pass this to `DefaultEngine::write_parquet`, which handles
  everything (transform, write, partition metadata).
- **Arrow-based custom engines**: write parquet yourself, then call `build_add_file_metadata`
  with the resulting `DataFileMetadata` and this `WriteContext` to produce the Add action
  `EngineData` for [`Transaction::add_files`].
- **Fully custom (non-Arrow) engines**: use [`physical_partition_values`] to build the
  `partitionValues` map in Add actions directly.

[`Transaction::partitioned_write_context`]: super::Transaction::partitioned_write_context
[`Transaction::unpartitioned_write_context`]: super::Transaction::unpartitioned_write_context
[`Transaction::add_files`]: super::Transaction::add_files
[`physical_partition_values`]: WriteContext::physical_partition_values

<a id="op-82eef8b4632355d0eb0fa2b8"></a>
## column_mapping_mode

`function` · `buoyant_kernel::transaction::write_context::WriteContext::column_mapping_mode` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn column_mapping_mode(&self) -> ColumnMappingMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L180).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:180`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`ColumnMappingMode`](../operations/buoyant_kernel.table_features.column_mapping.ColumnMappingMode.md#op-757f64913cb951ae565079da) for this table.

<a id="op-b2c124c28d06afdbb6609de8"></a>
## fmt

`function` · `buoyant_kernel::transaction::write_context::WriteContext::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L61).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 10], "end": [61, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:61`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e3ff4a075ea10fdf69ade5b"></a>
## logical_schema

`function` · `buoyant_kernel::transaction::write_context::WriteContext::logical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L164).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:164`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the schema which connectors' logical data should conform to.

<a id="op-126925e0949fbc3bffe089a4"></a>
## logical_to_physical

`function` · `buoyant_kernel::transaction::write_context::WriteContext::logical_to_physical` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_to_physical(&self) -> ExpressionRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L175).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:175`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the expression that transforms logical data to physical data for writing.

<a id="op-8b97aa9f4275c701116bdae8"></a>
## new_deletion_vector_path

`function` · `buoyant_kernel::transaction::write_context::WriteContext::new_deletion_vector_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_deletion_vector_path(&self, random_prefix: String) -> DeletionVectorPath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L277).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:277`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Generate a new unique absolute URL for a deletion vector file.

This method generates a unique file name in the table directory.
Each call to this method returns a new unique path.

# Arguments

* `random_prefix` - A random prefix to use for the deletion vector file name. Making this
  non-empty can help distributed load on object storage when writing/reading to avoid
  throttling.  Typically a random string of 2-4 characters is sufficient for this purpose.

# Examples

```rust,ignore
let write_context = transaction.unpartitioned_write_context()?;
let dv_path = write_context.new_deletion_vector_path(String::from(rand_string()));
```

<a id="op-d838d20b711636a518f89a13"></a>
## physical_partition_values

`function` · `buoyant_kernel::transaction::write_context::WriteContext::physical_partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_partition_values(&self) -> &HashMap<String, Option<String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L195).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:195`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the serialized partition values for this write context. Keys are physical
column names; values are protocol-serialized strings (`None` = null).

For unpartitioned tables, this is empty.

<a id="op-f6561c96d3c714a259379867"></a>
## physical_schema

`function` · `buoyant_kernel::transaction::write_context::WriteContext::physical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L170).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:170`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the physical schema (partition columns removed if applicable, column mapping
applied). Partition columns are kept when `materializePartitionColumns` is enabled.

<a id="op-6e2b2c9c7363f519bcff4c29"></a>
## resolve_file_path

`function` · `buoyant_kernel::transaction::write_context::WriteContext::resolve_file_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn resolve_file_path(&self, file_location: &Url) -> DeltaResult<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L237).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:237`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Computes the relative `add.path` value for the Delta log from a file's absolute URL.

Custom engines that write parquet files themselves (bypassing
`DefaultEngine::write_parquet`) should call this after writing each file to produce
the path for their Add action metadata.

# Examples

Given a table root of `s3://bucket/table/`:
- `s3://bucket/table/abc.parquet` -> `"abc.parquet"`
- `s3://bucket/table/year=2024/abc.parquet` -> `"year=2024/abc.parquet"`

Returns an error if the file is not under the table root.

<a id="op-834ea75fb06c394dd73f34f0"></a>
## stats_columns

`function` · `buoyant_kernel::transaction::write_context::WriteContext::stats_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn stats_columns(&self) -> &[ColumnName]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L187).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:187`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the column names that should have statistics collected during writes.

Based on table configuration (dataSkippingNumIndexedCols, dataSkippingStatsColumns).

<a id="op-843efd2c864ba8ee8bbc6b82"></a>
## table_root_dir

`function` · `buoyant_kernel::transaction::write_context::WriteContext::table_root_dir` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root_dir(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L75).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the table root URL.

<a id="op-03ea5e02b5d8871c30a2fcf9"></a>
## write_dir

`function` · `buoyant_kernel::transaction::write_context::WriteContext::write_dir` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_dir(&self) -> Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L137).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::write_context::WriteContext", "path": "WriteContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:137`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the recommended directory URL for writing Parquet data files. Connectors
should write files as `<write_dir>/<uuid>.parquet`. Not strictly required (data files
can live anywhere under the table root), but produces the conventional layout.

# The returned URL is URI-encoded

For CM=none partitioned tables, the Hive-escaped partition prefix is double-encoded
in the URL (e.g. `%3A` appears as `%253A`). Concrete examples for a single STRING
partition column `p`:

```text
partition value  |  encoded path prefix       |  URI-decoded (filesystem path)
-----------------+----------------------------+--------------------------------
"abc"            |  p=abc/                    |  p=abc/
"a%c"            |  p=a%2525c/                |  p=a%25c/
"a "             |  p=a%20/                   |  p=a /
```

On Windows, the Hive layer additionally escapes space, so `"a "` produces encoded
path prefix `p=a%2520/` with filesystem directory `p=a%20/`.

The same URL drives two outputs and custom engines must handle each correctly:

1. **Filesystem write path** — URI-decode once to get the on-disk directory name. Custom
   engines MUST decode before feeding `url.path()` to an OS-filesystem API. Use
   `object_store::path::Path::from_url_path` or an equivalent decoder. Feeding `url.path()`
   directly to the filesystem produces directories literally named `p=a%253Ab/` and breaks
   interop with every other Delta writer.

2. **`add.path` in the Delta log** — keep the URL URI-encoded. After writing the parquet
   file, pass the full (still-encoded) file URL — this URL plus the generated filename — to
   [`WriteContext::resolve_file_path`](../operations/buoyant_kernel.transaction.write_context.WriteContext.md#op-6e2b2c9c7363f519bcff4c29) to produce `add.path`. `make_relative` preserves the
   URI-encoded form, which is what the Delta protocol requires. Arrow-based engines can use
   `build_add_file_metadata` which handles this step.

`DefaultEngine::write_parquet` handles both steps automatically via `object_store`
and `build_add_file_metadata`.

# Layout

A random alphanumeric prefix is emitted whenever column mapping is on or the
`delta.randomizeFilePrefixes` table property is true. The prefix length is
controlled by `delta.randomPrefixLength`. When a random prefix is used on a
partitioned table, Hive-style path components are suppressed; the partition
values are still recorded in `add.partitionValues`.

```text
                           | randomize=false                     | randomize=true
---------------------------|-------------------------------------|--------------------------------
CM=None,  unpartitioned    | <table_root>/<uuid>.parquet         | <table_root>/<prefix>/<uuid>.parquet
CM=None,  partitioned      | <table_root>/col=val/.../<uuid>.pq  | <table_root>/<prefix>/<uuid>.parquet
CM=Id/Name, any            | <table_root>/<prefix>/<uuid>.parquet| <table_root>/<prefix>/<uuid>.parquet
```

Each call generates a fresh prefix. The alphanumeric charset is RFC 3986
unreserved, so the prefix is URI-safe at any length.

<a id="op-8aafe13b66acdacd7204c606"></a>
## logical_to_physical

`struct_field` · `buoyant_kernel::transaction::write_context::WriteContext::logical_to_physical` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
logical_to_physical: expressions::ExpressionRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L66).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Transforms logical data to physical data for writing. The logical data must not contain
any partition columns. The expression injects the partition columns when needed.

<a id="op-a45d115dd1f81abafc8ce358"></a>
## physical_partition_values

`struct_field` · `buoyant_kernel::transaction::write_context::WriteContext::physical_partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
physical_partition_values: std::collections::HashMap<String, Option<String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Physical column name -> serialized value (`None` = null partition value).
Empty for unpartitioned tables. Ordering for hive-style paths comes from
`shared.logical_partition_columns`, not from this map.

<a id="op-e3663ed83d1448360d4a54ba"></a>
## shared

`struct_field` · `buoyant_kernel::transaction::write_context::WriteContext::shared` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
shared: std::sync::Arc<SharedWriteState>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/write_context.rs#L63).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/write_context.rs:63`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
