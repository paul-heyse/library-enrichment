# `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.json).

<a id="op-41aad8f48fa144bd1cf86810"></a>
## HashJoinExec

`struct` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec` · datafusion-physical-plan 55.1.0

```rust
struct HashJoinExec
```

Source: `src/joins/hash_join/exec.rs:739`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Join execution plan: Evaluates equijoin predicates in parallel on multiple
partitions using a hash table and an optional filter list to apply post
join.

# Join Expressions

This implementation is optimized for evaluating equijoin predicates  (
`<col1> = <col2>`) expressions, which are represented as a list of `Columns`
in [`Self::on`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-69f99fe7a897d1524302b6fe).

Non-equality predicates, which can not pushed down to a join inputs (e.g.
`<col1> != <col2>`) are known as "filter expressions" and are evaluated
after the equijoin predicates.

# ArrayMap Optimization

For joins with a single integer-based join key, `HashJoinExec` may use an [`ArrayMap`](../operations/datafusion_physical_plan.joins.array_map.ArrayMap.md#op-1ef6fc1a0cd369e0c95a3ce2)
(also known as a "perfect hash join") instead of a general-purpose hash map.
This optimization is used when:
1. There is exactly one join key.
2. The join key is an integer type up to 64 bits wide that can be losslessly converted
   to `u64` (128-bit integer types such as `i128` and `u128` are not supported).
3. The range of keys is small enough (controlled by `perfect_hash_join_small_build_threshold`)
   OR the keys are sufficiently dense (controlled by `perfect_hash_join_min_key_density`).
4. build_side.num_rows() < u32::MAX
5. NullEqualsNothing || (NullEqualsNull && build side doesn't contain null)

See [`try_create_array_map`] for more details.

Note that when using [`PartitionMode::Partitioned`](../operations/datafusion_physical_plan.joins.PartitionMode.md#op-b71396d1d1725b5437c17c9b), the build side is split into multiple
partitions. This can cause a dense build side to become sparse within each partition,
potentially disabling this optimization.

For example, consider:
```sql
SELECT t1.value, t2.value
FROM range(10000) AS t1
JOIN range(10000) AS t2
  ON t1.value = t2.value;
```
With 24 partitions, each partition will only receive a subset of the 10,000 rows.
The first partition might contain values like `3, 10, 18, 39, 43`, which are sparse
relative to the original range, even though the overall data set is dense.

# "Build Side" vs "Probe Side"

HashJoin takes two inputs, which are referred to as the "build" and the
"probe". The build side is the first child, and the probe side is the second
child.

The two inputs are treated differently and it is VERY important that the
*smaller* input is placed on the build side to minimize the work of creating
the hash table.

```text
         ┌───────────┐
         │ HashJoin  │
         │           │
         └───────────┘
             │   │
       ┌─────┘   └─────┐
       ▼               ▼
┌────────────┐  ┌─────────────┐
│   Input    │  │    Input    │
│    [0]     │  │     [1]     │
└────────────┘  └─────────────┘

 "build side"    "probe side"
```

Execution proceeds in 2 stages:

1. the **build phase** creates a hash table from the tuples of the build side,
   and single concatenated batch containing data from all fetched record batches.
   Resulting hash table stores hashed join-key fields for each row as a key, and
   indices of corresponding rows in concatenated batch.

When using the standard `JoinHashMap`, hash join uses LIFO data structure as a hash table,
and in order to retain original build-side input order while obtaining data during probe phase,
hash table is updated by iterating batch sequence in reverse order -- it allows to
keep rows with smaller indices "on the top" of hash table, and still maintain
correct indexing for concatenated build-side data batch.

Example of build phase for 3 record batches:


```text

 Original build-side data   Inserting build-side values into hashmap    Concatenated build-side batch
                                                                        ┌───────────────────────────┐
                            hashmap.insert(row-hash, row-idx + offset)  │                      idx  │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 1 │        1) update_hash for batch 3 with offset 0    │          │ Row 6 │    0   │
  Batch 1  │       │           - hashmap.insert(Row 7, idx 1)           │ Batch 3  │       │        │
           │ Row 2 │           - hashmap.insert(Row 6, idx 0)           │          │ Row 7 │    1   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 3 │        2) update_hash for batch 2 with offset 2    │          │ Row 3 │    2   │
           │       │           - hashmap.insert(Row 5, idx 4)           │          │       │        │
  Batch 2  │ Row 4 │           - hashmap.insert(Row 4, idx 3)           │ Batch 2  │ Row 4 │    3   │
           │       │           - hashmap.insert(Row 3, idx 2)           │          │       │        │
           │ Row 5 │                                                    │          │ Row 5 │    4   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 6 │        3) update_hash for batch 1 with offset 5    │          │ Row 1 │    5   │
  Batch 3  │       │           - hashmap.insert(Row 2, idx 6)           │ Batch 1  │       │        │
           │ Row 7 │           - hashmap.insert(Row 1, idx 5)           │          │ Row 2 │    6   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
                                                                        └───────────────────────────┘
```

2. the **probe phase** where the tuples of the probe side are streamed
   through, checking for matches of the join keys in the hash table.

```text
                ┌────────────────┐          ┌────────────────┐
                │ ┌─────────┐    │          │ ┌─────────┐    │
                │ │  Hash   │    │          │ │  Hash   │    │
                │ │  Table  │    │          │ │  Table  │    │
                │ │(keys are│    │          │ │(keys are│    │
                │ │equi join│    │          │ │equi join│    │  Stage 2: batches from
 Stage 1: the   │ │columns) │    │          │ │columns) │    │    the probe side are
*entire* build  │ │         │    │          │ │         │    │  streamed through, and
 side is read   │ └─────────┘    │          │ └─────────┘    │   checked against the
into the hash   │      ▲         │          │          ▲     │   contents of the hash
    table       │       HashJoin │          │  HashJoin      │          table
                └──────┼─────────┘          └──────────┼─────┘
            ─ ─ ─ ─ ─ ─                                 ─ ─ ─ ─ ─ ─ ─
           │                                                         │

           │                                                         │
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘
          ...                                                       ...
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘

       build side                                                probe side
```

# Example "Optimal" Plans

The differences in the inputs means that for classic "Star Schema Query",
the optimal plan will be a **"Right Deep Tree"** . A Star Schema Query is
one where there is one large table and several smaller "dimension" tables,
joined on `Foreign Key = Primary Key` predicates.

A "Right Deep Tree" looks like this large table as the probe side on the
lowest join:

```text
            ┌───────────┐
            │ HashJoin  │
            │           │
            └───────────┘
                │   │
        ┌───────┘   └──────────┐
        ▼                      ▼
┌───────────────┐        ┌───────────┐
│ small table 1 │        │ HashJoin  │
│  "dimension"  │        │           │
└───────────────┘        └───┬───┬───┘
                  ┌──────────┘   └───────┐
                  │                      │
                  ▼                      ▼
          ┌───────────────┐        ┌───────────┐
          │ small table 2 │        │ HashJoin  │
          │  "dimension"  │        │           │
          └───────────────┘        └───┬───┬───┘
                              ┌────────┘   └────────┐
                              │                     │
                              ▼                     ▼
                      ┌───────────────┐     ┌───────────────┐
                      │ small table 3 │     │  large table  │
                      │  "dimension"  │     │    "fact"     │
                      └───────────────┘     └───────────────┘
```

# Clone / Shared State

Note this structure includes a [`OnceAsync`] that is used to coordinate the
loading of the left side with the processing in each output stream.
Therefore it can not be [`Clone`]

Unresolved upstream links (retained, not inferred): ``try_create_array_map``, ``OnceAsync``, ``Clone``.

<a id="op-b6aac3abfc52f13a8c98822f"></a>
## apply_expressions

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1347`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c62a7a20a8b1c441896b13a"></a>
## builder

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::builder` · datafusion-physical-plan 55.1.0

```rust
fn builder(&self) -> HashJoinExecBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:852`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a builder based on the existing [`HashJoinExec`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810).

Returned builder preserves all existing fields. If a field requiring properties
recomputation is modified, this will be done automatically during the node build.


<a id="op-27d55addc77a65d936142050"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1586`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a68d2f8ab37bab33cff24cc"></a>
## children

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1343`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-310860d554481b6c5c85d263"></a>
## contains_projection

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::contains_projection` · datafusion-physical-plan 55.1.0

```rust
fn contains_projection(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:1032`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return whether the join contains a projection

<a id="op-afc1c37eec538bab446a541d"></a>
## dynamic_expressions_produced

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::dynamic_expressions_produced` · datafusion-physical-plan 55.1.0

```rust
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1366`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd5703ad3b3c4e434b7c1dd8"></a>
## dynamic_filter_expr

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::dynamic_filter_expr` · datafusion-physical-plan 55.1.0

```rust
fn dynamic_filter_expr(&self) -> Option<&Arc<DynamicFilterPhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:983`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the dynamic filter expression produced by this hash join, if set.

<a id="op-8aa074e64a966a5010cc62bd"></a>
## execute

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1413`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-815bb21fcc8c34d6c21d200c"></a>
## fetch

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1829`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a978b7f0138b259b048672d"></a>
## filter

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::filter` · datafusion-physical-plan 55.1.0

```rust
fn filter(&self) -> Option<&JoinFilter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:953`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Filters applied before join output

<a id="op-f0df9af74d9a9de57fcb5182"></a>
## filter

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::filter` · datafusion-physical-plan 55.1.0

```rust
filter: Option<joins::utils::JoinFilter>
```

Source: `src/joins/hash_join/exec.rs:747`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Filters which are applied while finding matching rows

<a id="op-aac68ff810fc63323700064a"></a>
## fmt

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [793, 1], "end": [813, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/hash_join/exec.rs:794`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6ece7e4c8c173e1c1cc9f74"></a>
## fmt_as

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1192, 1], "end": [1284, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/joins/hash_join/exec.rs:1193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9193fe459e4a11855533746"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1672`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-027bd9a301c2e85866ffb270"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1791`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62256018d1be7601a2f939bc"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1299`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08e86963549e6791b84f3633"></a>
## join_schema

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::join_schema` · datafusion-physical-plan 55.1.0

```rust
fn join_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:964`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The schema after join. Please be careful when using this schema,
if there is a projection, the schema isn't the same as the output schema.

<a id="op-216c444d40136a935e8546c5"></a>
## join_type

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
fn join_type(&self) -> &JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:958`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

How the join is performed

<a id="op-372feb50e8a2cf582df4c17e"></a>
## join_type

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
join_type: datafusion_common::JoinType
```

Source: `src/joins/hash_join/exec.rs:749`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

How the join is performed (`OUTER`, `INNER`, etc)

<a id="op-5255afa4a8affb737570995e"></a>
## left

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
left: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/hash_join/exec.rs:741`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

left (build) side which gets hashed

<a id="op-fb0b36c4f03b0a1846bdeccd"></a>
## left

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
fn left(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:938`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

left (build) side which gets hashed

<a id="op-1641b356e1ad1a1896d15401"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96ccccc8ef58b59de0422141"></a>
## metrics

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1582`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9d1a90b355944a07802520d"></a>
## mode

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::mode` · datafusion-physical-plan 55.1.0

```rust
mode: joins::PartitionMode
```

Source: `src/joins/hash_join/exec.rs:763`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Partitioning mode to use

<a id="op-b212162c4702d3e236acd089"></a>
## name

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1287`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3540d8187c00f07474e18bb5"></a>
## null_aware

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::null_aware` · datafusion-physical-plan 55.1.0

```rust
null_aware: bool
```

Source: `src/joins/hash_join/exec.rs:773`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Flag to indicate if this is a null-aware anti join

<a id="op-80a0ed7149a51d8ac4d6725c"></a>
## null_equality

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::null_equality` · datafusion-physical-plan 55.1.0

```rust
null_equality: datafusion_common::NullEquality
```

Source: `src/joins/hash_join/exec.rs:771`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The equality null-handling behavior of the join algorithm.

<a id="op-ed73eb162762d055ffecdf08"></a>
## null_equality

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::null_equality` · datafusion-physical-plan 55.1.0

```rust
fn null_equality(&self) -> NullEquality
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get null_equality

<a id="op-69f99fe7a897d1524302b6fe"></a>
## on

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::on` · datafusion-physical-plan 55.1.0

```rust
fn on(&self) -> &[(PhysicalExprRef, PhysicalExprRef)]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:948`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set of common columns used to join on

<a id="op-b77794c93a9cc8019d36a356"></a>
## on

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::on` · datafusion-physical-plan 55.1.0

```rust
on: Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>
```

Source: `src/joins/hash_join/exec.rs:745`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set of equijoin columns from the relations: `(left_col, right_col)`

<a id="op-8b05579f2c94a9ed2e36d3bc"></a>
## partition_mode

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::partition_mode` · datafusion-physical-plan 55.1.0

```rust
fn partition_mode(&self) -> &PartitionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:969`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The partitioning mode of this hash join

<a id="op-6bbb08a3946738abd9221020"></a>
## probe_side

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::probe_side` · datafusion-physical-plan 55.1.0

```rust
fn probe_side() -> JoinSide
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:1026`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get probe side information for the hash join.

<a id="op-a9766b08ffaa2f771e45daaa"></a>
## projection

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::projection` · datafusion-physical-plan 55.1.0

```rust
projection: Option<datafusion_physical_expr::projection::ProjectionRef>
```

Source: `src/joins/hash_join/exec.rs:767`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The projection indices of the columns in the output schema of join

<a id="op-9cb2e61700f762d58a4af576"></a>
## properties

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1291`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c10874d90ae338085408fdb3"></a>
## replace_children

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1381`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a new HashJoinExec with different children while preserving configuration.

This method is called during query optimization when the optimizer creates new
plan nodes. Importantly, it creates a fresh bounds_accumulator via `try_new`
rather than cloning the existing one because partitioning may have changed.

<a id="op-a8599c3151ad60144bd3bb3c"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af25fbb1697d12fe80f142bf"></a>
## reset_state

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1409`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07fb1acc40fc9ce1d3759d49"></a>
## right

`struct_field` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
right: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/hash_join/exec.rs:743`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

right (probe) side which are filtered by the hash table

<a id="op-e3556dc1c46bcd8d5c36b29a"></a>
## right

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
fn right(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:943`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

right (probe) side which are filtered by the hash table

<a id="op-1dda1fbcfe5499572307bb9a"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1608`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6085b48bdf6f7ddbea243a26"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1822`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab8ffc5ca4894926893b682e"></a>
## swap_inputs

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::swap_inputs` · datafusion-physical-plan 55.1.0

```rust
fn swap_inputs(&self, partition_mode: PartitionMode) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:1143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a new `ExecutionPlan` that computes the same join as this one,
with the left and right inputs swapped using the  specified
`partition_mode`.

# Notes:

This function is public so other downstream projects can use it to
construct `HashJoinExec` with right side as the build side.

For using this interface directly, please refer to below:

Hash join execution may require specific input partitioning (for example,
the left child may have a single partition while the right child has multiple).

Calling this function on join nodes whose children have already been repartitioned
(e.g., after a `RepartitionExec` has been inserted) may break the partitioning
requirements of the hash join. Therefore, ensure you call this function
before inserting any repartitioning operators on the join's children.

In DataFusion's default SQL interface, this function is used by the `JoinSelection`
physical optimizer rule to determine a good join order, which is
executed before the `EnforceDistribution` rule (the rule that may
insert `RepartitionExec` operators).

<a id="op-97376fbf7dabb8a947ec936f"></a>
## try_from_proto

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1918, 1], "end": [2048, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:1920`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`HashJoinExec`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810) from its protobuf representation.

<a id="op-f8cadf700f311cb4fb8adb13"></a>
## try_new

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: JoinOn, filter: Option<JoinFilter>, join_type: &JoinType, projection: Option<Vec<usize>>, partition_mode: PartitionMode, null_equality: NullEquality, null_aware: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:827`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to create a new [`HashJoinExec`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810).

# Error
This function errors when it is not possible to join the left and right sides on keys `on`.

<a id="op-212021920acb192636294d8b"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1632`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to push `projection` down through `hash_join`. If possible, performs the
pushdown and returns a new [`HashJoinExec`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810) as the top plan which has projections
as its children. Otherwise, returns `None`.

<a id="op-a840fc2b045cbc9178797a58"></a>
## try_to_proto

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1841`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb58e6b2f3f67d6436b33022"></a>
## with_dynamic_filter_expr

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::with_dynamic_filter_expr` · datafusion-physical-plan 55.1.0

```rust
fn with_dynamic_filter_expr(self, filter: Arc<DynamicFilterPhysicalExpr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:993`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the dynamic filter on this hash join.

Resets any internal state that depends on any existing dynamic filter.

Validates that the filter's children reference valid columns in
the probe (right) side's schema.

<a id="op-3b3af3cb71d42894e8fa77a8"></a>
## with_fetch

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1833`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9bfb2a8048e253aa276e6fa"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1286, 1], "end": [1915, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/hash_join/exec.rs:1399`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0279fa34f9c32e8b605f9563"></a>
## with_projection

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [821, 1], "end": [1190, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:1037`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return new instance of [HashJoinExec](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810) with the given projection.

<a id="op-43c9cb7a5ec962bbd14dcd79"></a>
## with_projection

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [815, 1], "end": [819, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::projection::EmbeddedProjection", "path": "EmbeddedProjection"}, "trait_path": "datafusion_physical_plan::projection::EmbeddedProjection"}`

Source: `src/joins/hash_join/exec.rs:816`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
