# `datafusion_ffi::execution_plan`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.execution_plan.json`](../model/datafusion_ffi.execution_plan.json)

## ExecutionPlanPrivateData

`struct` · `datafusion_ffi::execution_plan::ExecutionPlanPrivateData`

```rust
struct ExecutionPlanPrivateData
```

**Fields**: `plan`, `runtime`

---

## FFI_ExecutionPlan

`struct` · `datafusion_ffi::execution_plan::FFI_ExecutionPlan`

```rust
struct FFI_ExecutionPlan
```

**Fields**: `properties`, `children`, `apply_expressions`, `dynamic_expressions_produced`, `with_new_children`, `name`, `execute`, `repartitioned`, `metrics`, `partition_statistics`, `clone`, `release`, `version`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (1)

```rust
fn new(plan: Arc<dyn ExecutionPlan>, runtime: Option<Handle>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing a [`ExecutionPlan`] across FFI boundaries.

---

## ForeignExecutionPlan

`struct` · `datafusion_ffi::execution_plan::ForeignExecutionPlan`

```rust
struct ForeignExecutionPlan
```

**Implements**: `core::convert::TryFrom`, `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug, Send, Sync

**via `core::convert::TryFrom`**

```rust
fn try_from(plan: FFI_ExecutionPlan) -> Result<Self, Self::Error>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn datafusion_physical_plan::PhysicalExpr>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &str
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
fn properties(&self) -> &Arc<PlanProperties>
fn repartitioned(&self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

This struct is used to access an execution plan provided by a foreign
library across a FFI boundary.

The ForeignExecutionPlan is to be used by the caller of the plan, so it has
no knowledge or access to the private data. All interaction with the plan
must occur through the functions defined in FFI_ExecutionPlan.

---
