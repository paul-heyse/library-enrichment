# `datafusion_physical_expr_common::tree_node`

Crate `datafusion-physical-expr-common` · 1 public items · structured records in [`model/datafusion_physical_expr_common.tree_node.json`](../model/datafusion_physical_expr_common.tree_node.json)

## ExprContext

`struct` · `datafusion_physical_expr_common::tree_node::ExprContext`

Also reachable as `datafusion_physical_expr::tree_node::ExprContext`

```rust
struct ExprContext<T: Sized>
```

**Fields**: `expr`, `data`, `children`

**Implements**: `core::fmt::Display`, `datafusion_common::tree_node::ConcreteTreeNode`

**Derives**: Debug

**Methods** (4)

```rust
fn new(expr: Arc<dyn PhysicalExpr>, data: T, children: Vec<Self>) -> Self
fn new_default(plan: Arc<dyn PhysicalExpr>) -> Self
fn new_unknown(expr: Arc<dyn PhysicalExpr>) -> Self
fn update_expr_from_children(self) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.tree_node.ExprContext.md).


A node object encapsulating a [`PhysicalExpr`] node with a payload. Since there are
two ways to access child plans—directly from the plan  and through child nodes—it's
recommended to perform mutable operations via [`Self::update_expr_from_children`].

---
