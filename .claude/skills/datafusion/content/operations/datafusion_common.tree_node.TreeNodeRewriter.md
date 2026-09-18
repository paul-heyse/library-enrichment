# `datafusion_common::tree_node::TreeNodeRewriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TreeNodeRewriter.json).

<a id="op-ce0e31a3e97b1b60aa392d42"></a>
## TreeNodeRewriter

`trait` · `datafusion_common::tree_node::TreeNodeRewriter` · datafusion-common 55.1.0

```rust
trait TreeNodeRewriter: Sized
```

Source: `src/tree_node.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively
rewriting [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s via [`TreeNode::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17).

For example you can implement this trait on a struct to rewrite `Expr` or
`LogicalPlan` that needs to track state during the rewrite.

See [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) for more details on available APIs

When passed to [`TreeNode::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17), [`TreeNodeRewriter::f_down`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-580851ccc9d593a3e9d6a1c1) and
[`TreeNodeRewriter::f_up`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-ad0d98dc4bb01fddb0a2a5c4) are invoked recursively on the tree.
See [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) for more details on controlling the traversal.

# Return Value
The returns value of `f_up` and `f_down` specifies how the tree walk should
proceed. See [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) for details. If an [`Err`] is returned,
the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeRewriter::f_up`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-ad0d98dc4bb01fddb0a2a5c4) or
[`TreeNodeRewriter::f_down`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-580851ccc9d593a3e9d6a1c1) that do nothing, consider using
[`TreeNode::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed) or [`TreeNode::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078) instead.

# See Also:
* [`TreeNode::visit`](../operations/datafusion_common.tree_node.TreeNode.md#op-b7ab737d845b0a801118a434) to inspect borrowed `TreeNode`s

Unresolved upstream links (retained, not inferred): ``Err``.

<a id="op-a2fd4e411d2c7d933c0a8de4"></a>
## Node

`assoc_type` · `datafusion_common::tree_node::TreeNodeRewriter::Node` · datafusion-common 55.1.0

```rust
Node
```

Source: `src/tree_node.rs:500`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The node type which is rewritable.

<a id="op-580851ccc9d593a3e9d6a1c1"></a>
## f_down

`function` · `datafusion_common::tree_node::TreeNodeRewriter::f_down` · datafusion-common 55.1.0

```rust
fn f_down(&mut self, node: Self::Node) -> Result<Transformed<Self::Node>>
```

Source: `src/tree_node.rs:504`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Invoked while traversing down the tree before any children are rewritten.
Default implementation returns the node as is and continues recursion.

<a id="op-ad0d98dc4bb01fddb0a2a5c4"></a>
## f_up

`function` · `datafusion_common::tree_node::TreeNodeRewriter::f_up` · datafusion-common 55.1.0

```rust
fn f_up(&mut self, node: Self::Node) -> Result<Transformed<Self::Node>>
```

Source: `src/tree_node.rs:510`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Invoked while traversing up the tree after all children have been rewritten.
Default implementation returns the node as is and continues recursion.
