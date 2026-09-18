# `datafusion_physical_plan::test::exec::StatisticsExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.StatisticsExec.json).

<a id="op-a672721408ead96091cdeafa"></a>
## StatisticsExec

`struct` · `datafusion_physical_plan::test::exec::StatisticsExec` · datafusion-physical-plan 55.1.0

```rust
struct StatisticsExec
```

Source: `src/test/exec.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A mock execution plan that simply returns the provided statistics

<a id="op-10672a747fc6f00498a66bae"></a>
## apply_expressions

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:722`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-503a6a09c310ca7a2695b05d"></a>
## children

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01d3f3fccde8d9d1f448e372"></a>
## clone

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> StatisticsExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 17], "end": [654, 22], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/test/exec.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7730aab16d1cb610c70e91ac"></a>
## execute

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, _partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:747`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ae8754c05e2d6d7c80a4b87"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 10], "end": [654, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57e24dc1fa28ca6e4cf33d6c"></a>
## fmt_as

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 1], "end": [707, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/test/exec.rs:687`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-767294392443d209ddf4bdac"></a>
## name

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:710`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a314ddebe14f03c14aa88ce"></a>
## new

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(stats: Statistics, schema: Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [660, 1], "end": [684, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af38ec65f31e27ea4b213c20"></a>
## properties

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89d5a8c160f70555c4f7250e"></a>
## replace_children

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:729`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faf261195eef4d4e65fbde1c"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:755`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56f630a3598bf18300fe48be"></a>
## with_new_children

`function` · `datafusion_physical_plan::test::exec::StatisticsExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::StatisticsExec", "path": "StatisticsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 1], "end": [766, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:737`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
