# `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.row_group_filter.RowGroupAccessPlanFilter.json).

<a id="op-9a6dde4da62936de432fc194"></a>
## RowGroupAccessPlanFilter

`struct` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter` · datafusion-datasource-parquet 55.1.0

```rust
struct RowGroupAccessPlanFilter
```

Source: `src/row_group_filter.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Reduces the [`ParquetAccessPlan`](../operations/datafusion_datasource_parquet.access_plan.ParquetAccessPlan.md#op-2786e9792f5bf477380471b9) based on row group level metadata.

This struct implements the various types of pruning that are applied to a
set of row groups within a parquet file, progressively narrowing down the
set of row groups (and ranges/selections within those row groups) that
should be scanned, based on the available metadata.

<a id="op-23e5c670d1c9add58bb780ef"></a>
## build

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::build` · datafusion-datasource-parquet 55.1.0

```rust
fn build(self) -> ParquetAccessPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Returns the inner access plan.

<a id="op-362b18b7d6985ea63a481569"></a>
## clone

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> RowGroupAccessPlanFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 17], "end": [43, 22], "filename": "src/row_group_filter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/row_group_filter.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0a0605f8bf39f538020cc72"></a>
## eq

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::eq` · datafusion-datasource-parquet 55.1.0

```rust
fn eq(&self, other: &RowGroupAccessPlanFilter) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 24], "end": [43, 33], "filename": "src/row_group_filter.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/row_group_filter.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2eb3b198205ace50db1d5cfd"></a>
## fmt

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/row_group_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/row_group_filter.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a127eb0c41e24db16fc3262d"></a>
## is_empty

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::is_empty` · datafusion-datasource-parquet 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return true if there are no row groups

<a id="op-da7011c601cdd0a2000ee34f"></a>
## is_fully_matched

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::is_fully_matched` · datafusion-datasource-parquet 55.1.0

```rust
fn is_fully_matched(&self) -> &Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Returns the is_fully_matched vector.

<a id="op-18ca5aff13ccb8e26d3bd5d6"></a>
## new

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(access_plan: ParquetAccessPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new `RowGroupPlanBuilder` for pruning out the groups to scan
based on metadata and statistics

<a id="op-9230816edffc688177472a61"></a>
## prune_by_bloom_filters

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::prune_by_bloom_filters` · datafusion-datasource-parquet 55.1.0

```rust
fn prune_by_bloom_filters(&mut self, predicate: &PruningPredicate, metrics: &ParquetFileMetrics, row_group_bloom_filters: &[BloomFilterStatistics])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Prune remaining row groups using loaded bloom filters and the
[`PruningPredicate`](../operations/datafusion_pruning.pruning_predicate.PruningPredicate.md#op-ee1b051e6137f40583414068).

Updates this set with row groups that should not be scanned.
`row_group_bloom_filters[idx]` contains the bloom filters for the
parquet row group at index `idx`.

# Panics
if `row_group_bloom_filters` does not have the same number of row groups as this set

<a id="op-df31b329e3eedeb6fa2db943"></a>
## prune_by_limit

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::prune_by_limit` · datafusion-datasource-parquet 55.1.0

```rust
fn prune_by_limit(&mut self, limit: usize, rg_metadata: &[RowGroupMetaData], metrics: &ParquetFileMetrics)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Prunes the access plan based on the limit and fully contained row groups.

The pruning works by leveraging the concept of fully matched row groups. Consider a query like:
`WHERE species LIKE 'Alpine%' AND s >= 50 LIMIT N`

After initial filtering, row groups can be classified into three states:

1. Not Matching / Pruned
2. Partially Matching (Row Group/Page contains some matches)
3. Fully Matching (Entire range is within predicate)

+-----------------------------------------------------------------------+
|                            NOT MATCHING                               |
|  Row group 1                                                          |
|  +-----------------------------------+-----------------------------+  |
|  | SPECIES                           | S                           |  |
|  +-----------------------------------+-----------------------------+  |
|  | Snow Vole                         | 7                           |  |
|  | Brown Bear                        | 133 ✅                      |  |
|  | Gray Wolf                         | 82  ✅                      |  |
|  +-----------------------------------+-----------------------------+  |
+-----------------------------------------------------------------------+

+---------------------------------------------------------------------------+
|                          PARTIALLY MATCHING                               |
|                                                                           |
|  Row group 2                              Row group 4                     |
|  +------------------+--------------+      +------------------+----------+ |
|  | SPECIES          | S            |      | SPECIES          | S        | |
|  +------------------+--------------+      +------------------+----------+ |
|  | Lynx             | 71 ✅        |      | Europ. Mole      | 4        | |
|  | Red Fox          | 40           |      | Polecat          | 16       | |
|  | Alpine Bat  ✅   | 6            |      | Alpine Ibex ✅  | 97 ✅    | |
|  +------------------+--------------+      +------------------+----------+ |
+---------------------------------------------------------------------------+

+-----------------------------------------------------------------------+
|                           FULLY MATCHING                              |
|  Row group 3                                                          |
|  +-----------------------------------+-----------------------------+  |
|  | SPECIES                           | S                           |  |
|  +-----------------------------------+-----------------------------+  |
|  | Alpine Ibex  ✅                  | 101    ✅                   |  |
|  | Alpine Goat  ✅                  | 76     ✅                   |  |
|  | Alpine Sheep ✅                  | 83     ✅                   |  |
|  +-----------------------------------+-----------------------------+  |
+-----------------------------------------------------------------------+

### Identification of Fully Matching Row Groups

DataFusion identifies row groups where ALL rows satisfy the filter by inverting the
predicate and checking if statistics prove the inverted version is false for the group.

For example, prefix matches like `species LIKE 'Alpine%'` are pruned using ranges:
1. Candidate Range: `species >= 'Alpine' AND species < 'Alpinf'`
2. Inverted Condition (to prove full match): `species < 'Alpine' OR species >= 'Alpinf'`
3. Statistical Evaluation (check if any row *could* satisfy the inverted condition):
   `min < 'Alpine' OR max >= 'Alpinf'`

If this evaluation is **false**, it proves no row can fail the original filter,
so the row group is **FULLY MATCHING**.

### Impact of Statistics Truncation

The precision of pruning depends on the metadata quality. Truncated statistics
may prevent the system from proving a full match.

**Example**: `WHERE species LIKE 'Alpine%'` (Target range: `['Alpine', 'Alpinf')`)

| Truncation Length | min / max           | Inverted Evaluation                                                 | Status                 |
|-------------------|---------------------|---------------------------------------------------------------------|------------------------|
| **Length 6**      | `Alpine` / `Alpine` | `"Alpine" < "Alpine" (F) OR "Alpine" >= "Alpinf" (F)` -> **false**  | **FULLY MATCHING**     |
| **Length 3**      | `Alp` / `Alq`       | `"Alp" < "Alpine" (T) OR "Alq" >= "Alpinf" (T)` -> **true**         | **PARTIALLY MATCHING** |

Even though Row Group 3 only contains matching rows, truncation to length 3 makes
the statistics `[Alp, Alq]` too broad to prove it (they could include "Alpha").
The system must conservatively scan the group.

Without limit pruning: Scan Partition 2 → Partition 3 → Partition 4 (until limit reached)
With limit pruning: If Partition 3 contains enough rows to satisfy the limit,
skip Partitions 2 and 4 entirely and go directly to Partition 3.

This optimization is particularly effective when:
- The limit is small relative to the total dataset size
- There are row groups that are fully matched by the filter predicates
- The fully matched row groups contain sufficient rows to satisfy the limit

For more information, see the [paper](https://arxiv.org/pdf/2504.11540)'s "Pruning for LIMIT Queries" part

<a id="op-20c879a460b11eb7ffe009f1"></a>
## prune_by_range

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::prune_by_range` · datafusion-datasource-parquet 55.1.0

```rust
fn prune_by_range(&mut self, groups: &[RowGroupMetaData], range: &FileRange)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Prune remaining row groups to only those  within the specified range.

Updates this set to mark row groups that should not be scanned

# Panics
if `groups.len() != self.len()`

<a id="op-4b075a5c0d70ed0dab7280ab"></a>
## prune_by_statistics

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::prune_by_statistics` · datafusion-datasource-parquet 55.1.0

```rust
fn prune_by_statistics(&mut self, arrow_schema: &Schema, parquet_schema: &SchemaDescriptor, groups: &[RowGroupMetaData], predicate: &PruningPredicate, metrics: &ParquetFileMetrics)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Prune remaining row groups using min/max/null_count statistics and
the [`PruningPredicate`](../operations/datafusion_pruning.pruning_predicate.PruningPredicate.md#op-ee1b051e6137f40583414068) to determine if the predicate can not be true.

Updates this set to mark row groups that should not be scanned

Note: This method currently ignores ColumnOrder
<https://github.com/apache/datafusion/issues/8335>

# Panics
if `groups.len() != self.len()`

<a id="op-faf6e29a15c506e3a28faa58"></a>
## remaining_row_group_count

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::remaining_row_group_count` · datafusion-datasource-parquet 55.1.0

```rust
fn remaining_row_group_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return the number of row groups that are currently expected to be scanned

<a id="op-a01b570b90fd7e906cef877f"></a>
## row_group_indexes

`function` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter::row_group_indexes` · datafusion-datasource-parquet 55.1.0

```rust
fn row_group_indexes(&self) -> impl Iterator<Item = usize> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter", "path": "RowGroupAccessPlanFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [457, 2], "filename": "src/row_group_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_group_filter.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return indexes of row groups that still need to be scanned.
