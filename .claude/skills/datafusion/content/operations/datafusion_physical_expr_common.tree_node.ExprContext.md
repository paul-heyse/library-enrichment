# `datafusion_physical_expr_common::tree_node::ExprContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.tree_node.ExprContext.json).

<a id="op-d0f3f03f01b7fb9f248f4f76"></a>
## ExprContext

`struct` · `datafusion_physical_expr_common::tree_node::ExprContext` · datafusion-physical-expr-common 55.1.0

```rust
struct ExprContext<T: Sized>
```

Source: `src/tree_node.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A node object encapsulating a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) node with a payload. Since there are
two ways to access child plans—directly from the plan  and through child nodes—it's
recommended to perform mutable operations via [`Self::update_expr_from_children`](../operations/datafusion_physical_expr_common.tree_node.ExprContext.md#op-9be0387548bb6eae7f1d78a5).

<a id="op-be7d86976a1e2e9085e6e2c4"></a>
## children

`struct_field` · `datafusion_physical_expr_common::tree_node::ExprContext::children` · datafusion-physical-expr-common 55.1.0

```rust
children: Vec<Self>
```

Source: `src/tree_node.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Child contexts of this node.

<a id="op-ff26d8e9de5af5f619e88b6b"></a>
## children

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::children` · datafusion-physical-expr-common 55.1.0

```rust
fn children(&self) -> &[Self]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [105, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::ConcreteTreeNode", "path": "ConcreteTreeNode"}, "trait_path": "datafusion_common::tree_node::ConcreteTreeNode"}`

Source: `src/tree_node.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b377fb7a12ed2693421c07a"></a>
## data

`struct_field` · `datafusion_physical_expr_common::tree_node::ExprContext::data` · datafusion-physical-expr-common 55.1.0

```rust
data: T
```

Source: `src/tree_node.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Custom data payload of the node.

<a id="op-32362cc2e54d1e1965f8dbc6"></a>
## expr

`struct_field` · `datafusion_physical_expr_common::tree_node::ExprContext::expr` · datafusion-physical-expr-common 55.1.0

```rust
expr: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/tree_node.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The physical expression associated with this context.

<a id="op-aebba59b8114c625465cf3d4"></a>
## fmt

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [89, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tree_node.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be244b65596926901c4a7cd0"></a>
## fmt

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tree_node.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f04fb41f8528f24087348a3a"></a>
## new

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(expr: Arc<dyn PhysicalExpr>, data: T, children: Vec<Self>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [69, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22a191296a0bd17b6b1b18dd"></a>
## new_default

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::new_default` · datafusion-physical-expr-common 55.1.0

```rust
fn new_default(plan: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [81, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49255a19b4b77e218eb85a1a"></a>
## new_unknown

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::new_unknown` · datafusion-physical-expr-common 55.1.0

```rust
fn new_unknown(expr: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Constructs a new `ExprPropertiesNode` with unknown properties for a
given physical expression. This node initializes with default properties
and recursively applies this to all child expressions.

<a id="op-9ba7de24a51fa1e40d8b2df1"></a>
## take_children

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::take_children` · datafusion-physical-expr-common 55.1.0

```rust
fn take_children(self) -> (Self, Vec<Self>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [105, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::ConcreteTreeNode", "path": "ConcreteTreeNode"}, "trait_path": "datafusion_common::tree_node::ConcreteTreeNode"}`

Source: `src/tree_node.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9be0387548bb6eae7f1d78a5"></a>
## update_expr_from_children

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::update_expr_from_children` · datafusion-physical-expr-common 55.1.0

```rust
fn update_expr_from_children(self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [69, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d12c051575929e12514df6a1"></a>
## with_new_children

`function` · `datafusion_physical_expr_common::tree_node::ExprContext::with_new_children` · datafusion-physical-expr-common 55.1.0

```rust
fn with_new_children(self, children: Vec<Self>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::tree_node::ExprContext", "path": "ExprContext"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [105, 2], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::ConcreteTreeNode", "path": "ConcreteTreeNode"}, "trait_path": "datafusion_common::tree_node::ConcreteTreeNode"}`

Source: `src/tree_node.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
