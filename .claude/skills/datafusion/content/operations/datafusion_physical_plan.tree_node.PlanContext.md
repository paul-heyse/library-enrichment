# `datafusion_physical_plan::tree_node::PlanContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.tree_node.PlanContext.json).

<a id="op-7fdf4c168599e89594546bfb"></a>
## PlanContext

`struct` · `datafusion_physical_plan::tree_node::PlanContext` · datafusion-physical-plan 55.1.0

```rust
struct PlanContext<T: Sized>
```

Source: `src/tree_node.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A node context object beneficial for writing optimizer rules.
This context encapsulating an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) node with a payload.

Since each wrapped node has it's children within both the `PlanContext.plan.children()`,
as well as separately within the `PlanContext.children` (which are child nodes wrapped in the context),
it's important to keep these child plans in sync when performing mutations.

Since there are two ways to access child plans directly -— it's recommended
to perform mutable operations via [`Self::update_plan_from_children`](../operations/datafusion_physical_plan.tree_node.PlanContext.md#op-117f25fafbc1ef6c201ab812).
After mutating the `PlanContext.children`, or after creating the `PlanContext`,
call `update_plan_from_children` to sync.

<a id="op-34c515c95036131809550f25"></a>
## children

`struct_field` · `datafusion_physical_plan::tree_node::PlanContext::children` · datafusion-physical-plan 55.1.0

```rust
children: Vec<Self>
```

Source: `src/tree_node.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Child contexts of this node.

<a id="op-45ceeb256030589e66dc7e4c"></a>
## children

`function` · `datafusion_physical_plan::tree_node::PlanContext::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> &[Self]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [118, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::ConcreteTreeNode", "path": "ConcreteTreeNode"}, "trait_path": "datafusion_common::tree_node::ConcreteTreeNode"}`

Source: `src/tree_node.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50bab269696e23ee742a7294"></a>
## data

`struct_field` · `datafusion_physical_plan::tree_node::PlanContext::data` · datafusion-physical-plan 55.1.0

```rust
data: T
```

Source: `src/tree_node.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Custom data payload of the node.

<a id="op-3dbf7bfaa45d87b83e54386a"></a>
## fmt

`function` · `datafusion_physical_plan::tree_node::PlanContext::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tree_node.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c0de2c50291f732b941d896"></a>
## fmt

`function` · `datafusion_physical_plan::tree_node::PlanContext::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [102, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tree_node.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3a55e7736fa55d24918b5f6"></a>
## new

`function` · `datafusion_physical_plan::tree_node::PlanContext::new` · datafusion-physical-plan 55.1.0

```rust
fn new(plan: Arc<dyn ExecutionPlan>, data: T, children: Vec<Self>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [81, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0c34276fd8fa65c63d4e655"></a>
## new_default

`function` · `datafusion_physical_plan::tree_node::PlanContext::new_default` · datafusion-physical-plan 55.1.0

```rust
fn new_default(plan: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [93, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00459079509e2a3b39c03730"></a>
## plan

`struct_field` · `datafusion_physical_plan::tree_node::PlanContext::plan` · datafusion-physical-plan 55.1.0

```rust
plan: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/tree_node.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The execution plan associated with this context.

<a id="op-a88f97ae21028238632326fc"></a>
## take_children

`function` · `datafusion_physical_plan::tree_node::PlanContext::take_children` · datafusion-physical-plan 55.1.0

```rust
fn take_children(self) -> (Self, Vec<Self>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [118, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::ConcreteTreeNode", "path": "ConcreteTreeNode"}, "trait_path": "datafusion_common::tree_node::ConcreteTreeNode"}`

Source: `src/tree_node.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-117f25fafbc1ef6c201ab812"></a>
## update_plan_from_children

`function` · `datafusion_physical_plan::tree_node::PlanContext::update_plan_from_children` · datafusion-physical-plan 55.1.0

```rust
fn update_plan_from_children(self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [81, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Update the `PlanContext.plan.children()` from the `PlanContext.children`,
if the `PlanContext.children` have been changed.

<a id="op-c0c206543df293ee3ff54ea1"></a>
## with_new_children

`function` · `datafusion_physical_plan::tree_node::PlanContext::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(self, children: Vec<Self>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::tree_node::PlanContext", "path": "PlanContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [118, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::ConcreteTreeNode", "path": "ConcreteTreeNode"}, "trait_path": "datafusion_common::tree_node::ConcreteTreeNode"}`

Source: `src/tree_node.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
