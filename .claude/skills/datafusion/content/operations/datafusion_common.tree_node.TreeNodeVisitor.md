# `datafusion_common::tree_node::TreeNodeVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TreeNodeVisitor.json).

<a id="op-6e3cec6b491fc9287d40a929"></a>
## TreeNodeVisitor

`trait` · `datafusion_common::tree_node::TreeNodeVisitor` · datafusion-common 55.1.0

```rust
trait TreeNodeVisitor<'n>: Sized
```

Source: `src/tree_node.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively
inspecting [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s via [`TreeNode::visit`](../operations/datafusion_common.tree_node.TreeNode.md#op-b7ab737d845b0a801118a434).

See [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) for more details on available APIs

When passed to [`TreeNode::visit`](../operations/datafusion_common.tree_node.TreeNode.md#op-b7ab737d845b0a801118a434), [`TreeNodeVisitor::f_down`](../operations/datafusion_common.tree_node.TreeNodeVisitor.md#op-da8e078d592dc1735097a685) and
[`TreeNodeVisitor::f_up`](../operations/datafusion_common.tree_node.TreeNodeVisitor.md#op-3c802ca363a23c46f090fefc) are invoked recursively on the tree.
See [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) for more details on controlling the traversal.

# Return Value
The returns value of `f_up` and `f_down` specifies how the tree walk should
proceed. See [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) for details. If an [`Err`] is returned,
the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeVisitor::f_up`](../operations/datafusion_common.tree_node.TreeNodeVisitor.md#op-3c802ca363a23c46f090fefc) or
[`TreeNodeVisitor::f_down`](../operations/datafusion_common.tree_node.TreeNodeVisitor.md#op-da8e078d592dc1735097a685) that do nothing, consider using
[`TreeNode::apply`](../operations/datafusion_common.tree_node.TreeNode.md#op-bd071500b276296c970d8bcd) instead.

# See Also:
* [`TreeNode::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17) to rewrite owned `TreeNode`s

Unresolved upstream links (retained, not inferred): ``Err``.

<a id="op-11a0600d09e8e845cc910269"></a>
## Node

`assoc_type` · `datafusion_common::tree_node::TreeNodeVisitor::Node` · datafusion-common 55.1.0

```rust
Node
```

Source: `src/tree_node.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The node type which is visitable.

<a id="op-da8e078d592dc1735097a685"></a>
## f_down

`function` · `datafusion_common::tree_node::TreeNodeVisitor::f_down` · datafusion-common 55.1.0

```rust
fn f_down(&mut self, _node: &'n Self::Node) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Invoked while traversing down the tree, before any children are visited.
Default implementation continues the recursion.

<a id="op-3c802ca363a23c46f090fefc"></a>
## f_up

`function` · `datafusion_common::tree_node::TreeNodeVisitor::f_up` · datafusion-common 55.1.0

```rust
fn f_up(&mut self, _node: &'n Self::Node) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Invoked while traversing up the tree after children are visited. Default
implementation continues the recursion.
