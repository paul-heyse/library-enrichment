# `datafusion_physical_plan::test::exec::MockExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.MockExec.json).

<a id="op-c860d8ecad1634cffe7d2e7e"></a>
## MockExec

`struct` · `datafusion_physical_plan::test::exec::MockExec` · datafusion-physical-plan 55.1.0

```rust
struct MockExec
```

Source: `src/test/exec.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A Mock ExecutionPlan that can be used for writing tests of other
ExecutionPlans

<a id="op-e347e684a87e4d19c9163fc9"></a>
## apply_expressions

`function` · `datafusion_physical_plan::test::exec::MockExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07c6a1a8db4fd917119e326a"></a>
## children

`function` · `datafusion_physical_plan::test::exec::MockExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc639ceef7ff028ff4d71753"></a>
## execute

`function` · `datafusion_physical_plan::test::exec::MockExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a stream which yields data

<a id="op-916e2446442751a959936ad7"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::MockExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6817ce1ffdf13e4f6d7beeea"></a>
## fmt_as

`function` · `datafusion_physical_plan::test::exec::MockExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [184, 1], "end": [200, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/test/exec.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-befb767fd24965769323c91d"></a>
## name

`function` · `datafusion_physical_plan::test::exec::MockExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-170ad95bd1c488ee306fb7ed"></a>
## new

`function` · `datafusion_physical_plan::test::exec::MockExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(data: Vec<Result<RecordBatch>>, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [182, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new `MockExec` with a single partition that returns
the specified `Results`s.

By default, the batches are not produced immediately (the
caller has to actually yield and another task must run) to
ensure any poll loops are correct. This behavior can be
changed with `with_use_task`

<a id="op-53c23d2d632cf70ef890b35a"></a>
## properties

`function` · `datafusion_physical_plan::test::exec::MockExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1babe6f6f35e0f633a765616"></a>
## replace_children

`function` · `datafusion_physical_plan::test::exec::MockExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3327abdc8384051d13131fd6"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::test::exec::MockExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7854e794cbd85991a8bc84f"></a>
## with_new_children

`function` · `datafusion_physical_plan::test::exec::MockExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [313, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f6224d576e4950f957370f"></a>
## with_unknown_statistics

`function` · `datafusion_physical_plan::test::exec::MockExec::with_unknown_statistics` · datafusion-physical-plan 55.1.0

```rust
fn with_unknown_statistics(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [182, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Report unknown statistics rather than computing them from `data`.

By default statistics are derived from `data`, which propagates any
planted errors when statistics are requested during planning (for
example when a parent node computes its properties). Use this when a
planted error should only surface at execution time.

<a id="op-157d57a2c33835e254bd0155"></a>
## with_use_task

`function` · `datafusion_physical_plan::test::exec::MockExec::with_use_task` · datafusion-physical-plan 55.1.0

```rust
fn with_use_task(self, use_task: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::MockExec", "path": "MockExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [182, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If `use_task` is true (the default) then the batches are sent
back using a separate task to ensure the underlying stream is
not immediately ready
