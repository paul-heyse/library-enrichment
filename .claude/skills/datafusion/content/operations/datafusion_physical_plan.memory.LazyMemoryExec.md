# `datafusion_physical_plan::memory::LazyMemoryExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.memory.LazyMemoryExec.json).

<a id="op-dd53c4e97a6defd2b8a53923"></a>
## LazyMemoryExec

`struct` · `datafusion_physical_plan::memory::LazyMemoryExec` · datafusion-physical-plan 55.1.0

```rust
struct LazyMemoryExec
```

Source: `src/memory.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execution plan for lazy in-memory batches of data

This plan generates output batches lazily, it doesn't have to buffer all batches
in memory up front (compared to `MemorySourceConfig`), thus consuming constant memory.

<a id="op-35b23f4adbd665b60ce3f885"></a>
## add_ordering

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::add_ordering` · datafusion-physical-plan 55.1.0

```rust
fn add_ordering(&mut self, ordering: impl IntoIterator<Item = PhysicalSortExpr>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [266, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97e1a5e1e53d2e2de2a4893f"></a>
## apply_expressions

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-013ef7f2e803bde3a9596cd1"></a>
## children

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d79b0df94c4480f54be1c73"></a>
## execute

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffc43185b39a4013a22a9dcc"></a>
## fmt

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [275, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd43c677be232018434a7805"></a>
## fmt_as

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [307, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/memory.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fff12dd5da616b06886a0e2"></a>
## generators

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::generators` · datafusion-physical-plan 55.1.0

```rust
fn generators(&self) -> &Vec<Arc<RwLock<dyn LazyBatchGenerator>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [266, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the batch generators

<a id="op-e1c9fa18bc518afe8e4c7c99"></a>
## metrics

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a879107af78bf2b8bd353222"></a>
## name

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67473a1e1b3242594663e2c6"></a>
## properties

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93f8920b8cf9b9775ff94479"></a>
## replace_children

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-379d1f801b27cd4488b6cc5f"></a>
## reset_state

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb726e8e612d2836efca2460"></a>
## schema

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1969a00060201937b733b5c9"></a>
## try_new

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(schema: SchemaRef, generators: Vec<Arc<RwLock<dyn LazyBatchGenerator>>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [266, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new lazy memory execution plan

<a id="op-f5f38afbc48b56cd9c65211a"></a>
## try_set_partitioning

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::try_set_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn try_set_partitioning(&mut self, partitioning: Partitioning) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [266, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d1e2cc267bae33e023f2d40"></a>
## with_new_children

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [400, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/memory.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51603014f4425c18c10cd82e"></a>
## with_projection

`function` · `datafusion_physical_plan::memory::LazyMemoryExec::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryExec", "path": "LazyMemoryExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [266, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
