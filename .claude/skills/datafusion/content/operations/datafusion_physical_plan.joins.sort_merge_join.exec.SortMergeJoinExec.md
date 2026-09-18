# `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.json).

<a id="op-f1df017b4f9f3f9c2ca109fe"></a>
## SortMergeJoinExec

`struct` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec` · datafusion-physical-plan 55.1.0

```rust
struct SortMergeJoinExec
```

Source: `src/joins/sort_merge_join/exec.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Join execution plan that executes equi-join predicates on multiple partitions using Sort-Merge
join algorithm and applies an optional filter post join. Can be used to join arbitrarily large
inputs where one or both of the inputs don't fit in the available memory.

# Join Expressions

Equi-join predicate (e.g. `<col1> = <col2>`) expressions are represented by [`Self::on`](../operations/datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.md#op-70bf0868fe6379f68a846696).

Non-equality predicates, which can not be pushed down to join inputs (e.g.
`<col1> != <col2>`) are known as "filter expressions" and are evaluated
after the equijoin predicates. They are represented by [`Self::filter`](../operations/datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.md#op-c14452ac6a39382a5de532da). These are optional
expressions.

# Sorting

Assumes that both the left and right input to the join are pre-sorted. It is not the
responsibility of this execution plan to sort the inputs.

# "Streamed" vs "Buffered"

The number of record batches of streamed input currently present in the memory will depend
on the output batch size of the execution plan. There is no spilling support for streamed input.
The comparisons are performed from values of join keys in streamed input with the values of
join keys in buffered input. One row in streamed record batch could be matched with multiple rows in
buffered input batches. Streamed input batches are represented by `StreamedBatch`.

Buffered input is buffered for all record batches having the same value of join key.
If the memory limit increases beyond the specified value and spilling is enabled,
buffered batches could be spilled to disk. If spilling is disabled, the execution
will fail under the same conditions. Multiple record batches of buffered could currently reside
in memory/disk during the execution. The number of buffered batches residing in
memory/disk depends on the number of rows of buffered input having the same value
of join key as that of streamed input rows currently present in memory. Due to pre-sorted inputs,
the algorithm understands when it is not needed anymore, and releases the buffered batches
from memory/disk. Buffered input batches are represented by `BufferedBatch`.

Depending on the type of join, left or right input may be selected as streamed or buffered
respectively. For example, in a left-outer join, the left execution plan will be selected as
streamed input while in a right-outer join, the right execution plan will be selected as the
streamed input.

Reference for the algorithm:
<https://en.wikipedia.org/wiki/Sort-merge_join>.

Helpful short video demonstration:
<https://www.youtube.com/watch?v=jiWCPJtDE2c>.

<a id="op-781b58620b8bc4e4a8838e23"></a>
## apply_expressions

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4e4829e123e696bf375b1ff"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:599`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7044501f1f8afe599276430b"></a>
## children

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6833c1f2d20e8630f57e2e2"></a>
## clone

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> SortMergeJoinExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 17], "end": [107, 22], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/joins/sort_merge_join/exec.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddce6ace15467e7daa635d3b"></a>
## execute

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:504`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c14452ac6a39382a5de532da"></a>
## filter

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::filter` · datafusion-physical-plan 55.1.0

```rust
fn filter(&self) -> &Option<JoinFilter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to join filter

<a id="op-e089540d9d5e65a3c06a60fb"></a>
## filter

`struct_field` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::filter` · datafusion-physical-plan 55.1.0

```rust
filter: Option<joins::utils::JoinFilter>
```

Source: `src/joins/sort_merge_join/exec.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Filters which are applied while finding matching rows

<a id="op-ce5d46846a14f48008050b7c"></a>
## fmt

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 10], "end": [107, 15], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/sort_merge_join/exec.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84b3f757f30e97eae67e5a34"></a>
## fmt_as

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 1], "end": [401, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/joins/sort_merge_join/exec.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17829e6e42a06f7be39916f5"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4949508220affb169595d722"></a>
## join_type

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
fn join_type(&self) -> JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Join type

<a id="op-c28b47771da3bd2061f63b61"></a>
## join_type

`struct_field` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
join_type: datafusion_common::JoinType
```

Source: `src/joins/sort_merge_join/exec.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

How the join is performed

<a id="op-8a795622ed64cbf1c4f71a9a"></a>
## left

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
fn left(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to left execution plan

<a id="op-e1b150c589881e45d8174780"></a>
## left

`struct_field` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
left: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/sort_merge_join/exec.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Left sorted joining execution plan

<a id="op-c7153fada428197fb21692d9"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34956ca50ac0c0d9aabb4bb6"></a>
## metrics

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-736057f0a096f2e0ffeb32ea"></a>
## name

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69c38d97def3800639035566"></a>
## null_equality

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::null_equality` · datafusion-physical-plan 55.1.0

```rust
fn null_equality(&self) -> NullEquality
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Null equality

<a id="op-af1bbabb031e7a50edaac059"></a>
## null_equality

`struct_field` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::null_equality` · datafusion-physical-plan 55.1.0

```rust
null_equality: datafusion_common::NullEquality
```

Source: `src/joins/sort_merge_join/exec.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Defines the null equality for the join.

<a id="op-4e34ec2de3f1f35527ef1825"></a>
## on

`struct_field` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::on` · datafusion-physical-plan 55.1.0

```rust
on: joins::utils::JoinOn
```

Source: `src/joins/sort_merge_join/exec.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set of common columns used to join on

<a id="op-70bf0868fe6379f68a846696"></a>
## on

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::on` · datafusion-physical-plan 55.1.0

```rust
fn on(&self) -> &[(PhysicalExprRef, PhysicalExprRef)]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set of common columns used to join on

<a id="op-9111721267205a59a60b808c"></a>
## probe_side

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::probe_side` · datafusion-physical-plan 55.1.0

```rust
fn probe_side(join_type: &JoinType) -> JoinSide
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get probe side (e.g streaming side) information for this sort merge join.
In current implementation, probe side is determined according to join type.

<a id="op-ab018a6d85a187a630c3d79b"></a>
## properties

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:408`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f778b566f4b7154501b9c3a7"></a>
## replace_children

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7558d31570fdfce6e2cd1332"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd2adca8df6caa3281682570"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a40f1b651af16fa98c0ef39"></a>
## right

`struct_field` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
right: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/sort_merge_join/exec.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Right sorting joining execution plan

<a id="op-3130c2639a4bda7d2ad056e9"></a>
## right

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
fn right(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to right execution plan

<a id="op-00aa2a9103561e60a9a9881d"></a>
## sort_options

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::sort_options` · datafusion-physical-plan 55.1.0

```rust
fn sort_options(&self) -> &[SortOptions]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to sort options

<a id="op-4d8a272dd993d8fd756d163e"></a>
## sort_options

`struct_field` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::sort_options` · datafusion-physical-plan 55.1.0

```rust
sort_options: Vec<arrow::compute::SortOptions>
```

Source: `src/joins/sort_merge_join/exec.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort options of join columns used in sorting left and right execution plans

<a id="op-ea1b3f134ea15966327b1dc1"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2f590f267ab88e8914e97b5"></a>
## swap_inputs

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::swap_inputs` · datafusion-physical-plan 55.1.0

```rust
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

# Notes:

This function should be called BEFORE inserting any repartitioning
operators on the join's children. Check [`super::super::HashJoinExec::swap_inputs`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-ab8ffc5ca4894926893b682e)
for more details.

<a id="op-49c96081c265cc8c924c265f"></a>
## try_from_proto

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [739, 1], "end": [826, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:745`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`SortMergeJoinExec`](../operations/datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.md#op-f1df017b4f9f3f9c2ca109fe) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`].

[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto

<a id="op-4211b73129ef64e78a8b298b"></a>
## try_new

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: JoinOn, filter: Option<JoinFilter>, join_type: JoinType, sort_options: Vec<SortOptions>, null_equality: NullEquality) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [347, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/sort_merge_join/exec.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to create a new [SortMergeJoinExec](../operations/datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.md#op-f1df017b4f9f3f9c2ca109fe).
The inputs are sorted using `sort_options` are applied to the columns in the `on`
# Error
This function errors when it is not possible to join the left and right sides on keys `on`.

<a id="op-ed68c131e2c9fbb12b86e90c"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:630`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to swap the projection with its input [`SortMergeJoinExec`](../operations/datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.md#op-f1df017b4f9f3f9c2ca109fe). If it can be done,
it returns the new swapped version having the [`SortMergeJoinExec`](../operations/datafusion_physical_plan.joins.sort_merge_join.exec.SortMergeJoinExec.md#op-f1df017b4f9f3f9c2ca109fe) as the top plan.
Otherwise, it returns None.

<a id="op-3dbe60fb3706568248188888"></a>
## try_to_proto

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:683`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb5c8eee32ee4023ce6a1086"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-434d2b667e511ecf97d8e90b"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec", "path": "SortMergeJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [736, 2], "filename": "src/joins/sort_merge_join/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/sort_merge_join/exec.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
