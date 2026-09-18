# `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.json).

<a id="op-e0ba0adb06eaa3e5277df91c"></a>
## SortPreservingMergeExec

`struct` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec` · datafusion-physical-plan 55.1.0

```rust
struct SortPreservingMergeExec
```

Source: `src/sorts/sort_preserving_merge.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort preserving merge execution plan

# Overview

This operator implements a K-way merge. It is used to merge multiple sorted
streams into a single sorted stream and is highly optimized.

## Inputs:

1. A list of sort expressions
2. An input plan, where each partition is sorted with respect to
   these sort expressions.

## Output:

1. A single partition that is also sorted with respect to the expressions

## Diagram

```text
┌─────────────────────────┐
│ ┌───┬───┬───┬───┐       │
│ │ A │ B │ C │ D │ ...   │──┐
│ └───┴───┴───┴───┘       │  │
└─────────────────────────┘  │  ┌───────────────────┐    ┌───────────────────────────────┐
  Stream 1                   │  │                   │    │ ┌───┬───╦═══╦───┬───╦═══╗     │
                             ├─▶│SortPreservingMerge│───▶│ │ A │ B ║ B ║ C │ D ║ E ║ ... │
                             │  │                   │    │ └───┴─▲─╩═══╩───┴───╩═══╝     │
┌─────────────────────────┐  │  └───────────────────┘    └─┬─────┴───────────────────────┘
│ ╔═══╦═══╗               │  │
│ ║ B ║ E ║     ...       │──┘                             │
│ ╚═══╩═══╝               │              Stable sort if `enable_round_robin_repartition=false`:
└─────────────────────────┘              the merged stream places equal rows from stream 1
  Stream 2


 Input Partitions                                          Output Partition
   (sorted)                                                  (sorted)
```

# Error Handling

If any of the input partitions return an error, the error is propagated to
the output and inputs are not polled again.

<a id="op-11471511323bfb969e439715"></a>
## apply_expressions

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3f6178cfcdd627b47c067bc"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ef2adc3b80c52c1d9ee00c6"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1805d80db006afdb81b69214"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48a3fc4d679a7bd64c06cabb"></a>
## children

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b6a4636c537d6e6af603531"></a>
## clone

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> SortPreservingMergeExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 17], "end": [90, 22], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sorts/sort_preserving_merge.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f3db90b06134de5465d55f8"></a>
## execute

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c75a59f3edd792b1e4e5a40a"></a>
## expr

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::expr` · datafusion-physical-plan 55.1.0

```rust
fn expr(&self) -> &LexOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [188, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort_preserving_merge.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort expressions

<a id="op-0e9fe7ae81cf8d13695f724f"></a>
## fetch

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [188, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort_preserving_merge.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Fetch

<a id="op-51fd6a805d7ed21bbcf92ac6"></a>
## fetch

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7f8a85bb29cde64bb837d63"></a>
## fmt

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 10], "end": [90, 15], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sorts/sort_preserving_merge.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3b427eb2148c920e0876ed4"></a>
## fmt_as

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [221, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/sorts/sort_preserving_merge.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-770648de39ba2fd59632478d"></a>
## input

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [188, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort_preserving_merge.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input schema

<a id="op-e001dd56076c37c627f012aa"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7adde57595012727bdffb8bf"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7baaf3473488284eeb0121d6"></a>
## metrics

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-783a2167a4452f8bf05933f0"></a>
## name

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ab56f6bbea285182b585b53"></a>
## new

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [188, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort_preserving_merge.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new sort execution plan

<a id="op-ffa0627d80a00dea1aad328e"></a>
## properties

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-0c354d695f5b7278a4497551"></a>
## replace_children

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e5d64e0ee2f2b71c5e332f8"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c0817b15038654181181438"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de8b781cf0359cec1aa0acce"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba73e08b68db025b270c641f"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4d5e2c1a5100dd79835c9e5"></a>
## try_from_proto

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 1], "end": [562, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort_preserving_merge.rs:509`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12072fadd66d6bf4d03d5b01"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to swap the projection with its input [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c).
If this is possible, it returns the new [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c) whose
child is a projection. Otherwise, it returns None.

<a id="op-2af47a49c5fc1397f5c3dfb2"></a>
## try_to_proto

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c524175995b020727dcb846"></a>
## with_fetch

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sets the number of rows to fetch

<a id="op-b82195c2ccd065eef3006630"></a>
## with_fetch

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [188, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort_preserving_merge.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sets the number of rows to fetch

<a id="op-5cdcf45c01af2c82fc5e1163"></a>
## with_new_children

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e5ab2c7ba3a01715796a4b1"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd349ae89c320f43bed7427b"></a>
## with_preserve_order

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::with_preserve_order` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [505, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort_preserving_merge.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec8ceabcfd6fdccdcbc73977"></a>
## with_round_robin_repartition

`function` · `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec::with_round_robin_repartition` · datafusion-physical-plan 55.1.0

```rust
fn with_round_robin_repartition(self, enable_round_robin_repartition: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec", "path": "SortPreservingMergeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [188, 2], "filename": "src/sorts/sort_preserving_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort_preserving_merge.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sets the selection strategy of tied winners of the loser tree algorithm

If true (the default) equal output rows are placed in the merged stream
in round robin fashion. This approach consumes input streams at more
even rates when there are many rows with the same sort key.

If false, equal output rows are always placed in the merged stream in
the order of the inputs, resulting in potentially slower execution but a
stable output order.
