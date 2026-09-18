# `datafusion_physical_plan::test::exec::BlockingExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.BlockingExec.json).

<a id="op-78f8129482a10574e38600da"></a>
## BlockingExec

`struct` · `datafusion_physical_plan::test::exec::BlockingExec` · datafusion-physical-plan 55.1.0

```rust
struct BlockingExec
```

Source: `src/test/exec.rs:772`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execution plan that emits streams that block forever.

This is useful to test shutdown / cancellation behavior of certain execution plans.

<a id="op-fac177f46b7c1170b75379e3"></a>
## apply_expressions

`function` · `datafusion_physical_plan::test::exec::BlockingExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [879, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:852`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-191a67e6a29d686d1715e6c9"></a>
## children

`function` · `datafusion_physical_plan::test::exec::BlockingExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [879, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:839`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20cf81a886f976fe8f6f68a3"></a>
## execute

`function` · `datafusion_physical_plan::test::exec::BlockingExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, _partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [879, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:869`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e047935d20da550953b56d9"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::BlockingExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 10], "end": [771, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:771`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a126817323b51b267652272"></a>
## fmt_as

`function` · `datafusion_physical_plan::test::exec::BlockingExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 1], "end": [828, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/test/exec.rs:813`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e0aa232d3fe9d1197344629"></a>
## name

`function` · `datafusion_physical_plan::test::exec::BlockingExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [879, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:831`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d15a49fbb9f1b57d8a10fa2"></a>
## new

`function` · `datafusion_physical_plan::test::exec::BlockingExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef, n_partitions: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [781, 1], "end": [810, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:783`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create new [`BlockingExec`](../operations/datafusion_physical_plan.test.exec.BlockingExec.md#op-78f8129482a10574e38600da) with a give schema and number of partitions.

<a id="op-49d94207829459851dfd48e3"></a>
## properties

`function` · `datafusion_physical_plan::test::exec::BlockingExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [879, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:835`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ac9ca78a0e4dd366993327f"></a>
## refs

`function` · `datafusion_physical_plan::test::exec::BlockingExec::refs` · datafusion-physical-plan 55.1.0

```rust
fn refs(&self) -> Weak<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [781, 1], "end": [810, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:797`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Weak pointer that can be used for ref-counting this execution plan and its streams.

Use [`Weak::strong_count`] to determine if the plan itself and its streams are dropped (should be 0 in that
case). Note that tokio might take some time to cancel spawned tasks, so you need to wrap this check into a retry
loop. Use [`assert_strong_count_converges_to_zero`](../operations/datafusion_physical_plan.test.exec.assert_strong_count_converges_to_zero.md#op-cf35cbca5e4d1e38c0560486) to archive this.

Unresolved upstream links (retained, not inferred): ``Weak::strong_count``.

<a id="op-88958ed2fe40c65cef825f0a"></a>
## replace_children

`function` · `datafusion_physical_plan::test::exec::BlockingExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [879, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:844`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c2044f6afce838ef574862d"></a>
## with_new_children

`function` · `datafusion_physical_plan::test::exec::BlockingExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingExec", "path": "BlockingExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [879, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:859`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
