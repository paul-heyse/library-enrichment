# `datafusion_physical_plan::repartition::RepartitionExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.repartition.RepartitionExec.json).

<a id="op-a0cd55fc5d1d094b433c884b"></a>
## RepartitionExec

`struct` · `datafusion_physical_plan::repartition::RepartitionExec` · datafusion-physical-plan 55.1.0

```rust
struct RepartitionExec
```

Source: `src/repartition/mod.rs:1409`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Maps `N` input partitions to `M` output partitions based on a
[`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) scheme.

# Background

DataFusion, like most other commercial systems, with the
notable exception of DuckDB, uses the "Exchange Operator" based
approach to parallelism which works well in practice given
sufficient care in implementation.

DataFusion's planner picks the target number of partitions and
then [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b) redistributes [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to that number
of output partitions.

For example, given `target_partitions=3` (trying to use 3 cores)
but scanning an input with 2 partitions, `RepartitionExec` can be
used to get 3 even streams of `RecordBatch`es


```text
       ▲                  ▲                  ▲
       │                  │                  │
       │                  │                  │
       │                  │                  │
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│    GroupBy    │  │    GroupBy    │  │    GroupBy    │
│   (Partial)   │  │   (Partial)   │  │   (Partial)   │
└───────────────┘  └───────────────┘  └───────────────┘
       ▲                  ▲                  ▲
       └──────────────────┼──────────────────┘
                          │
             ┌─────────────────────────┐
             │     RepartitionExec     │
             │   (hash/round robin)    │
             └─────────────────────────┘
                        ▲   ▲
            ┌───────────┘   └───────────┐
            │                           │
            │                           │
       .─────────.                 .─────────.
    ,─'           '─.           ,─'           '─.
   ;      Input      :         ;      Input      :
   :   Partition 0   ;         :   Partition 1   ;
    ╲               ╱           ╲               ╱
     '─.         ,─'             '─.         ,─'
        `───────'                   `───────'
```

# Error Handling

If any of the input partitions return an error, the error is propagated to
all output partitions and inputs are not polled again.

# Output Ordering

If more than one stream is being repartitioned, the output will be some
arbitrary interleaving (and thus unordered) unless
[`Self::with_preserve_order`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-61bea2b7fb9c9d0204f85945) specifies otherwise.

# Batch coalescing

Repartitioning one [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) implies creating multiple smaller batches, potentially
as many as the number of output partitions. [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b) makes sure that the returned
batches adhere to the configured `datafusion.execution.batch_size` for efficient operations,
and for that, it will automatically coalesce batches right after repartitioning for bounded
inputs. Coalescing is skipped for unbounded inputs so partial batches are emitted promptly.

For this, one shared [`LimitedBatchCoalescer`](../operations/datafusion_physical_plan.coalesce.LimitedBatchCoalescer.md#op-3f1aa30591797055625df08b) per output partition is used:

```text
                        ┌───┐                           ┌───┐
                     ┌─▶│   │────────▶.───────────.     │   │     ┌──────────────────┐
                     │  └───┘ ┌───┐  ( Coalescer 0 )──▶ ├───┤ ───▶│     Output 0     │
                     │┌──────▶│   │──▶`───────────'     │   │     └──────────────────┘
                     ││       └───┘                     └───┘
┌──────────────────┐ ││                                           ┌──────────────────┐
│BatchPartitioner 0│─┘│                                           │     Output 1     │
└──────────────────┘  │                                           └──────────────────┘
                      │
┌──────────────────┐  │                ...                        ┌──────────────────┐
│BatchPartitioner 1│──┘                                           │     Output 2     │
└──────────────────┘                                              └──────────────────┘

                                                                  ┌──────────────────┐
                                                                  │     Output 3     │
                                                                  └──────────────────┘
```

# Spilling Architecture

RepartitionExec uses [`SpillPool`](crate::spill::spill_pool) channels to handle
memory pressure during repartitioning. Each (input partition, output partition)
pair gets its own SpillPool channel for FIFO ordering.

```text
Input Partitions (N)          Output Partitions (M)
────────────────────          ─────────────────────

   Input 0 ──┐                      ┌──▶ Output 0
             │  ┌──────────────┐    │
             ├─▶│ SpillPool    │────┤
             │  │ [In0→Out0]   │    │
   Input 1 ──┤  └──────────────┘    ├──▶ Output 1
             │                       │
             │  ┌──────────────┐    │
             ├─▶│ SpillPool    │────┤
             │  │ [In1→Out0]   │    │
   Input 2 ──┤  └──────────────┘    ├──▶ Output 2
             │                      │
             │       ... (N×M SpillPools total)
             │                      │
             │  ┌──────────────┐    │
             └─▶│ SpillPool    │────┘
                │ [InN→OutM]   │
                └──────────────┘

Each SpillPool maintains FIFO order for its (input, output) pair.
See `RepartitionBatch` for details on the memory/spill decision logic.
```

# Footnote

The "Exchange Operator" was first described in the 1989 paper
[Encapsulation of parallelism in the Volcano query processing
system Paper](https://dl.acm.org/doi/pdf/10.1145/93605.98720)
which uses the term "Exchange" for the concept of repartitioning
data across threads.

For more background, please also see the [Optimizing Repartitions in DataFusion] blog.

[Optimizing Repartitions in DataFusion]: https://datafusion.apache.org/blog/2025/12/15/avoid-consecutive-repartitions

<a id="op-d7433fb40c1c2195e43d0587"></a>
## apply_expressions

`function` · `datafusion_physical_plan::repartition::RepartitionExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1553`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eef06bbb19f02a42786447e3"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::repartition::RepartitionExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1613`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6c02472d70a774481102a43"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::repartition::RepartitionExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1821`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7bb5244325e6c5cede8c2e4"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::repartition::RepartitionExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1774`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec3746bf316229e270418ed7"></a>
## children

`function` · `datafusion_physical_plan::repartition::RepartitionExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1549`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee26075c010b8bd34691859d"></a>
## clone

`function` · `datafusion_physical_plan::repartition::RepartitionExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> RepartitionExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 17], "end": [1408, 22], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/repartition/mod.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fe83b3d84b9137ca72f852e"></a>
## execute

`function` · `datafusion_physical_plan::repartition::RepartitionExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1621`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0be9c4ceec65cfa0b8c3e28e"></a>
## fmt

`function` · `datafusion_physical_plan::repartition::RepartitionExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 10], "end": [1408, 15], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/repartition/mod.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b53ce704891f3cd1897b5014"></a>
## fmt_as

`function` · `datafusion_physical_plan::repartition::RepartitionExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1492, 1], "end": [1537, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/repartition/mod.rs:1493`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec76d324388d789ecdc7dd84"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::repartition::RepartitionExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1889`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fd9114d3ec184ca0186cd4a"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::repartition::RepartitionExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1898`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-def2abb8ade4b8e079bcca48"></a>
## input

`function` · `datafusion_physical_plan::repartition::RepartitionExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1469, 1], "end": [1490, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input execution plan

<a id="op-a247e792c5f67c41fa81db08"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::repartition::RepartitionExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1617`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae8fa2976bee39e12097a94d"></a>
## metrics

`function` · `datafusion_physical_plan::repartition::RepartitionExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1770`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-981631057f9124902410b04e"></a>
## name

`function` · `datafusion_physical_plan::repartition::RepartitionExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1469, 1], "end": [1490, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1487`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get name used to display this Exec

<a id="op-ef20abf7d177ae12c9be473c"></a>
## name

`function` · `datafusion_physical_plan::repartition::RepartitionExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1540`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dabe19fd3aaf6b6525cf5623"></a>
## partitioning

`function` · `datafusion_physical_plan::repartition::RepartitionExec::partitioning` · datafusion-physical-plan 55.1.0

```rust
fn partitioning(&self) -> &Partitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1469, 1], "end": [1490, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1476`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Partitioning scheme to use

<a id="op-b666e64a3515b59980d02180"></a>
## preserve_order

`function` · `datafusion_physical_plan::repartition::RepartitionExec::preserve_order` · datafusion-physical-plan 55.1.0

```rust
fn preserve_order(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1469, 1], "end": [1490, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1482`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get preserve_order flag of the RepartitionExec
`true` means `SortPreservingRepartitionExec`, `false` means `RepartitionExec`

<a id="op-d17a2f4c2c75d1e1f93e7e7f"></a>
## properties

`function` · `datafusion_physical_plan::repartition::RepartitionExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1545`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-bf5661434c837c3624fccebd"></a>
## repartitioned

`function` · `datafusion_physical_plan::repartition::RepartitionExec::repartitioned` · datafusion-physical-plan 55.1.0

```rust
fn repartitioned(&self, target_partitions: usize, _config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1928`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4be577c91c18472ea23206c"></a>
## replace_children

`function` · `datafusion_physical_plan::repartition::RepartitionExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1567`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36dcbe911c548ddff5123337"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::repartition::RepartitionExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1778`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6976dbd2ef3f28d890e51625"></a>
## try_from_proto

`function` · `datafusion_physical_plan::repartition::RepartitionExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1977, 1], "end": [2018, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1979`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b) from its protobuf representation.

<a id="op-a74923e48a954566659b370f"></a>
## try_new

`function` · `datafusion_physical_plan::repartition::RepartitionExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(input: Arc<dyn ExecutionPlan>, partitioning: Partitioning) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 1], "end": [2247, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:2024`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new RepartitionExec, that produces output `partitioning`, and
does not preserve the order of the input (see [`Self::with_preserve_order`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-61bea2b7fb9c9d0204f85945)
for more details)

<a id="op-aef63aa26fac6e7d532ac82d"></a>
## try_pushdown_sort

`function` · `datafusion_physical_plan::repartition::RepartitionExec::try_pushdown_sort` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1907`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a9a6d5a2fdf5a635126a57f"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::repartition::RepartitionExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1825`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-115670c35235800fe403fb1a"></a>
## try_to_proto

`function` · `datafusion_physical_plan::repartition::RepartitionExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1954`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ddae2235c852fe456ae09ba"></a>
## with_new_children

`function` · `datafusion_physical_plan::repartition::RepartitionExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1593`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4b16faac26517006b867c5a"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::repartition::RepartitionExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1539, 1], "end": [1974, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/repartition/mod.rs:1603`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61bea2b7fb9c9d0204f85945"></a>
## with_preserve_order

`function` · `datafusion_physical_plan::repartition::RepartitionExec::with_preserve_order` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_order(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RepartitionExec", "path": "RepartitionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2020, 1], "end": [2247, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:2088`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specify if this repartitioning operation should preserve the order of
rows from its input when producing output. Preserving order is more
expensive at runtime, so should only be set if the output of this
operator can take advantage of it.

If the input is not ordered, or has only one partition, this is a no op,
and the node remains a `RepartitionExec`.
