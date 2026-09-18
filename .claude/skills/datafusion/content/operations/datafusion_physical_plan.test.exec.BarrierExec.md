# `datafusion_physical_plan::test::exec::BarrierExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.BarrierExec.json).

<a id="op-6f7ed19fb28b6b1888a5e986"></a>
## BarrierExec

`struct` · `datafusion_physical_plan::test::exec::BarrierExec` · datafusion-physical-plan 55.1.0

```rust
struct BarrierExec
```

Source: `src/test/exec.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A Mock ExecutionPlan that does not start producing input until a
barrier is called

<a id="op-67e7c22a6a746e932237bd44"></a>
## apply_expressions

`function` · `datafusion_physical_plan::test::exec::BarrierExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d71d533d36eb39520fda7e9"></a>
## children

`function` · `datafusion_physical_plan::test::exec::BarrierExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a0447eff4e035e1c883b8b2"></a>
## execute

`function` · `datafusion_physical_plan::test::exec::BarrierExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:490`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a stream which yields data

<a id="op-95f80452582cf6caa1f3862a"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::BarrierExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 10], "end": [325, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9575544fd6b7672414450d2"></a>
## fmt_as

`function` · `datafusion_physical_plan::test::exec::BarrierExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 1], "end": [449, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/test/exec.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a1471514d5b9fbc5256f6ad"></a>
## is_finish_barrier_reached

`function` · `datafusion_physical_plan::test::exec::BarrierExec::is_finish_barrier_reached` · datafusion-physical-plan 55.1.0

```rust
fn is_finish_barrier_reached(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [431, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return true if the finish barrier has been reached in all partitions

<a id="op-5538559e726daa3e99375439"></a>
## name

`function` · `datafusion_physical_plan::test::exec::BarrierExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da0c0e4c2338289119275214"></a>
## new

`function` · `datafusion_physical_plan::test::exec::BarrierExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(data: Vec<Vec<RecordBatch>>, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [431, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new exec with some number of partitions.

<a id="op-14c83cced988ce0cf5bdcb28"></a>
## properties

`function` · `datafusion_physical_plan::test::exec::BarrierExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13d3d2652e0e79f289b9dbeb"></a>
## replace_children

`function` · `datafusion_physical_plan::test::exec::BarrierExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e47d7b70d60dc18731ea4660"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::test::exec::BarrierExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c95ee910ba6213c3ad983403"></a>
## wait

`function` · `datafusion_physical_plan::test::exec::BarrierExec::wait` · datafusion-physical-plan 55.1.0

```rust
async fn wait(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [431, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

wait until all the input streams and this function is ready

<a id="op-17f693f7c4f1c55cfd936ed8"></a>
## wait_finish

`function` · `datafusion_physical_plan::test::exec::BarrierExec::wait_finish` · datafusion-physical-plan 55.1.0

```rust
async fn wait_finish(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [431, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b89bb6c5b5534bd749790e0"></a>
## with_finish_barrier

`function` · `datafusion_physical_plan::test::exec::BarrierExec::with_finish_barrier` · datafusion-physical-plan 55.1.0

```rust
fn with_finish_barrier(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [431, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbfbf68458b47f55133bee2d"></a>
## with_log

`function` · `datafusion_physical_plan::test::exec::BarrierExec::with_log` · datafusion-physical-plan 55.1.0

```rust
fn with_log(self, log: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [431, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf83418a40db374e0fecda18"></a>
## with_new_children

`function` · `datafusion_physical_plan::test::exec::BarrierExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [549, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:479`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a4003faf615611e72b06326"></a>
## without_start_barrier

`function` · `datafusion_physical_plan::test::exec::BarrierExec::without_start_barrier` · datafusion-physical-plan 55.1.0

```rust
fn without_start_barrier(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BarrierExec", "path": "BarrierExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 1], "end": [431, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
