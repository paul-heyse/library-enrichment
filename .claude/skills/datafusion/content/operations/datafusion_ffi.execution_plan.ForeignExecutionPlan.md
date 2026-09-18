# `datafusion_ffi::execution_plan::ForeignExecutionPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.execution_plan.ForeignExecutionPlan.json).

<a id="op-62b284f47fee1f025f2be316"></a>
## ForeignExecutionPlan

`struct` · `datafusion_ffi::execution_plan::ForeignExecutionPlan` · datafusion-ffi 55.1.0

```rust
struct ForeignExecutionPlan
```

Source: `src/execution_plan.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct is used to access an execution plan provided by a foreign
library across a FFI boundary.

The ForeignExecutionPlan is to be used by the caller of the plan, so it has
no knowledge or access to the private data. All interaction with the plan
must occur through the functions defined in FFI_ExecutionPlan.

<a id="op-b59c6a15d0f861e8c57881fb"></a>
## Error

`assoc_type` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 1], "end": [449, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::FFI_ExecutionPlan", "path": "FFI_ExecutionPlan"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/execution_plan.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaad137769479c3f9d197386"></a>
## apply_expressions

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::apply_expressions` · datafusion-ffi 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8c027a4c60ae03d10845948"></a>
## children

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::children` · datafusion-ffi 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b19368566c0e963f4407f526"></a>
## dynamic_expressions_produced

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::dynamic_expressions_produced` · datafusion-ffi 55.1.0

```rust
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn datafusion_physical_plan::PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4be1558c8875a67540a32152"></a>
## execute

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::execute` · datafusion-ffi 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b20b87b4c6e77a13aa52582d"></a>
## fmt

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 10], "end": [380, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e6a37734d810c25288cb1a9"></a>
## fmt_as

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::fmt_as` · datafusion-ffi 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [412, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/execution_plan.rs:392`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d51b5c9bc3128a0be688674"></a>
## metrics

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::metrics` · datafusion-ffi 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b115b7eff2651b2031e26e83"></a>
## name

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::name` · datafusion-ffi 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-213cc0c64c68a82a3f632537"></a>
## partition_statistics

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::partition_statistics` · datafusion-ffi 55.1.0

```rust
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-223721d27cf0642e14c5ddc8"></a>
## properties

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::properties` · datafusion-ffi 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aacce10652996ec506d4855"></a>
## repartitioned

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::repartitioned` · datafusion-ffi 55.1.0

```rust
fn repartitioned(&self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:528`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fa0fb21de156b8ddd9ec2c1"></a>
## replace_children

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::replace_children` · datafusion-ffi 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-996b7118f16cad5650ed3f8e"></a>
## try_from

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(plan: FFI_ExecutionPlan) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 1], "end": [449, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::FFI_ExecutionPlan", "path": "FFI_ExecutionPlan"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/execution_plan.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-568ff0585101182899270b8d"></a>
## with_new_children

`function` · `datafusion_ffi::execution_plan::ForeignExecutionPlan::with_new_children` · datafusion-ffi 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::ForeignExecutionPlan", "path": "ForeignExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [451, 1], "end": [556, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/execution_plan.rs:479`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
