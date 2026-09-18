# `datafusion_common::tree_node::TreeNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TreeNode.json).

<a id="op-19aa424c8e392fc74e9acd29"></a>
## TreeNode

`trait` · `datafusion_common::tree_node::TreeNode` · datafusion-common 55.1.0

```rust
trait TreeNode: Sized
```

Source: `src/tree_node.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

API for inspecting and rewriting tree data structures.

The `TreeNode` API is used to express algorithms separately from traversing
the structure of `TreeNode`s, avoiding substantial code duplication.

This trait is implemented for plans ([`ExecutionPlan`], [`LogicalPlan`]) and
expression trees ([`PhysicalExpr`], [`Expr`]) as well as Plan+Payload
combinations [`PlanContext`] and [`ExprContext`].

# Overview
There are three categories of TreeNode APIs:

1. "Inspecting" APIs to traverse a tree of `&TreeNodes`:
   [`apply`], [`visit`], [`exists`].

2. "Transforming" APIs that traverse and consume a tree of `TreeNode`s
   producing possibly changed `TreeNode`s: [`transform`], [`transform_up`],
   [`transform_down`], [`transform_down_up`], and [`rewrite`].

3. Internal APIs used to implement the `TreeNode` API: [`apply_children`],
   and [`map_children`].

| Traversal Order | Inspecting | Transforming |
| --- | --- | --- |
| top-down | [`apply`], [`exists`] | [`transform_down`]|
| bottom-up | | [`transform`] , [`transform_up`]|
| combined with separate `f_down` and `f_up` closures | | [`transform_down_up`] |
| combined with `f_down()` and `f_up()` in an object | [`visit`]  | [`rewrite`] |

**Note**:while there is currently no in-place mutation API that uses `&mut
TreeNode`, the transforming APIs are efficient and optimized to avoid
cloning.

[`apply`]: Self::apply
[`visit`]: Self::visit
[`exists`]: Self::exists
[`transform`]: Self::transform
[`transform_up`]: Self::transform_up
[`transform_down`]: Self::transform_down
[`transform_down_up`]: Self::transform_down_up
[`rewrite`]: Self::rewrite
[`apply_children`]: Self::apply_children
[`map_children`]: Self::map_children

# Terminology
The following terms are used in this trait

* `f_down`: Invoked before any children of the current node are visited.
* `f_up`: Invoked after all children of the current node are visited.
* `f`: closure that is applied to the current node.
* `map_*`: applies a transformation to rewrite owned nodes
* `apply_*`:  invokes a function on borrowed nodes
* `transform_`: applies a transformation to rewrite owned nodes

<!-- Since these are in the datafusion-common crate, can't use intra doc links) -->
[`ExecutionPlan`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html
[`PhysicalExpr`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.PhysicalExpr.html
[`LogicalPlan`]: https://docs.rs/datafusion-expr/latest/datafusion_expr/logical_plan/enum.LogicalPlan.html
[`Expr`]: https://docs.rs/datafusion-expr/latest/datafusion_expr/expr/enum.Expr.html
[`PlanContext`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/tree_node/struct.PlanContext.html
[`ExprContext`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/tree_node/struct.ExprContext.html

<a id="op-bd071500b276296c970d8bcd"></a>
## apply

`function` · `datafusion_common::tree_node::TreeNode::apply` · datafusion-common 55.1.0

```rust
fn apply<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Applies `f` to the node then each of its children, recursively (a
top-down, pre-order traversal).

The return [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) controls the recursion and can cause
an early return.

# See Also
* [`Self::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078) for the equivalent transformation API.
* [`Self::visit`](../operations/datafusion_common.tree_node.TreeNode.md#op-b7ab737d845b0a801118a434) for both top-down and bottom up traversal.

<a id="op-5865e77f6b800cfeddf0e0fc"></a>
## apply_children

`function` · `datafusion_common::tree_node::TreeNode::apply_children` · datafusion-common 55.1.0

```rust
fn apply_children<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Low-level API used to implement other APIs.

If you want to implement the [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) trait for your own type, you
should implement this method and [`Self::map_children`](../operations/datafusion_common.tree_node.TreeNode.md#op-68847d001fe9160150e23725).

Users should use one of the higher level APIs described on [`Self`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29).

Description: Apply `f` to inspect node's children (but not the node
itself).

<a id="op-1dd3d36dcdbf23fccd60ad7a"></a>
## exists

`function` · `datafusion_common::tree_node::TreeNode::exists` · datafusion-common 55.1.0

```rust
fn exists<F: FnMut(&Self) -> Result<bool>>(&self, f: F) -> Result<bool>
```

Source: `src/tree_node.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if `f` returns true for any node in the tree.

Stops recursion as soon as a matching node is found

<a id="op-68847d001fe9160150e23725"></a>
## map_children

`function` · `datafusion_common::tree_node::TreeNode::map_children` · datafusion-common 55.1.0

```rust
fn map_children<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Source: `src/tree_node.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Low-level API used to implement other APIs.

If you want to implement the [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) trait for your own type, you
should implement this method and [`Self::apply_children`](../operations/datafusion_common.tree_node.TreeNode.md#op-5865e77f6b800cfeddf0e0fc).

Users should use one of the higher level APIs described on [`Self`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29).

Description: Apply `f` to rewrite the node's children (but not the node itself).

<a id="op-af82c9ed79a9f7ee9db9ed17"></a>
## rewrite

`function` · `datafusion_common::tree_node::TreeNode::rewrite` · datafusion-common 55.1.0

```rust
fn rewrite<R: TreeNodeRewriter<Node = Self>>(self, rewriter: &mut R) -> Result<Transformed<Self>>
```

Source: `src/tree_node.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Rewrite the tree node with a [`TreeNodeRewriter`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-ce0e31a3e97b1b60aa392d42), performing a
depth-first walk of the node and its children.

[`TreeNodeRewriter::f_down()`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-580851ccc9d593a3e9d6a1c1) is called in top-down order (before
children are visited), [`TreeNodeRewriter::f_up()`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-ad0d98dc4bb01fddb0a2a5c4) is called in
bottom-up order (after children are visited).

Note: If using the default [`TreeNodeRewriter::f_up`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-ad0d98dc4bb01fddb0a2a5c4) or
[`TreeNodeRewriter::f_down`](../operations/datafusion_common.tree_node.TreeNodeRewriter.md#op-580851ccc9d593a3e9d6a1c1) that do nothing, consider using
[`Self::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078) instead.

# Return Value
The returns value specifies how the tree walk should proceed. See
[`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) for details. If an [`Err`] is returned, the
recursion stops immediately.

# See Also
* [`Self::visit`](../operations/datafusion_common.tree_node.TreeNode.md#op-b7ab737d845b0a801118a434) for inspecting (without modification) `TreeNode`s
* [Self::transform_down_up](../operations/datafusion_common.tree_node.TreeNode.md#op-e30b76102a8ade63c654b8d6) for a top-down (pre-order) traversal.
* [Self::transform_down](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078) for a top-down (pre-order) traversal.
* [`Self::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed) for a bottom-up (post-order) traversal.

# Example
Consider the following tree structure:
```text
ParentNode
   left: ChildNode1
   right: ChildNode2
```

Here, the nodes would be visited using the following order:
```text
TreeNodeRewriter::f_down(ParentNode)
TreeNodeRewriter::f_down(ChildNode1)
TreeNodeRewriter::f_up(ChildNode1)
TreeNodeRewriter::f_down(ChildNode2)
TreeNodeRewriter::f_up(ChildNode2)
TreeNodeRewriter::f_up(ParentNode)
```

Unresolved upstream links (retained, not inferred): ``Err``.

<a id="op-335537caf61ac03893f423ab"></a>
## transform

`function` · `datafusion_common::tree_node::TreeNode::transform` · datafusion-common 55.1.0

```rust
fn transform<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Source: `src/tree_node.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Recursively rewrite the node's children and then the node using `f`
(a bottom-up post-order traversal).

A synonym of [`Self::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed).

<a id="op-7f8f7d15d1860327f1638078"></a>
## transform_down

`function` · `datafusion_common::tree_node::TreeNode::transform_down` · datafusion-common 55.1.0

```rust
fn transform_down<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Source: `src/tree_node.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Recursively rewrite the tree using `f` in a top-down (pre-order)
fashion.

`f` is applied to the node first, and then its children.

# See Also
* [`Self::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed) for a bottom-up (post-order) traversal.
* [Self::transform_down_up](../operations/datafusion_common.tree_node.TreeNode.md#op-e30b76102a8ade63c654b8d6) for a combined traversal with closures
* [`Self::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17) for a combined traversal with a visitor

<a id="op-e30b76102a8ade63c654b8d6"></a>
## transform_down_up

`function` · `datafusion_common::tree_node::TreeNode::transform_down_up` · datafusion-common 55.1.0

```rust
fn transform_down_up<FD: FnMut(Self) -> Result<Transformed<Self>>, FU: FnMut(Self) -> Result<Transformed<Self>>>(self, f_down: FD, f_up: FU) -> Result<Transformed<Self>>
```

Source: `src/tree_node.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Transforms the node using `f_down` while traversing the tree top-down
(pre-order), and using `f_up` while traversing the tree bottom-up
(post-order).

The method behaves the same as calling [`Self::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078) followed
by [`Self::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed) on the same node. Use this method if you want
to start the `f_up` process right where `f_down` jumps. This can make
the whole process faster by reducing the number of `f_up` steps.

# See Also
* [`Self::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed) for a bottom-up (post-order) traversal.
* [Self::transform_down](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078) for a top-down (pre-order) traversal.
* [`Self::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17) for a combined traversal with a visitor

# Example
Consider the following tree structure:
```text
ParentNode
   left: ChildNode1
   right: ChildNode2
```

The nodes are visited using the following order:
```text
f_down(ParentNode)
f_down(ChildNode1)
f_up(ChildNode1)
f_down(ChildNode2)
f_up(ChildNode2)
f_up(ParentNode)
```

See [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) for more details on controlling the traversal.

If `f_down` or `f_up` returns [`Err`], the recursion stops immediately.

Example:
```text
                                              |   +---+
                                              |   | J |
                                              |   +---+
                                              |     |
                                              |   +---+
                 TreeNodeRecursion::Continue  |   | I |
                                              |   +---+
                                              |     |
                                              |   +---+
                                             \|/  | F |
                                              '   +---+
                                                 /     \ ___________________
                 When `f_down` is           +---+                           \ ---+
                 applied on node "E",       | E |                            | G |
                 it returns with "Jump".    +---+                            +---+
                                              |                                |
                                            +---+                            +---+
                                            | C |                            | H |
                                            +---+                            +---+
                                            /   \
                                       +---+     +---+
                                       | B |     | D |
                                       +---+     +---+
                                                   |
                                                 +---+
                                                 | A |
                                                 +---+

Instead of starting from leaf nodes, `f_up` starts from the node "E".
                                                  +---+
                                              |   | J |
                                              |   +---+
                                              |     |
                                              |   +---+
                                              |   | I |
                                              |   +---+
                                              |     |
                                             /    +---+
                                           /      | F |
                                         /        +---+
                                       /         /     \ ______________________
                                      |     +---+   .                          \ ---+
                                      |     | E |  /|\  After `f_down` jumps    | G |
                                      |     +---+   |   on node E, `f_up`       +---+
                                       \------| ---/   if applied on node E.      |
                                            +---+                               +---+
                                            | C |                               | H |
                                            +---+                               +---+
                                            /   \
                                       +---+     +---+
                                       | B |     | D |
                                       +---+     +---+
                                                   |
                                                 +---+
                                                 | A |
                                                 +---+
```

Unresolved upstream links (retained, not inferred): ``Err``.

<a id="op-86d9d244e6b5fe16b173f4ed"></a>
## transform_up

`function` · `datafusion_common::tree_node::TreeNode::transform_up` · datafusion-common 55.1.0

```rust
fn transform_up<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Source: `src/tree_node.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Recursively rewrite the node using `f` in a bottom-up (post-order)
fashion.

`f` is applied to the node's  children first, and then to the node itself.

# See Also
* [`Self::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078) top-down (pre-order) traversal.
* [Self::transform_down_up](../operations/datafusion_common.tree_node.TreeNode.md#op-e30b76102a8ade63c654b8d6) for a combined traversal with closures
* [`Self::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17) for a combined traversal with a visitor

<a id="op-b7ab737d845b0a801118a434"></a>
## visit

`function` · `datafusion_common::tree_node::TreeNode::visit` · datafusion-common 55.1.0

```rust
fn visit<'n, V: TreeNodeVisitor<'n, Node = Self>>(&'n self, visitor: &mut V) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Visit the tree node with a [`TreeNodeVisitor`](../operations/datafusion_common.tree_node.TreeNodeVisitor.md#op-6e3cec6b491fc9287d40a929), performing a
depth-first walk of the node and its children.

[`TreeNodeVisitor::f_down()`](../operations/datafusion_common.tree_node.TreeNodeVisitor.md#op-da8e078d592dc1735097a685) is called in top-down order (before
children are visited), [`TreeNodeVisitor::f_up()`](../operations/datafusion_common.tree_node.TreeNodeVisitor.md#op-3c802ca363a23c46f090fefc) is called in
bottom-up order (after children are visited).

# Return Value
Specifies how the tree walk ended. See [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) for details.

# See Also:
* [`Self::apply`](../operations/datafusion_common.tree_node.TreeNode.md#op-bd071500b276296c970d8bcd) for inspecting nodes with a closure
* [`Self::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17) to rewrite owned `TreeNode`s

# Example
Consider the following tree structure:
```text
ParentNode
   left: ChildNode1
   right: ChildNode2
```

Here, the nodes would be visited using the following order:
```text
TreeNodeVisitor::f_down(ParentNode)
TreeNodeVisitor::f_down(ChildNode1)
TreeNodeVisitor::f_up(ChildNode1)
TreeNodeVisitor::f_down(ChildNode2)
TreeNodeVisitor::f_up(ChildNode2)
TreeNodeVisitor::f_up(ParentNode)
```
