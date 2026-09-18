# `datafusion_physical_plan::sorts::partial_sort::PartialSortExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.partial_sort.PartialSortExec.json).

<a id="op-0ac508bb4f53820d3d249464"></a>
## PartialSortExec

`struct` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec` · datafusion-physical-plan 55.1.0

```rust
struct PartialSortExec
```

Source: `src/sorts/partial_sort.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort execution plan for inputs that are already partially sorted.

This operator takes input ordered by a prefix of the required ordering, and
produces output ordered by the required ordering, emitting rows sooner
(streaming) and using less peak memory than [`SortExec`] which must buffer
all rows before producing any output.

[`PartialSortExec`](../operations/datafusion_physical_plan.sorts.partial_sort.PartialSortExec.md#op-0ac508bb4f53820d3d249464) relies on the property that rows with the same sort
prefix are contiguous, so it can sort one prefix group at a time, emitting
completed groups without reading (and buffering) the entire input.

For example, if the required output is `(a, b, c)`, but the input is only
ordered by `(a, b)`, `PartialSortExec` sorts only within each `(a, b)`
group to produce output ordered by `(a, b, c)`.

```text
input ordered by a, b              output ordered by a, b, c

+---+---+---+                      +---+---+---+
| a | b | c |                      | a | b | c |
+---+---+---+                      +---+---+---+
| 0 | 0 | 3 |  --  new group  -->  | 0 | 0 | 1 |
| 0 | 0 | 2 |                      | 0 | 0 | 2 |
| 0 | 0 | 1 |                      | 0 | 0 | 3 |
| 0 | 1 | 1 |  --  new group  -->  | 0 | 1 | 1 |
| 0 | 2 | 4 |  --  new group  -->  | 0 | 2 | 0 |
| 0 | 2 | 0 |                      | 0 | 2 | 4 |
| 1 | 0 | 5 |  --  new group  -->  | 1 | 0 | 5 |
+---+---+---+                      +---+---+---+
```

# Buffering and Emitting Rows

[`PartialSortExec`](../operations/datafusion_physical_plan.sorts.partial_sort.PartialSortExec.md#op-0ac508bb4f53820d3d249464) buffers rows only until it can *prove* a prefix group
will never be seen again, then sorts and emits buffered rows. A group is
guaranteed to never be seen again once a row with a *different* prefix
value arrives. This relies on the input's existing ordering guarantees.

Using the example from above, rows accumulate in the in-memory buffer in
batches. As long as the `(a, b)` prefix keeps repeating, more rows are
buffered.

```text
           Buffer
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 0 | 0 | 3 |
       | 0 | 0 | 2 |
       | 0 | 0 | 1 |
       +---+---+---+
```

Once a batch arrives that contains a new `(a, b)` prefix, e.g. `(0, 2)`:
every buffered row for previous prefixes may be emitted:

```text
           Buffer
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 0 | 0 | 3 |
       | 0 | 0 | 2 |
       | 0 | 0 | 1 |
       | 0 | 1 | 1 |  <-- first row of new batch, new prefix
       | 0 | 2 | 4 |  <-- new prefix
       | 0 | 2 | 0 |
       | 1 | 0 | 5 |  <-- last row of new batch, new prefix
       +---+---+---+
```

Once known complete, the buffered rows are sorted by the full `(a, b, c)`
ordering and emitted as a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34); Any rows from the most recently
seen prefix remain buffered (as more rows with the same prefix may arrive in
future batches.

```text
         Emitted      <-- fully sorted on (a, b, c)
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 0 | 0 | 1 |   <-- completed group
       | 0 | 0 | 2 |
       | 0 | 0 | 3 |
       | 0 | 2 | 0 |   <-- completed group
       | 0 | 2 | 4 |
       | 0 | 1 | 1 |   <-- completed group
       +---+---+---+

           Buffer
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 1 | 0 | 5 |   <-- (possibly) in progress group
       +---+---+---+
```

[`SortExec`]: crate::sorts::sort::SortExec

<a id="op-9a8f88cff78da9da7ee710f3"></a>
## apply_expressions

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fa7823fccd3f007a73ae30b"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44b0d07d12e4c590eb8bcc22"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abf93d13df02f41bc66f40b6"></a>
## children

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56c1028cdc4947a2efa0ba4d"></a>
## clone

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PartialSortExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 17], "end": [179, 22], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sorts/partial_sort.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b0bcc3e2c489f34f4b18620"></a>
## common_prefix_length

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::common_prefix_length` · datafusion-physical-plan 55.1.0

```rust
fn common_prefix_length(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Common prefix length

<a id="op-6e27ee91e5746370cd04e40d"></a>
## execute

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5215af53eadb222ec26adb5c"></a>
## expr

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::expr` · datafusion-physical-plan 55.1.0

```rust
fn expr(&self) -> &LexOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort expressions

<a id="op-3096d95c610b30c386b1085b"></a>
## fetch

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If `Some(fetch)`, limits output to only the first "fetch" items

<a id="op-be3690cc268b76ebc694398d"></a>
## fetch

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-217fe1294fc5b24e8a1b044f"></a>
## fmt

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 10], "end": [179, 15], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sorts/partial_sort.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91c9dc32a63f3d0a325de79e"></a>
## fmt_as

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [343, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/sorts/partial_sort.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d59bf15de1cf85a57a501d97"></a>
## input

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input schema

<a id="op-e13326233985217cb4440dc1"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c59c3b0b29ee1ed55a8f92cb"></a>
## metrics

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d798d860a5196cfcbbcbbbb0"></a>
## name

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-106ab1971b4bc82afc9475ac"></a>
## new

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>, common_prefix_length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new partial sort execution plan

<a id="op-7ee62e6b1146512d22ad9a92"></a>
## preserve_partitioning

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::preserve_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn preserve_partitioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Whether this `PartialSortExec` preserves partitioning of the children

<a id="op-ead10c9d113e55a03fc75203"></a>
## properties

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a138a65ddeef4a7fd2c82252"></a>
## replace_children

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:388`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-991ab7c2e5ad677181a22252"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01ffee9ba7fc11d108f01527"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9318baba545e51b6ca91df8"></a>
## with_fetch

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Modify how many rows to include in the result

If None, then all rows will be returned, in sorted order.
If Some, then only the top `fetch` rows will be returned.
This can reduce the memory pressure required by the sort
operation since rows that are not going to be included
can be dropped.

<a id="op-1253a24182952f2edfc6dca4"></a>
## with_new_children

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3651d99c3a6d4430b89a483c"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [480, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/partial_sort.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8248e1d799ca1d2d0f87066"></a>
## with_preserve_partitioning

`function` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec::with_preserve_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_partitioning(self, preserve_partitioning: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partial_sort::PartialSortExec", "path": "PartialSortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [306, 2], "filename": "src/sorts/partial_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/partial_sort.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specify the partitioning behavior of this partial sort exec

If `preserve_partitioning` is true, sorts each partition
individually, producing one sorted stream for each input partition.

If `preserve_partitioning` is false, sorts and merges all
input partitions producing a single, sorted partition.
