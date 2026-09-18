# `datafusion_physical_plan::test::exec::PanicExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.PanicExec.json).

<a id="op-4f64d549270a851ec840bf94"></a>
## PanicExec

`struct` · `datafusion_physical_plan::test::exec::PanicExec` · datafusion-physical-plan 55.1.0

```rust
struct PanicExec
```

Source: `src/test/exec.rs:928`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execution plan that emits streams that panics.

This is useful to test panic handling of certain execution plans.

<a id="op-e0b9b2cb88ee8b62efe8f0b5"></a>
## apply_expressions

`function` · `datafusion_physical_plan::test::exec::PanicExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1041, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:1004`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce3e2b4a3f8a8e99d1bac45d"></a>
## children

`function` · `datafusion_physical_plan::test::exec::PanicExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1041, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:999`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ea8d9a2e50ae3edbd2f3b3f"></a>
## execute

`function` · `datafusion_physical_plan::test::exec::PanicExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1041, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:1029`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fdbcf90d9d3b43a7c2fca7d"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::PanicExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [927, 10], "end": [927, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:927`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fec677ed41e37c321cc5eaa"></a>
## fmt_as

`function` · `datafusion_physical_plan::test::exec::PanicExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [972, 1], "end": [988, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/test/exec.rs:973`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-884aab43d8fe013db9a7aed8"></a>
## name

`function` · `datafusion_physical_plan::test::exec::PanicExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1041, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:991`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b02eab55421e0d3ca57ef77d"></a>
## new

`function` · `datafusion_physical_plan::test::exec::PanicExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef, n_partitions: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [970, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:941`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create new [`PanicExec`](../operations/datafusion_physical_plan.test.exec.PanicExec.md#op-4f64d549270a851ec840bf94) with a give schema and number of
partitions, which will each panic immediately.

<a id="op-120b648b9db4b8e3147fb87a"></a>
## properties

`function` · `datafusion_physical_plan::test::exec::PanicExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1041, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:995`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39d8119d8f469b21b4035e5b"></a>
## replace_children

`function` · `datafusion_physical_plan::test::exec::PanicExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1041, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:1011`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96b8aa0aa54b9ab7a46f2faa"></a>
## with_new_children

`function` · `datafusion_physical_plan::test::exec::PanicExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1041, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:1019`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f91f75b547f4654bd2b430e2"></a>
## with_partition_panic

`function` · `datafusion_physical_plan::test::exec::PanicExec::with_partition_panic` · datafusion-physical-plan 55.1.0

```rust
fn with_partition_panic(self, partition: usize, count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::PanicExec", "path": "PanicExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [970, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:952`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the number of batches prior to panic for a partition
