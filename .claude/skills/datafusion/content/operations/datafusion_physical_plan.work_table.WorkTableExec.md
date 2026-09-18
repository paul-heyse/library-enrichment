# `datafusion_physical_plan::work_table::WorkTableExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.work_table.WorkTableExec.json).

<a id="op-dc914058da7ef84c1e4b54ee"></a>
## WorkTableExec

`struct` · `datafusion_physical_plan::work_table::WorkTableExec` · datafusion-physical-plan 55.1.0

```rust
struct WorkTableExec
```

Source: `src/work_table.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A temporary "working table" operation where the input data will be
taken from the named handle during the execution and will be re-published
as is (kind of like a mirror).

Most notably used in the implementation of recursive queries where the
underlying relation does not exist yet but the data will come as the previous
term is evaluated. This table will be used such that the recursive plan
will register a receiver in the task context and this plan will use that
receiver to get the data and stream it back up so that the batches are available
in the next iteration.

<a id="op-eec2ee12f91a9540b4c762d2"></a>
## apply_expressions

`function` · `datafusion_physical_plan::work_table::WorkTableExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-818f5682a0a075407bc9c6fe"></a>
## children

`function` · `datafusion_physical_plan::work_table::WorkTableExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38c7db96e8d917a620d43972"></a>
## clone

`function` · `datafusion_physical_plan::work_table::WorkTableExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> WorkTableExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 10], "end": [101, 15], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/work_table.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ab38f60df20174c67d0d72b"></a>
## execute

`function` · `datafusion_physical_plan::work_table::WorkTableExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Stream the batches that were written to the work table.

<a id="op-8959bd93c557046ff31d95c7"></a>
## fmt

`function` · `datafusion_physical_plan::work_table::WorkTableExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 17], "end": [101, 22], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/work_table.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d94a1232bc4b9c18dcf50e3"></a>
## fmt_as

`function` · `datafusion_physical_plan::work_table::WorkTableExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [175, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/work_table.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f43c4425d1de02fd31d59586"></a>
## metrics

`function` · `datafusion_physical_plan::work_table::WorkTableExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43acb5a3ee764452d8228e2c"></a>
## name

`function` · `datafusion_physical_plan::work_table::WorkTableExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [158, 2], "filename": "src/work_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/work_table.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ref to name

<a id="op-74b7c9839a8b93d7cba5cea6"></a>
## name

`function` · `datafusion_physical_plan::work_table::WorkTableExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24629ce731ba806d7d013879"></a>
## new

`function` · `datafusion_physical_plan::work_table::WorkTableExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(name: String, schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [158, 2], "filename": "src/work_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/work_table.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new execution plan for a worktable exec.

<a id="op-dfc373bb645bd9effd80be02"></a>
## properties

`function` · `datafusion_physical_plan::work_table::WorkTableExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b48acf8e8573d8355b6fb65d"></a>
## replace_children

`function` · `datafusion_physical_plan::work_table::WorkTableExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b22ab29d7402c2dfee3e666"></a>
## schema

`function` · `datafusion_physical_plan::work_table::WorkTableExec::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [158, 2], "filename": "src/work_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/work_table.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Arc clone of ref to schema

<a id="op-0db0f4a146638ceaa1712abe"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::work_table::WorkTableExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e44127479c064a69bc9221d"></a>
## with_new_children

`function` · `datafusion_physical_plan::work_table::WorkTableExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a777c48afec81105985d58a7"></a>
## with_new_state

`function` · `datafusion_physical_plan::work_table::WorkTableExec::with_new_state` · datafusion-physical-plan 55.1.0

```rust
fn with_new_state(&self, state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTableExec", "path": "WorkTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [285, 2], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/work_table.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Injects run-time state into this `WorkTableExec`.

The only state this node currently understands is an [`Arc<WorkTable>`].
If `state` can be down-cast to that type, a new `WorkTableExec` backed
by the provided work table is returned.  Otherwise `None` is returned
so that callers can attempt to propagate the state further down the
execution plan tree.

Unresolved upstream links (retained, not inferred): ``Arc<WorkTable>``.
