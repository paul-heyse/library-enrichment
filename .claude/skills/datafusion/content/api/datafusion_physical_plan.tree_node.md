# `datafusion_physical_plan::tree_node`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.tree_node.json`](../model/datafusion_physical_plan.tree_node.json)

## PlanContext

`struct` · `datafusion_physical_plan::tree_node::PlanContext`

```rust
struct PlanContext<T: Sized>
```

**Fields**: `plan`, `data`, `children`

**Implements**: `core::fmt::Display`, `datafusion_common::tree_node::ConcreteTreeNode`

**Derives**: Debug

**Methods** (3)

```rust
fn new(plan: Arc<dyn ExecutionPlan>, data: T, children: Vec<Self>) -> Self
fn new_default(plan: Arc<dyn ExecutionPlan>) -> Self
fn update_plan_from_children(self) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_common::tree_node::ConcreteTreeNode`**

```rust
fn children(&self) -> &[Self]
fn take_children(self) -> (Self, Vec<Self>)
fn with_new_children(self, children: Vec<Self>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.tree_node.PlanContext.md).


A node context object beneficial for writing optimizer rules.
This context encapsulating an [`ExecutionPlan`] node with a payload.

Since each wrapped node has it's children within both the `PlanContext.plan.children()`,
as well as separately within the `PlanContext.children` (which are child nodes wrapped in the context),
it's important to keep these child plans in sync when performing mutations.

Since there are two ways to access child plans directly -— it's recommended
to perform mutable operations via [`Self::update_plan_from_children`].
After mutating the `PlanContext.children`, or after creating the `PlanContext`,
call `update_plan_from_children` to sync.

---
