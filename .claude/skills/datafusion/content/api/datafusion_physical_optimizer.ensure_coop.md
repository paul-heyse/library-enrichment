# `datafusion_physical_optimizer::ensure_coop`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.ensure_coop.json`](../model/datafusion_physical_optimizer.ensure_coop.json)

## EnsureCooperative

`struct` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative`

```rust
struct EnsureCooperative
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

`EnsureCooperative` is a [`PhysicalOptimizerRule`] that inspects the physical plan for
sub plans that do not participate in cooperative scheduling. The plan is subdivided into sub
plans on eager evaluation boundaries. Leaf nodes and eager evaluation roots are checked
to see if they participate in cooperative scheduling. Those that do no are wrapped in
a [`CooperativeExec`] parent.

---
