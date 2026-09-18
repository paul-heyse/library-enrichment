# `datafusion_ffi::execution_plan::FFI_ExecutionPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.execution_plan.FFI_ExecutionPlan.json).

<a id="op-9600304f9dbd2887afcc0a23"></a>
## FFI_ExecutionPlan

`struct` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan` · datafusion-ffi 55.1.0

```rust
struct FFI_ExecutionPlan
```

Source: `src/execution_plan.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing a [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) across FFI boundaries.

<a id="op-9804c61ee02430e1d8e1607b"></a>
## apply_expressions

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::apply_expressions` · datafusion-ffi 55.1.0

```rust
apply_expressions: unsafe fn(&Self) -> util::FFI_Result<stabby::vec::Vec<physical_expr::FFI_PhysicalExpr>>
```

Source: `src/execution_plan.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the physical expression roots owned by this plan node.

<a id="op-58c6945524eeabd2c4938f29"></a>
## children

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::children` · datafusion-ffi 55.1.0

```rust
children: unsafe fn(&Self) -> stabby::vec::Vec<FFI_ExecutionPlan>
```

Source: `src/execution_plan.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return a vector of children plans

<a id="op-458930a0caddaf1b8e128dad"></a>
## clone

`function` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::FFI_ExecutionPlan", "path": "FFI_ExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [276, 1], "end": [280, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_plan.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee9dd7040e14ead322a8ffd0"></a>
## clone

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/execution_plan.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-2a7638da383d0b0345e11f6d"></a>
## drop

`function` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::FFI_ExecutionPlan", "path": "FFI_ExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 1], "end": [372, 2], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/execution_plan.rs:369`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c650e839a6e5883a4cc3c3a"></a>
## dynamic_expressions_produced

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::dynamic_expressions_produced` · datafusion-ffi 55.1.0

```rust
dynamic_expressions_produced: unsafe fn(&Self) -> stabby::vec::Vec<physical_expr::FFI_PhysicalExpr>
```

Source: `src/execution_plan.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the dynamic expressions produced by this plan node.

<a id="op-b82f570d3a1f03bd89bb7faf"></a>
## execute

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::execute` · datafusion-ffi 55.1.0

```rust
execute: unsafe fn(&Self, usize, execution::FFI_TaskContext) -> util::FFI_Result<record_batch_stream::FFI_RecordBatchStream>
```

Source: `src/execution_plan.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Execute the plan and return a record batch stream. Errors
will be returned as a string.

<a id="op-c94a9c3d6fbb2f66fa795fc2"></a>
## fmt

`function` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::FFI_ExecutionPlan", "path": "FFI_ExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4eb589aac090349edf6b786"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/execution_plan.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-764b6d98b8aada70942d0e65"></a>
## metrics

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::metrics` · datafusion-ffi 55.1.0

```rust
metrics: unsafe fn(&Self) -> util::FFI_Option<physical_expr::metrics::FFI_MetricsSet>
```

Source: `src/execution_plan.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Snapshot the plan's execution metrics. Returns `None` when the
underlying [`ExecutionPlan::metrics`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-f9e5333c240a14987dead676) returned `None`.

<a id="op-e5910b141942bba01f8c3e36"></a>
## name

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::name` · datafusion-ffi 55.1.0

```rust
name: unsafe fn(&Self) -> stabby::string::String
```

Source: `src/execution_plan.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the plan name.

<a id="op-a15279e92185968c42e7311f"></a>
## new

`function` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::new` · datafusion-ffi 55.1.0

```rust
fn new(plan: Arc<dyn ExecutionPlan>, runtime: Option<Handle>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution_plan::FFI_ExecutionPlan", "path": "FFI_ExecutionPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [332, 1], "end": [366, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This function is called on the provider's side.

<a id="op-9f611134605eca39209ee794"></a>
## partition_statistics

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::partition_statistics` · datafusion-ffi 55.1.0

```rust
partition_statistics: unsafe fn(&Self, util::FFI_Option<usize>) -> util::FFI_Result<stabby::vec::Vec<u8>>
```

Source: `src/execution_plan.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Snapshot partition statistics. `partition == None` corresponds to
statistics over all partitions; `Some(idx)` corresponds to a specific
partition. The returned bytes are a prost-encoded
`datafusion_proto_common::Statistics`.

<a id="op-ca06c8981715153207f7c21d"></a>
## private_data

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/execution_plan.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
A [`ForeignExecutionPlan`](../operations/datafusion_ffi.execution_plan.ForeignExecutionPlan.md#op-62b284f47fee1f025f2be316) should never attempt to access this data.

<a id="op-b0398c988cd0d64a9a4dd87f"></a>
## properties

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::properties` · datafusion-ffi 55.1.0

```rust
properties: unsafe fn(&Self) -> plan_properties::FFI_PlanProperties
```

Source: `src/execution_plan.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the plan properties

<a id="op-c752674a311b0b7439b66640"></a>
## release

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/execution_plan.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-5cbccdfdf2c471cf8f7dd3d2"></a>
## repartitioned

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::repartitioned` · datafusion-ffi 55.1.0

```rust
repartitioned: unsafe fn(&Self, usize, config::FFI_ConfigOptions) -> util::FFI_Result<util::FFI_Option<FFI_ExecutionPlan>>
```

Source: `src/execution_plan.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-883a746ef321a352876246a8"></a>
## version

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/execution_plan.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.

<a id="op-f299d798ef0f25066017891d"></a>
## with_new_children

`struct_field` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan::with_new_children` · datafusion-ffi 55.1.0

```rust
with_new_children: unsafe fn(&Self, stabby::vec::Vec<Self>) -> util::FFI_Result<Self>
```

Source: `src/execution_plan.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
