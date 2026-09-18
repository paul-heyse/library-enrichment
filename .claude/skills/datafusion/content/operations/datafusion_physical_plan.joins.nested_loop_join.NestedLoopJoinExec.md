# `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.nested_loop_join.NestedLoopJoinExec.json).

<a id="op-879865d333b3afaee6f0226e"></a>
## NestedLoopJoinExec

`struct` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec` · datafusion-physical-plan 55.1.0

```rust
struct NestedLoopJoinExec
```

Source: `src/joins/nested_loop_join.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

NestedLoopJoinExec is a build-probe join operator designed for joins that
do not have equijoin keys in their `ON` clause.

# Execution Flow

```text
                                               Incoming right batch
               Left Side Buffered Batches
                      ┌───────────┐              ┌───────────────┐
                      │ ┌───────┐ │              │               │
                      │ │       │ │              │               │
 Current Left Row ───▶│ ├───────├─┤──────────┐   │               │
                      │ │       │ │          │   └───────────────┘
                      │ │       │ │          │           │
                      │ │       │ │          │           │
                      │ └───────┘ │          │           │
                      │ ┌───────┐ │          │           │
                      │ │       │ │          │     ┌─────┘
                      │ │       │ │          │     │
                      │ │       │ │          │     │
                      │ │       │ │          │     │
                      │ │       │ │          │     │
                      │ └───────┘ │          ▼     ▼
                      │   ......  │  ┌──────────────────────┐
                      │           │  │X (Cartesian Product) │
                      │           │  └──────────┬───────────┘
                      └───────────┘             │
                                                │
                                                ▼
                                     ┌───────┬───────────────┐
                                     │       │               │
                                     │       │               │
                                     │       │               │
                                     └───────┴───────────────┘
                                       Intermediate Batch
                                 (For join predicate evaluation)
```

The execution follows a two-phase design:

## 1. Buffering Left Input
- The operator eagerly buffers all left-side input batches into memory,
  util a memory limit is reached.
  Currently, an out-of-memory error will be thrown if all the left-side input batches
  cannot fit into memory at once.
  In the future, it's possible to make this case finish execution. (see
  'Memory-limited Execution' section)
- The rationale for buffering the left side is that scanning the right side
  can be expensive (e.g., decoding Parquet files), so buffering more left
  rows reduces the number of right-side scan passes required.

## 2. Probing Right Input
- Right-side input is streamed batch by batch.
- For each right-side batch:
  - It evaluates the join filter against the full buffered left input.
    This results in a Cartesian product between the right batch and each
    left row -- with the join predicate/filter applied -- for each inner
    loop iteration.
  - Matched results are accumulated into an output buffer. (see more in
    `Output Buffering Strategy` section)
- This process continues until all right-side input is consumed.

# Producing unmatched build-side data
- For special join types like left/full joins, it's required to also output
  unmatched pairs. During execution, bitmaps are kept for both left and right
  sides of the input; they'll be handled by dedicated states in `NLJStream`.
- The final output of the left side unmatched rows is handled by a single
  partition for simplicity, since it only counts a small portion of the
  execution time. (e.g. if probe side has 10k rows, the final output of
  unmatched build side only roughly counts for 1/10k of the total time)

# Output Buffering Strategy
The operator uses an intermediate output buffer to accumulate results. Once
the output threshold is reached (currently set to the same value as
`batch_size` in the configuration), the results will be eagerly output.

# Extra Notes
- The operator always considers the **left** side as the build (buffered) side.
  Therefore, the physical optimizer should assign the smaller input to the left.
- The design try to minimize the intermediate data size to approximately
  1 batch, for better cache locality and memory efficiency.

# Memory-limited Execution
When the memory budget is exceeded during left-side buffering, the operator
falls back to a multi-pass strategy:
1. Buffer as many left rows as fit in memory (one "chunk")
2. On the first pass, the right side is both processed and spilled to disk
3. For each subsequent left chunk, the right side is re-read from the spill file

The fallback is triggered automatically when the initial in-memory load
fails with `ResourcesExhausted` and disk spilling is available. Each
output partition independently re-executes the left child and manages
its own spill state.

All join types are supported. For RIGHT/FULL/RIGHT SEMI/RIGHT ANTI/
RIGHT MARK joins, a global right-side bitmap (indexed by right batch
sequence number) accumulates matches across all left chunks. After the
last left chunk is processed, the right side is replayed one more time
to emit unmatched right rows using the accumulated bitmap.

Tracking issue: <https://github.com/apache/datafusion/issues/15760>

# Clone / Shared State
Note this structure includes a [`OnceAsync`] that is used to coordinate the
loading of the left side with the processing in each output stream.
Therefore it can not be [`Clone`]

Unresolved upstream links (retained, not inferred): ``OnceAsync``, ``Clone``.

<a id="op-a3d8b872b07c55145773f88a"></a>
## apply_expressions

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21f4c1279f5d4b84d1bbc4c9"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:727`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3749dffbcfe26f81a4a6e35"></a>
## children

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:562`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb5f9f663a03ea6122c1df78"></a>
## contains_projection

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::contains_projection` · datafusion-physical-plan 55.1.0

```rust
fn contains_projection(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc659d124de7b065e46f89e3"></a>
## execute

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:634`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e510c387eb78c70a264c4842"></a>
## filter

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::filter` · datafusion-physical-plan 55.1.0

```rust
fn filter(&self) -> Option<&JoinFilter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Filters applied before join output

<a id="op-2b72223d2b1b1f087b8b564c"></a>
## fmt

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 10], "end": [193, 15], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/nested_loop_join.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-090a8f15dcc883ee2ea154f8"></a>
## fmt_as

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [536, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/joins/nested_loop_join.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65391d455d12259bc3a0334a"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34abaf3aa901012c7b422423"></a>
## join_type

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
fn join_type(&self) -> &JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

How the join is performed

<a id="op-3c56e8c56fbd7614c9bda848"></a>
## left

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
fn left(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

left side

<a id="op-630a064268b4e255bdb034e6"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5201e97ebd560dee4bfcd096"></a>
## metrics

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29f72fd0e8b37cb391233393"></a>
## name

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad7fbe0f2d4e3500e2111773"></a>
## projection

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::projection` · datafusion-physical-plan 55.1.0

```rust
fn projection(&self) -> &Option<ProjectionRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9b8c905511c2953fc80f2ae"></a>
## properties

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d2de9e3bdb48da84970ea02"></a>
## replace_children

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e63b65adb5d13d9b86a3449"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb3618f2852ed439f8b49391"></a>
## right

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
fn right(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

right side

<a id="op-0707807ecde9e53422494523"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:733`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2ebc970f174bf3406c62ae9"></a>
## swap_inputs

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::swap_inputs` · datafusion-physical-plan 55.1.0

```rust
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:454`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a new `ExecutionPlan` that runs NestedLoopsJoins with the left
and right inputs swapped.

# Notes:

This function should be called BEFORE inserting any repartitioning
operators on the join's children. Check [`super::HashJoinExec::swap_inputs`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-ab8ffc5ca4894926893b682e)
for more details.

<a id="op-680e6bbbff6007e1f3c36267"></a>
## try_from_proto

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 1], "end": [886, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:840`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1adfd82eed473d36c9277901"></a>
## try_new

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, filter: Option<JoinFilter>, join_type: &JoinType, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Try to create a new [`NestedLoopJoinExec`](../operations/datafusion_physical_plan.joins.nested_loop_join.NestedLoopJoinExec.md#op-879865d333b3afaee6f0226e)

<a id="op-fe4f49e76e29f3bd8322b8a3"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:765`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to push `projection` down through `nested_loop_join`. If possible, performs the
pushdown and returns a new [`NestedLoopJoinExec`](../operations/datafusion_physical_plan.joins.nested_loop_join.NestedLoopJoinExec.md#op-879865d333b3afaee6f0226e) as the top plan which has projections
as its children. Otherwise, returns `None`.

<a id="op-d1912e6d4920e81b354d3464"></a>
## try_to_proto

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce2250f3a31c6bc2613fb6e5"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0b1eceafed1c87acf9643a5"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [836, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/nested_loop_join.rs:624`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67d39d265579dd3297405d9a"></a>
## with_projection

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [888, 1], "end": [892, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::projection::EmbeddedProjection", "path": "EmbeddedProjection"}, "trait_path": "datafusion_physical_plan::projection::EmbeddedProjection"}`

Source: `src/joins/nested_loop_join.rs:889`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86fd2101a2e1310c7808666a"></a>
## with_projection

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [493, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
