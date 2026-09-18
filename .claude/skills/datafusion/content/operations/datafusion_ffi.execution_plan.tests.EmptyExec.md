# `datafusion_ffi::execution_plan::tests::EmptyExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.execution_plan.tests.EmptyExec.json).

<a id="op-3d93b6ca9d23330caf4a0328"></a>
## EmptyExec

`struct` · `datafusion_ffi::execution_plan::tests::EmptyExec` · datafusion-ffi 55.1.0

```rust
struct EmptyExec
```

Source: `src/execution_plan.rs:567`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dad18024074dbe84979822a"></a>
## apply_expressions

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::apply_expressions` · datafusion-ffi 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11e6f039372d4e0912f0e124"></a>
## children

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::children` · datafusion-ffi 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:639`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d212f3817763d00ddfa30684"></a>
## dynamic_expressions_produced

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::dynamic_expressions_produced` · datafusion-ffi 55.1.0

```rust
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:676`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43eba82b3f461eefcd974845"></a>
## execute

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::execute` · datafusion-ffi 55.1.0

```rust
fn execute(&self, _partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:668`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1dcf793b596c8bb44c602f6"></a>
## fmt

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [566, 14], "end": [566, 19], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af05d40e6240e993e96eff54"></a>
## fmt_as

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::fmt_as` · datafusion-ffi 55.1.0

```rust
fn fmt_as(&self, _t: DisplayFormatType, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 5], "end": [628, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/execution_plan.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e17a3d7653dbe9dc3ef5fe2"></a>
## metrics

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::metrics` · datafusion-ffi 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:680`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a31f986d8285cefe89465a"></a>
## name

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::name` · datafusion-ffi 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:631`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63782bc9c399ead759336f11"></a>
## new

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::new` · datafusion-ffi 55.1.0

```rust
fn new(schema: arrow::datatypes::SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [576, 5], "end": [618, 6], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b81a14aa62df7bfedfca9c5"></a>
## properties

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::properties` · datafusion-ffi 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:635`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc1c7410a2baa5d438a37322"></a>
## replace_children

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::replace_children` · datafusion-ffi 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:643`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d907684a6c6b4f272b0a49cf"></a>
## statistics_from_inputs

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::statistics_from_inputs` · datafusion-ffi 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:684`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5dd484649170da93262ffe3"></a>
## with_dynamic_expressions

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::with_dynamic_expressions` · datafusion-ffi 55.1.0

```rust
fn with_dynamic_expressions(self, dynamic_expressions: Vec<Arc<dyn PhysicalExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [576, 5], "end": [618, 6], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:611`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8000e6458ff39c4e6ec4dcbf"></a>
## with_expressions

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::with_expressions` · datafusion-ffi 55.1.0

```rust
fn with_expressions(self, expressions: Vec<Arc<dyn PhysicalExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [576, 5], "end": [618, 6], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7419bce564329b04a8a30114"></a>
## with_metrics

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::with_metrics` · datafusion-ffi 55.1.0

```rust
fn with_metrics(self, metrics: MetricsSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [576, 5], "end": [618, 6], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:593`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9d59a1e6c1b9d95bec84900"></a>
## with_new_children

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::with_new_children` · datafusion-ffi 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [630, 5], "end": [700, 6], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af6e294c74380c57f3a2886a"></a>
## with_statistics

`function` · `datafusion_ffi::execution_plan::tests::EmptyExec::with_statistics` · datafusion-ffi 55.1.0

```rust
fn with_statistics(self, statistics: Statistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::tests::EmptyExec", "path": "EmptyExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [576, 5], "end": [618, 6], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:598`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
