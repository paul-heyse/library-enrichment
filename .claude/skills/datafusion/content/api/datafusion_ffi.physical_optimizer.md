# `datafusion_ffi::physical_optimizer`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.physical_optimizer.json`](../model/datafusion_ffi.physical_optimizer.json)

## FFI_PhysicalOptimizerContext

`struct` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext`

```rust
struct FFI_PhysicalOptimizerContext
```

**Fields**: `config_options`, `release`, `private_data`

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug, Send, Sync

**Methods** (1)

```rust
fn new(context: &dyn PhysicalOptimizerContext) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing [`PhysicalOptimizerContext`] across FFI boundaries.

This provides access to configuration options for optimizer rules that need
extended context beyond the plan itself.

---

## FFI_PhysicalOptimizerRule

`struct` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule`

```rust
struct FFI_PhysicalOptimizerRule
```

**Fields**: `optimize`, `name`, `schema_check`, `clone`, `release`, `version`, `optimize_with_context`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (1)

```rust
fn new(rule: Arc<dyn PhysicalOptimizerRule + Send + Sync>, runtime: Option<Handle>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing [`PhysicalOptimizerRule`] across FFI boundaries.

---

## ForeignPhysicalOptimizerRule

`struct` · `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule`

```rust
struct ForeignPhysicalOptimizerRule
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug, Send, Sync

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn optimize_with_context(&self, plan: Arc<dyn ExecutionPlan>, context: &dyn PhysicalOptimizerContext) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_PhysicalOptimizerRule to interact with the foreign rule.

---
