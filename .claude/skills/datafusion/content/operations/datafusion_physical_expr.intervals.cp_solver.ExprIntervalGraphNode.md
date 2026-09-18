# `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.cp_solver.ExprIntervalGraphNode.json).

<a id="op-42c357458f2dc2a7ff534359"></a>
## ExprIntervalGraphNode

`struct` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode` · datafusion-physical-expr 55.1.0

```rust
struct ExprIntervalGraphNode
```

Source: `src/intervals/cp_solver.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This is a node in the DAEG; it encapsulates a reference to the actual
[`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) as well as an interval containing expression bounds.

<a id="op-042666d7088bac0c195bb699"></a>
## clone

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ExprIntervalGraphNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 10], "end": [183, 15], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/intervals/cp_solver.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2db808347ca9597915426e7"></a>
## eq

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [193, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/intervals/cp_solver.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a505f9dcf904f109385a430"></a>
## fmt

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 17], "end": [183, 22], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/intervals/cp_solver.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f00f9a6d8ea1fd20fb2facc2"></a>
## fmt

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [199, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/intervals/cp_solver.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b0a3c4a92c0ec5428b5680f"></a>
## interval

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::interval` · datafusion-physical-expr 55.1.0

```rust
fn interval(&self) -> &Interval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [232, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the interval object representing the range of the expression.

<a id="op-ba47b61c2616ff7de5657e13"></a>
## make_node

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::make_node` · datafusion-physical-expr 55.1.0

```rust
fn make_node(node: &ExprTreeNode<NodeIndex>, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [232, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function creates a DAEG node from DataFusion's [`ExprTreeNode`](../operations/datafusion_physical_expr.utils.ExprTreeNode.md#op-f3771825a1a9d4c62ed84d22)
object. Literals are created with definite, singleton intervals while
any other expression starts with an indefinite interval (`[-∞, ∞]`).

<a id="op-75012f9c5d051e1314f93786"></a>
## new_unbounded

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::new_unbounded` · datafusion-physical-expr 55.1.0

```rust
fn new_unbounded(expr: Arc<dyn PhysicalExpr>, dt: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [232, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Constructs a new DAEG node with an `[-∞, ∞]` range.

<a id="op-c035363d2864045589a20cd3"></a>
## new_with_interval

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode::new_with_interval` · datafusion-physical-expr 55.1.0

```rust
fn new_with_interval(expr: Arc<dyn PhysicalExpr>, interval: Interval) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode", "path": "ExprIntervalGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [232, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Constructs a new DAEG node with the given range.
