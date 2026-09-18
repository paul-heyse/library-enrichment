# `datafusion_common::config::OptimizerOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.OptimizerOptions.json).

<a id="op-c1bca17ddd60fbb0925d4514"></a>
## OptimizerOptions

`struct` · `datafusion_common::config::OptimizerOptions` · datafusion-common 55.1.0

```rust
struct OptimizerOptions
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options related to query optimization

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

<a id="op-a031f17ff2cbce66890541a2"></a>
## allow_symmetric_joins_without_pruning

`struct_field` · `datafusion_common::config::OptimizerOptions::allow_symmetric_joins_without_pruning` · datafusion-common 55.1.0

```rust
allow_symmetric_joins_without_pruning: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion allow symmetric hash joins for unbounded data sources even when
its inputs do not have any ordering or filtering If the flag is not enabled,
the SymmetricHashJoin operator will be unable to prune its internal buffers,
resulting in certain join types - such as Full, Left, LeftAnti, LeftSemi, Right,
RightAnti, and RightSemi - being produced only at the end of the execution.
This is not typical in stream processing. Additionally, without proper design for
long runner execution, all types of joins may encounter out-of-memory errors.

<a id="op-deb6fcae70af23a46ce7fb70"></a>
## clone

`function` · `datafusion_common::config::OptimizerOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> OptimizerOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::OptimizerOptions", "path": "OptimizerOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1769, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a151d784c37b113e03d54fad"></a>
## default

`function` · `datafusion_common::config::OptimizerOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::OptimizerOptions", "path": "OptimizerOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1769, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eecac3c0d8aff1375fba9de"></a>
## default_filter_selectivity

`struct_field` · `datafusion_common::config::OptimizerOptions::default_filter_selectivity` · datafusion-common 55.1.0

```rust
default_filter_selectivity: u8
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The default filter selectivity used by Filter Statistics
when an exact selectivity cannot be determined. Valid values are
between 0 (no selectivity) and 100 (all rows are selected).

<a id="op-558869d8df2b6ade83a21209"></a>
## enable_aggregate_dynamic_filter_pushdown

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_aggregate_dynamic_filter_pushdown` · datafusion-common 55.1.0

```rust
enable_aggregate_dynamic_filter_pushdown: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will attempt to push down Aggregate dynamic filters
into the file scan phase.

<a id="op-5ab67707c8fceb8a72d24fb7"></a>
## enable_distinct_aggregation_soft_limit

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_distinct_aggregation_soft_limit` · datafusion-common 55.1.0

```rust
enable_distinct_aggregation_soft_limit: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will push a limit operation into
grouped aggregations which have no aggregate expressions, as a soft limit,
emitting groups once the limit is reached, before all rows in the group are read.

<a id="op-2c3ff257938446597c80b78a"></a>
## enable_dynamic_filter_pushdown

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_dynamic_filter_pushdown` · datafusion-common 55.1.0

```rust
enable_dynamic_filter_pushdown: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true attempts to push down dynamic filters generated by operators (TopK, Join & Aggregate) into the file scan phase.
For example, for a query such as `SELECT * FROM t ORDER BY timestamp DESC LIMIT 10`, the optimizer
will attempt to push down the current top 10 timestamps that the TopK operator references into the file scans.
This means that if we already have 10 timestamps in the year 2025
any files that only have timestamps in the year 2024 can be skipped / pruned at various stages in the scan.
The config will suppress `enable_join_dynamic_filter_pushdown`, `enable_topk_dynamic_filter_pushdown` & `enable_aggregate_dynamic_filter_pushdown`
So if you disable `enable_topk_dynamic_filter_pushdown`, then enable `enable_dynamic_filter_pushdown`, the `enable_topk_dynamic_filter_pushdown` will be overridden.

<a id="op-eb3b2e927d94b0b0a52a057d"></a>
## enable_join_dynamic_filter_pushdown

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_join_dynamic_filter_pushdown` · datafusion-common 55.1.0

```rust
enable_join_dynamic_filter_pushdown: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will attempt to push down Join dynamic filters
into the file scan phase.

<a id="op-30813873fb857578df9d7ba1"></a>
## enable_leaf_expression_pushdown

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_leaf_expression_pushdown` · datafusion-common 55.1.0

```rust
enable_leaf_expression_pushdown: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will extract leaf expressions
(such as `get_field`) from filter/sort/join nodes into projections
closer to the leaf table scans, and push those projections down
towards the leaf nodes.

<a id="op-7a0e28f70e7ec25e550f66d6"></a>
## enable_physical_uncorrelated_scalar_subquery

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_physical_uncorrelated_scalar_subquery` · datafusion-common 55.1.0

```rust
enable_physical_uncorrelated_scalar_subquery: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, uncorrelated scalar subqueries are
left in the logical plan and executed by `ScalarSubqueryExec` during
physical execution. When set to false, all scalar subqueries
(including uncorrelated ones) are rewritten to left joins by the
`ScalarSubqueryToJoin` optimizer rule.

Note disabling this option is not recommended. It restores
pre <https://github.com/apache/datafusion/pull/21240>
behavior, which silently produces incorrect results for
multi-row subqueries and does not support scalar subqueries in
ORDER BY / JOIN ON / aggregate-function arguments. This option is
intended as a temporary escape hatch for distributed execution
frameworks and is planned to be removed in a future DataFusion
release.

<a id="op-186d7ea0b3601305127422e4"></a>
## enable_piecewise_merge_join

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_piecewise_merge_join` · datafusion-common 55.1.0

```rust
enable_piecewise_merge_join: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, piecewise merge join is enabled. PiecewiseMergeJoin is currently
experimental. Physical planner will opt for PiecewiseMergeJoin when there is only
one range filter.

<a id="op-83db8ee19f05522a8ed3be7d"></a>
## enable_round_robin_repartition

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_round_robin_repartition` · datafusion-common 55.1.0

```rust
enable_round_robin_repartition: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the physical plan optimizer will try to add round robin
repartitioning to increase parallelism to leverage more CPU cores

<a id="op-3c71d119781253e6474582c5"></a>
## enable_sort_pushdown

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_sort_pushdown` · datafusion-common 55.1.0

```rust
enable_sort_pushdown: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Enable sort pushdown optimization.
When enabled, attempts to push sort requirements down to data sources
that can natively handle them (e.g., by reversing file/row group read order).

Returns **inexact ordering**: Sort operator is kept for correctness,
but optimized input enables early termination for TopK queries (ORDER BY ... LIMIT N),
providing significant speedup.

Memory: No additional overhead (only changes read order).

Future: Will add option to detect perfectly sorted data and eliminate Sort completely.

Default: true

<a id="op-8fa5c63d916ac979d6706d0a"></a>
## enable_topk_aggregation

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_topk_aggregation` · datafusion-common 55.1.0

```rust
enable_topk_aggregation: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will attempt to perform limit operations
during aggregations, if possible

<a id="op-10125cfc1de6229cc94a27ac"></a>
## enable_topk_dynamic_filter_pushdown

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_topk_dynamic_filter_pushdown` · datafusion-common 55.1.0

```rust
enable_topk_dynamic_filter_pushdown: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will attempt to push down TopK dynamic filters
into the file scan phase.

<a id="op-e3bbb213737bc49d82efe5cd"></a>
## enable_topk_repartition

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_topk_repartition` · datafusion-common 55.1.0

```rust
enable_topk_repartition: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will push TopK (Sort with fetch)
below hash repartition when the partition key is a prefix of the
sort key, reducing data volume before the shuffle.

<a id="op-3a51349dc22a0f6574e5d360"></a>
## enable_unions_to_filter

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_unions_to_filter` · datafusion-common 55.1.0

```rust
enable_unions_to_filter: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the logical optimizer will rewrite `UNION DISTINCT` branches that
read from the same source and differ only by filter predicates into a single branch
with a combined filter. This optimization is conservative and only applies when the
branches share the same source and compatible wrapper nodes such as identical
projections or aliases.

<a id="op-58363ab203ee37ffd5be274b"></a>
## enable_window_limits

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_window_limits` · datafusion-common 55.1.0

```rust
enable_window_limits: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will attempt to push limit operations
past window functions, if possible

<a id="op-9f99c15c9c83f95f0940c8ca"></a>
## enable_window_topn

`struct_field` · `datafusion_common::config::OptimizerOptions::enable_window_topn` · datafusion-common 55.1.0

```rust
enable_window_topn: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will replace
Filter(rn<=K) → Window(ROW_NUMBER) → Sort patterns with a
PartitionedTopKExec that maintains per-partition heaps, avoiding
a full sort of the input.
When the window partition key has low cardinality, enabling this optimization
can improve performance. However, for high cardinality keys, it may
cause regressions in both memory usage and runtime.

<a id="op-269135417b9ddfe79380669c"></a>
## eq

`function` · `datafusion_common::config::OptimizerOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &OptimizerOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::OptimizerOptions", "path": "OptimizerOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1769, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0228b1b4e17225d40fd903dd"></a>
## expand_views_at_output

`struct_field` · `datafusion_common::config::OptimizerOptions::expand_views_at_output` · datafusion-common 55.1.0

```rust
expand_views_at_output: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, if the returned type is a view type
then the output will be coerced to a non-view.
Coerces `Utf8View` to `LargeUtf8`, and `BinaryView` to `LargeBinary`.

<a id="op-93ad78e2afe502346306883a"></a>
## filter_null_join_keys

`struct_field` · `datafusion_common::config::OptimizerOptions::filter_null_join_keys` · datafusion-common 55.1.0

```rust
filter_null_join_keys: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will insert filters before a join between
a nullable and non-nullable column to filter out nulls on the nullable side. This
filter can add additional overhead when the file format does not fully support
predicate push down.

<a id="op-bbf25183d37251461a8173f8"></a>
## fmt

`function` · `datafusion_common::config::OptimizerOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::OptimizerOptions", "path": "OptimizerOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1769, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9041ab1b3eef65428a91c51"></a>
## hash_join_inlist_pushdown_max_distinct_values

`struct_field` · `datafusion_common::config::OptimizerOptions::hash_join_inlist_pushdown_max_distinct_values` · datafusion-common 55.1.0

```rust
hash_join_inlist_pushdown_max_distinct_values: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maximum number of distinct values (rows) in the build side of a hash join to be pushed down as an InList expression for dynamic filtering.
Build sides with more rows than this will use hash table lookups instead.
Set to 0 to always use hash table lookups.

This provides an additional limit beyond `hash_join_inlist_pushdown_max_size` to prevent
very large IN lists that might not provide much benefit over hash table lookups.

This uses the deduplicated row count once the build side has been evaluated.

The default is 150 values per partition.
This is inspired by Trino's `max-filter-keys-per-column` setting.
See: <https://trino.io/docs/current/admin/dynamic-filtering.html#dynamic-filter-collection-thresholds>

<a id="op-476ec1bf418c8f4c94357589"></a>
## hash_join_inlist_pushdown_max_size

`struct_field` · `datafusion_common::config::OptimizerOptions::hash_join_inlist_pushdown_max_size` · datafusion-common 55.1.0

```rust
hash_join_inlist_pushdown_max_size: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maximum size in bytes for the build side of a hash join to be pushed down as an InList expression for dynamic filtering.
Build sides larger than this will use hash table lookups instead.
Set to 0 to always use hash table lookups.

InList pushdown can be more efficient for small build sides because it can result in better
statistics pruning as well as use any bloom filters present on the scan side.
InList expressions are also more transparent and easier to serialize over the network in distributed uses of DataFusion.
On the other hand InList pushdown requires making a copy of the data and thus adds some overhead to the build side and uses more memory.

This setting is per-partition, so we may end up using `hash_join_inlist_pushdown_max_size` * `target_partitions` memory.

The default is 128kB per partition.
This should allow point lookup joins (e.g. joining on a unique primary key) to use InList pushdown in most cases
but avoids excessive memory usage or overhead for larger joins.

<a id="op-adb1e561cada1f809da0accc"></a>
## hash_join_single_partition_threshold

`struct_field` · `datafusion_common::config::OptimizerOptions::hash_join_single_partition_threshold` · datafusion-common 55.1.0

```rust
hash_join_single_partition_threshold: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The maximum estimated size in bytes for one input side of a HashJoin
will be collected into a single partition

<a id="op-84cef8dd06fb74b1b8ed62a9"></a>
## hash_join_single_partition_threshold_rows

`struct_field` · `datafusion_common::config::OptimizerOptions::hash_join_single_partition_threshold_rows` · datafusion-common 55.1.0

```rust
hash_join_single_partition_threshold_rows: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The maximum estimated size in rows for one input side of a HashJoin
will be collected into a single partition

<a id="op-ee08c88df7fd8f16ce67076b"></a>
## join_reordering

`struct_field` · `datafusion_common::config::OptimizerOptions::join_reordering` · datafusion-common 55.1.0

```rust
join_reordering: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the physical plan optimizer may swap join inputs
based on statistics. When set to false, statistics-driven join
input reordering is disabled and the original join order in the
query is used.

<a id="op-75f81d50b6c729949241f450"></a>
## max_passes

`struct_field` · `datafusion_common::config::OptimizerOptions::max_passes` · datafusion-common 55.1.0

```rust
max_passes: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of times that the optimizer will attempt to optimize the plan

<a id="op-31096c6a01db3303f22d738e"></a>
## prefer_existing_sort

`struct_field` · `datafusion_common::config::OptimizerOptions::prefer_existing_sort` · datafusion-common 55.1.0

```rust
prefer_existing_sort: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When true, DataFusion will opportunistically remove sorts when the data is already sorted,
(i.e. setting `preserve_order` to true on `RepartitionExec`  and
using `SortPreservingMergeExec`)

When false, DataFusion will maximize plan parallelism using
`RepartitionExec` even if this requires subsequently resorting data using a `SortExec`.

<a id="op-812747914ae857ca562173d6"></a>
## prefer_existing_union

`struct_field` · `datafusion_common::config::OptimizerOptions::prefer_existing_union` · datafusion-common 55.1.0

```rust
prefer_existing_union: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the optimizer will not attempt to convert Union to Interleave

<a id="op-066b860366f4208542a7c0b4"></a>
## prefer_hash_join

`struct_field` · `datafusion_common::config::OptimizerOptions::prefer_hash_join` · datafusion-common 55.1.0

```rust
prefer_hash_join: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the physical plan optimizer will prefer HashJoin over SortMergeJoin.
HashJoin can work more efficiently than SortMergeJoin but consumes more memory

<a id="op-20ece8e7353ef4a7329f995b"></a>
## preserve_file_partitions

`struct_field` · `datafusion_common::config::OptimizerOptions::preserve_file_partitions` · datafusion-common 55.1.0

```rust
preserve_file_partitions: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Minimum number of distinct partition values required to group files by their
Hive partition column values (enabling output partitioning declaration).

How the option is used:
    - preserve_file_partitions=0: Disable it.
    - preserve_file_partitions=1: Always enable it.
    - preserve_file_partitions=N, actual file partitions=M: Only enable when M >= N.
    This threshold preserves I/O parallelism when file partitioning is below it.

Note: This may reduce parallelism, rooting from the I/O level, if the number of distinct
partitions is less than the target_partitions.

<a id="op-ba1a89a998944c2e46d77a16"></a>
## repartition_aggregations

`struct_field` · `datafusion_common::config::OptimizerOptions::repartition_aggregations` · datafusion-common 55.1.0

```rust
repartition_aggregations: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion repartition data using the aggregate keys to execute aggregates
in parallel using the provided `target_partitions` level

<a id="op-bc0047cefe97dc4e4f24e97c"></a>
## repartition_file_min_size

`struct_field` · `datafusion_common::config::OptimizerOptions::repartition_file_min_size` · datafusion-common 55.1.0

```rust
repartition_file_min_size: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Minimum total file size in bytes for file-group byte-range
splitting to fire. Files (or merged file groups) smaller than this
stay as one partition. Lower values produce more, smaller
partitions — better at filling `target_partitions` worth of cores
when files are modestly sized, at the cost of slightly more
per-partition open / metadata-load overhead.

<a id="op-e278a3338862f845cb6fa62e"></a>
## repartition_file_scans

`struct_field` · `datafusion_common::config::OptimizerOptions::repartition_file_scans` · datafusion-common 55.1.0

```rust
repartition_file_scans: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to `true`, datasource partitions will be repartitioned to achieve maximum parallelism.
This applies to both in-memory partitions and FileSource's file groups (1 group is 1 partition).

For FileSources, only Parquet and CSV formats are currently supported.

If set to `true` for a FileSource, all files will be repartitioned evenly (i.e., a single large file
might be partitioned into smaller chunks) for parallel scanning.
If set to `false` for a FileSource, different files will be read in parallel, but repartitioning won't
happen within a single file.

If set to `true` for an in-memory source, all memtable's partitions will have their batches
repartitioned evenly to the desired number of `target_partitions`. Repartitioning can change
the total number of partitions and batches per partition, but does not slice the initial
record tables provided to the MemTable on creation.

<a id="op-a1f0d8e238bd2c6d137e8291"></a>
## repartition_joins

`struct_field` · `datafusion_common::config::OptimizerOptions::repartition_joins` · datafusion-common 55.1.0

```rust
repartition_joins: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion repartition data using the join keys to execute joins in parallel
using the provided `target_partitions` level

<a id="op-f7e33147c14d4c571b6aa4ba"></a>
## repartition_sorts

`struct_field` · `datafusion_common::config::OptimizerOptions::repartition_sorts` · datafusion-common 55.1.0

```rust
repartition_sorts: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion execute sorts in a per-partition fashion and merge
afterwards instead of coalescing first and sorting globally.
With this flag is enabled, plans in the form below

```text
     "SortExec: [a@0 ASC]",
     "  CoalescePartitionsExec",
     "    RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```
would turn into the plan below which performs better in multithreaded environments

```text
     "SortPreservingMergeExec: [a@0 ASC]",
     "  SortExec: [a@0 ASC]",
     "    RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```

<a id="op-8737a0ca50d4bf6e8f09dc2e"></a>
## repartition_windows

`struct_field` · `datafusion_common::config::OptimizerOptions::repartition_windows` · datafusion-common 55.1.0

```rust
repartition_windows: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should DataFusion repartition data using the partitions keys to execute window
functions in parallel using the provided `target_partitions` level

<a id="op-6ac69655e0913e2b73faf31e"></a>
## reset

`function` · `datafusion_common::config::OptimizerOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::OptimizerOptions", "path": "OptimizerOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1769, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98f6f86976eb30eff321cea1"></a>
## set

`function` · `datafusion_common::config::OptimizerOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::OptimizerOptions", "path": "OptimizerOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1769, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d981d7277e101311e2f6bbc8"></a>
## skip_failed_rules

`struct_field` · `datafusion_common::config::OptimizerOptions::skip_failed_rules` · datafusion-common 55.1.0

```rust
skip_failed_rules: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the logical plan optimizer will produce warning
messages if any optimization rules produce errors and then proceed to the next
rule. When set to false, any rules that produce errors will cause the query to fail

<a id="op-8d7fe0c4f1182b9d85c84b57"></a>
## subset_repartition_threshold

`struct_field` · `datafusion_common::config::OptimizerOptions::subset_repartition_threshold` · datafusion-common 55.1.0

```rust
subset_repartition_threshold: usize
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Partition count threshold for subset satisfaction optimization.

When the current partition count is >= this threshold, DataFusion will
skip repartitioning if the required partitioning expression is a subset
of the current partition expression such as Hash(a) satisfies Hash(a, b).

When the current partition count is < this threshold, DataFusion will
repartition to increase parallelism even when subset satisfaction applies.

Set to 0 to always repartition (disable subset satisfaction optimization).
Set to a high value to always use subset satisfaction.

Example (subset_repartition_threshold = 4):
```text
    Hash([a]) satisfies Hash([a, b]) because (Hash([a, b]) is subset of Hash([a])

    If current partitions (3) < threshold (4), repartition:
    AggregateExec: mode=FinalPartitioned, gby=[a, b], aggr=[SUM(x)]
      RepartitionExec: partitioning=Hash([a, b], 8), input_partitions=3
        AggregateExec: mode=Partial, gby=[a, b], aggr=[SUM(x)]
          DataSourceExec: file_groups={...}, output_partitioning=Hash([a], 3)

    If current partitions (8) >= threshold (4), use subset satisfaction:
    AggregateExec: mode=SinglePartitioned, gby=[a, b], aggr=[SUM(x)]
      DataSourceExec: file_groups={...}, output_partitioning=Hash([a], 8)
```

<a id="op-d23215a4d6d8fe774c92c026"></a>
## top_down_join_key_reordering

`struct_field` · `datafusion_common::config::OptimizerOptions::top_down_join_key_reordering` · datafusion-common 55.1.0

```rust
top_down_join_key_reordering: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the physical plan optimizer will run a top down
process to reorder the join keys

<a id="op-b0000f45ef3b6396c345e686"></a>
## use_statistics_registry

`struct_field` · `datafusion_common::config::OptimizerOptions::use_statistics_registry` · datafusion-common 55.1.0

```rust
use_statistics_registry: bool
```

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the physical plan optimizer uses the pluggable
`StatisticsRegistry` for statistics propagation across operators.
This enables more accurate cardinality estimates compared to each
operator's built-in `partition_statistics`.

<a id="op-817d25e275d2d40a0d6813ee"></a>
## visit

`function` · `datafusion_common::config::OptimizerOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::OptimizerOptions", "path": "OptimizerOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1769, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
