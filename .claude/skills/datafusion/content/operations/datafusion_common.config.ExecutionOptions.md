# `datafusion_common::config::ExecutionOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ExecutionOptions.json).

<a id="op-15fab70a83be6ab4e79c51a4"></a>
## ExecutionOptions

`struct` · `datafusion_common::config::ExecutionOptions` · datafusion-common 55.1.0

```rust
struct ExecutionOptions
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options related to query execution

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

<a id="op-c25412ecbe58e58a4d83f3dd"></a>
## batch_size

`struct_field` · `datafusion_common::config::ExecutionOptions::batch_size` · datafusion-common 55.1.0

```rust
batch_size: ConfigNonZeroUsize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Default batch size while creating new batches, it's especially useful for
buffer-in-memory batches since creating tiny batches would result in too much
metadata memory consumption

<a id="op-248d8f9f58cd7ae3481b8d93"></a>
## clone

`function` · `datafusion_common::config::ExecutionOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ExecutionOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExecutionOptions", "path": "ExecutionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [1080, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-669b16d00e5a6ecb17a46b75"></a>
## coalesce_batches

`struct_field` · `datafusion_common::config::ExecutionOptions::coalesce_batches` · datafusion-common 55.1.0

```rust
coalesce_batches: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, record batches will be examined between each operator and
small batches will be coalesced into larger batches. This is helpful when there
are highly selective filters or joins that could produce tiny output batches. The
target batch size is determined by the configuration setting

<a id="op-62700c953ca6aa800a6a6f72"></a>
## collect_statistics

`struct_field` · `datafusion_common::config::ExecutionOptions::collect_statistics` · datafusion-common 55.1.0

```rust
collect_statistics: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion collect statistics when first creating a table.
Has no effect after the table is created. Defaults to true.

<a id="op-fdc68302b59692b60fe040d1"></a>
## default

`function` · `datafusion_common::config::ExecutionOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExecutionOptions", "path": "ExecutionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [1080, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7809cb00fb30bbf6fb8a722c"></a>
## enable_ansi_mode

`struct_field` · `datafusion_common::config::ExecutionOptions::enable_ansi_mode` · datafusion-common 55.1.0

```rust
enable_ansi_mode: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether to enable ANSI SQL mode.

The flag is experimental and relevant only for DataFusion Spark built-in functions

When `enable_ansi_mode` is set to `true`, the query engine follows ANSI SQL
semantics for expressions, casting, and error handling. This means:
- **Strict type coercion rules:** implicit casts between incompatible types are disallowed.
- **Standard SQL arithmetic behavior:** operations such as division by zero,
  numeric overflow, or invalid casts raise runtime errors rather than returning
  `NULL` or adjusted values.
- **Consistent ANSI behavior** for string concatenation, comparisons, and `NULL` handling.

When `enable_ansi_mode` is `false` (the default), the engine uses a more permissive,
non-ANSI mode designed for user convenience and backward compatibility. In this mode:
- Implicit casts between types are allowed (e.g., string to integer when possible).
- Arithmetic operations are more lenient — for example, `abs()` on the minimum
  representable integer value returns the input value instead of raising overflow.
- Division by zero or invalid casts may return `NULL` instead of failing.

# Default
`false` — ANSI SQL mode is disabled by default.

<a id="op-2cc6d80933b99fe65fb25598"></a>
## enable_file_stream_work_stealing

`struct_field` · `datafusion_common::config::ExecutionOptions::enable_file_stream_work_stealing` · datafusion-common 55.1.0

```rust
enable_file_stream_work_stealing: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When `true` (the default), DataFusion's built-in file scans
dynamically rebalance files across partitions at query execution
time: a partition that goes idle reads files (or byte-range morsels)
originally assigned to a sibling partition, which keeps all
partitions busy in a single process.

Executors that depend on the plan-time partition assignment — such as
Ballista and datafusion-distributed, which run each partition as an
isolated task and never poll the siblings — should set this to
`false` so each partition reads only its own file group and no
runtime reassignment occurs.

<a id="op-b9aa195ac6b94e57438d4fb3"></a>
## enable_migration_aggregate

`struct_field` · `datafusion_common::config::ExecutionOptions::enable_migration_aggregate` · datafusion-common 55.1.0

```rust
enable_migration_aggregate: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Temporary switch for aggregate stream implementations that are being
migrated from `GroupedHashAggregateStream`.

When set to true, DataFusion tries the migrated implementations when
their preconditions are satisfied. When set to false, grouped
aggregation falls back to `GroupedHashAggregateStream`. This option
will be removed after the migration is finished.

See <https://github.com/apache/datafusion/issues/22710> for details.

<a id="op-3ce144659c948fd2daa4caae"></a>
## enable_recursive_ctes

`struct_field` · `datafusion_common::config::ExecutionOptions::enable_recursive_ctes` · datafusion-common 55.1.0

```rust
enable_recursive_ctes: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion support recursive CTEs

<a id="op-24e5f9f249e4949cf6553b87"></a>
## enforce_batch_size_in_joins

`struct_field` · `datafusion_common::config::ExecutionOptions::enforce_batch_size_in_joins` · datafusion-common 55.1.0

```rust
enforce_batch_size_in_joins: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion enforce batch size in joins or not. By default,
DataFusion will not enforce batch size in joins. Enforcing batch size
in joins can reduce memory usage when joining large
tables with a highly-selective join filter, but is also slightly slower.

<a id="op-1d29f2ac71e790bbc80ac064"></a>
## eq

`function` · `datafusion_common::config::ExecutionOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ExecutionOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExecutionOptions", "path": "ExecutionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [1080, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3420e27fbef059e53173d2a8"></a>
## fmt

`function` · `datafusion_common::config::ExecutionOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExecutionOptions", "path": "ExecutionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [1080, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16db407aa7a97012b69dd0e7"></a>
## hash_join_buffering_capacity

`struct_field` · `datafusion_common::config::ExecutionOptions::hash_join_buffering_capacity` · datafusion-common 55.1.0

```rust
hash_join_buffering_capacity: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

How many bytes to buffer in the probe side of hash joins while the build side is
concurrently being built.

Without this, hash joins will wait until the full materialization of the build side
before polling the probe side. This is useful in scenarios where the query is not
completely CPU bounded, allowing to do some early work concurrently and reducing the
latency of the query.

Note that when hash join buffering is enabled, the probe side will start eagerly
polling data, not giving time for the producer side of dynamic filters to produce any
meaningful predicate. Queries with dynamic filters might see performance degradation.

Disabled by default, set to a number greater than 0 for enabling it.

<a id="op-255e3b9d2b0e32e69b561d1f"></a>
## keep_partition_by_columns

`struct_field` · `datafusion_common::config::ExecutionOptions::keep_partition_by_columns` · datafusion-common 55.1.0

```rust
keep_partition_by_columns: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion keep the columns used for partition_by in the output RecordBatches

<a id="op-8f8e761307dd550da1a05ce5"></a>
## listing_table_factory_infer_partitions

`struct_field` · `datafusion_common::config::ExecutionOptions::listing_table_factory_infer_partitions` · datafusion-common 55.1.0

```rust
listing_table_factory_infer_partitions: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should a `ListingTable` created through the `ListingTableFactory` infer table
partitions from Hive compliant directories. Defaults to true (partition columns are
inferred and will be represented in the table schema).

<a id="op-f5992eeaa3b2c7baccc873b1"></a>
## listing_table_ignore_subdirectory

`struct_field` · `datafusion_common::config::ExecutionOptions::listing_table_ignore_subdirectory` · datafusion-common 55.1.0

```rust
listing_table_ignore_subdirectory: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should sub directories be ignored when scanning directories for data
files. Defaults to true (ignores subdirectories), consistent with
Hive. Note that this setting does not affect reading partitioned
tables (e.g. `/table/year=2021/month=01/data.parquet`).

<a id="op-c4fba470894561e1ef2a1a30"></a>
## max_buffered_batches_per_output_file

`struct_field` · `datafusion_common::config::ExecutionOptions::max_buffered_batches_per_output_file` · datafusion-common 55.1.0

```rust
max_buffered_batches_per_output_file: ConfigMinTwoUsize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This is the maximum number of RecordBatches buffered
for each output file being worked. Higher values can potentially
give faster write performance at the cost of higher peak
memory consumption.

This budget is split evenly between two independent points in the
write pipeline (see the demuxer diagram in #7791): how many files
can be in flight from the demuxer to a writer task, and how many
RecordBatches are buffered for a single file's writer. Must be at
least 2 so each half gets at least 1 unit of buffering - 0 or 1
would leave one side with a zero-capacity channel and panic at
write time.

<a id="op-6461cbc18ff8bd9037530273"></a>
## max_spill_file_size_bytes

`struct_field` · `datafusion_common::config::ExecutionOptions::max_spill_file_size_bytes` · datafusion-common 55.1.0

```rust
max_spill_file_size_bytes: ConfigNonZeroUsize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maximum size in bytes for individual spill files before rotating to a new file.

When operators spill data to disk (e.g., RepartitionExec), they write
multiple batches to the same file until this size limit is reached, then rotate
to a new file. This reduces syscall overhead compared to one-file-per-batch
while preventing files from growing too large.

A larger value reduces file creation overhead but may hold more disk space.
A smaller value creates more files but allows finer-grained space reclamation
as files can be deleted once fully consumed.

Now only `RepartitionExec` supports this spill file rotation feature, other spilling operators
may create spill files larger than the limit.

Default: 128 MB

<a id="op-7d3affa55ae71f4332c163b6"></a>
## meta_fetch_concurrency

`struct_field` · `datafusion_common::config::ExecutionOptions::meta_fetch_concurrency` · datafusion-common 55.1.0

```rust
meta_fetch_concurrency: ConfigNonZeroUsize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of files to read in parallel when inferring schema and statistics

<a id="op-e383a2623d14e23e56423729"></a>
## minimum_parallel_output_files

`struct_field` · `datafusion_common::config::ExecutionOptions::minimum_parallel_output_files` · datafusion-common 55.1.0

```rust
minimum_parallel_output_files: ConfigNonZeroUsize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Guarantees a minimum level of output files running in parallel.
RecordBatches will be distributed in round robin fashion to each
parallel writer. Each writer is closed and a new file opened once
soft_max_rows_per_output_file is reached.

<a id="op-243e4ee135db3c36708494d0"></a>
## objectstore_writer_buffer_size

`struct_field` · `datafusion_common::config::ExecutionOptions::objectstore_writer_buffer_size` · datafusion-common 55.1.0

```rust
objectstore_writer_buffer_size: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Size (bytes) of data buffer DataFusion uses when writing output files.
This affects the size of the data chunks that are uploaded to remote
object stores (e.g. AWS S3). If very large (>= 100 GiB) output files are being
written, it may be necessary to increase this size to avoid errors from
the remote end point.

<a id="op-6a4fde5c8353b19e8acd5f0c"></a>
## parquet

`struct_field` · `datafusion_common::config::ExecutionOptions::parquet` · datafusion-common 55.1.0

```rust
parquet: ParquetOptions
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Parquet options

<a id="op-392056702421e61cf8b32c30"></a>
## perfect_hash_join_min_key_density

`struct_field` · `datafusion_common::config::ExecutionOptions::perfect_hash_join_min_key_density` · datafusion-common 55.1.0

```rust
perfect_hash_join_min_key_density: f64
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The minimum required density of join keys on the build side to consider a
perfect hash join (see `HashJoinExec` for more details). Density is calculated as:
`(number of rows) / (max_key - min_key + 1)`.
A perfect hash join may be used if the actual key density > this
value.

Currently only supports cases where build_side.num_rows() < u32::MAX.
Support for build_side.num_rows() >= u32::MAX will be added in the future.

<a id="op-143df30d7cfb79300c20c19f"></a>
## perfect_hash_join_small_build_threshold

`struct_field` · `datafusion_common::config::ExecutionOptions::perfect_hash_join_small_build_threshold` · datafusion-common 55.1.0

```rust
perfect_hash_join_small_build_threshold: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A perfect hash join (see `HashJoinExec` for more details) will be considered
if the range of keys (max - min) on the build side is < this threshold.
This provides a fast path for joins with very small key ranges,
bypassing the density check.

Currently only supports cases where build_side.num_rows() < u32::MAX.
Support for build_side.num_rows() >= u32::MAX will be added in the future.

<a id="op-6966dc9e1aca0e65e0659d5e"></a>
## planning_concurrency

`struct_field` · `datafusion_common::config::ExecutionOptions::planning_concurrency` · datafusion-common 55.1.0

```rust
planning_concurrency: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Fan-out during initial physical planning.

This is mostly use to plan `UNION` children in parallel.

Defaults to the number of CPU cores on the system

<a id="op-8d7cbf52b7fe084b57880f6e"></a>
## reset

`function` · `datafusion_common::config::ExecutionOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExecutionOptions", "path": "ExecutionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [1080, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-876e3826b42832ccca0c0604"></a>
## set

`function` · `datafusion_common::config::ExecutionOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExecutionOptions", "path": "ExecutionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [1080, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c843fdd694bf93d1c561c8b"></a>
## skip_partial_aggregation_probe_ratio_threshold

`struct_field` · `datafusion_common::config::ExecutionOptions::skip_partial_aggregation_probe_ratio_threshold` · datafusion-common 55.1.0

```rust
skip_partial_aggregation_probe_ratio_threshold: f64
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Aggregation ratio (number of distinct groups / number of input rows)
threshold for skipping partial aggregation. If the value is greater
then partial aggregation will skip aggregation for further input

<a id="op-4c496c735d0a96c0452c5656"></a>
## skip_partial_aggregation_probe_rows_threshold

`struct_field` · `datafusion_common::config::ExecutionOptions::skip_partial_aggregation_probe_rows_threshold` · datafusion-common 55.1.0

```rust
skip_partial_aggregation_probe_rows_threshold: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of input rows partial aggregation partition should process, before
aggregation ratio check and trying to switch to skipping aggregation mode

<a id="op-892258aeecbac7ac00a35064"></a>
## skip_physical_aggregate_schema_check

`struct_field` · `datafusion_common::config::ExecutionOptions::skip_physical_aggregate_schema_check` · datafusion-common 55.1.0

```rust
skip_physical_aggregate_schema_check: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, skips verifying that the schema produced by
planning the input of `LogicalPlan::Aggregate` exactly matches the
schema of the input plan.

When set to false, if the schema does not match exactly
(including nullability and metadata), a planning error will be raised.

This is used to workaround bugs in the planner that are now caught by
the new schema verification step.

<a id="op-4049c286fa8b62c889effef4"></a>
## soft_max_rows_per_output_file

`struct_field` · `datafusion_common::config::ExecutionOptions::soft_max_rows_per_output_file` · datafusion-common 55.1.0

```rust
soft_max_rows_per_output_file: ConfigNonZeroUsize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Target number of rows in output files when writing multiple.
This is a soft max, so it can be exceeded slightly. There also
will be one file smaller than the limit if the total
number of rows written is not roughly divisible by the soft max

<a id="op-cdfa1ae74193746e6a8b2634"></a>
## sort_in_place_threshold_bytes

`struct_field` · `datafusion_common::config::ExecutionOptions::sort_in_place_threshold_bytes` · datafusion-common 55.1.0

```rust
sort_in_place_threshold_bytes: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When sorting, below what size should data be concatenated
and sorted in a single RecordBatch rather than sorted in
batches and merged.

<a id="op-8f8c48562e8b4d6b038e2a2f"></a>
## sort_pushdown_buffer_capacity

`struct_field` · `datafusion_common::config::ExecutionOptions::sort_pushdown_buffer_capacity` · datafusion-common 55.1.0

```rust
sort_pushdown_buffer_capacity: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maximum buffer capacity (in bytes) per partition for BufferExec
inserted during sort pushdown optimization.

When PushdownSort eliminates a SortExec under SortPreservingMergeExec,
a BufferExec is inserted to replace SortExec's buffering role. This
prevents I/O stalls by allowing the scan to run ahead of the merge.

This uses strictly less memory than the SortExec it replaces (which
buffers the entire partition). The buffer respects the global memory
pool limit. Setting this to a large value is safe — actual memory
usage is bounded by partition size and global memory limits.

<a id="op-c7679cf68a04e4db5d429b13"></a>
## sort_spill_reservation_bytes

`struct_field` · `datafusion_common::config::ExecutionOptions::sort_spill_reservation_bytes` · datafusion-common 55.1.0

```rust
sort_spill_reservation_bytes: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specifies the reserved memory for each spillable sort operation to
facilitate an in-memory merge.

When a sort operation spills to disk, the in-memory data must be
sorted and merged before being written to a file. This setting reserves
a specific amount of memory for that in-memory sort/merge process.

Note: This setting is irrelevant if the sort operation cannot spill
(i.e., if there's no `DiskManager` configured).

<a id="op-4f4e9401f65dec7baefdbe31"></a>
## spill_compression

`struct_field` · `datafusion_common::config::ExecutionOptions::spill_compression` · datafusion-common 55.1.0

```rust
spill_compression: SpillCompression
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets the compression codec used when spilling data to disk.

Since datafusion writes spill files using the Arrow IPC Stream format,
only codecs supported by the Arrow IPC Stream Writer are allowed.
Valid values are: uncompressed, lz4_frame, zstd.
Note: lz4_frame offers faster (de)compression, but typically results in
larger spill files. In contrast, zstd achieves
higher compression ratios at the cost of slower (de)compression speed.

<a id="op-1ab1ec51f3e47f0f999ac1d1"></a>
## split_file_groups_by_statistics

`struct_field` · `datafusion_common::config::ExecutionOptions::split_file_groups_by_statistics` · datafusion-common 55.1.0

```rust
split_file_groups_by_statistics: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Attempt to eliminate sorts by packing & sorting files with non-overlapping
statistics into the same file groups.
Currently experimental

<a id="op-46ada57ee69e97ba4665453c"></a>
## target_partitions

`struct_field` · `datafusion_common::config::ExecutionOptions::target_partitions` · datafusion-common 55.1.0

```rust
target_partitions: usize
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of partitions for query execution. Increasing partitions can increase
concurrency.

Defaults to the number of CPU cores on the system

<a id="op-1f9b347df14db18076a554f1"></a>
## time_zone

`struct_field` · `datafusion_common::config::ExecutionOptions::time_zone` · datafusion-common 55.1.0

```rust
time_zone: Option<String>
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The default time zone

Some functions, e.g. `now` return timestamps in this time zone

<a id="op-c229aa0a54a325634f2cfe28"></a>
## use_row_number_estimates_to_optimize_partitioning

`struct_field` · `datafusion_common::config::ExecutionOptions::use_row_number_estimates_to_optimize_partitioning` · datafusion-common 55.1.0

```rust
use_row_number_estimates_to_optimize_partitioning: bool
```

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion use row number estimates at the input to decide
whether increasing parallelism is beneficial or not. By default,
only exact row numbers (not estimates) are used for this decision.
Setting this flag to `true` will likely produce better plans.
if the source of statistics is accurate.
We plan to make this the default in the future.

<a id="op-725ee1268650ce8b41793f04"></a>
## visit

`function` · `datafusion_common::config::ExecutionOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExecutionOptions", "path": "ExecutionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [1080, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
