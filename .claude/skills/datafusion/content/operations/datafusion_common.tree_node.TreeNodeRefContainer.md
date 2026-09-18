# `datafusion_common::tree_node::TreeNodeRefContainer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TreeNodeRefContainer.json).

<a id="op-448a0103d611637e63be3b9b"></a>
## TreeNodeRefContainer

`trait` · `datafusion_common::tree_node::TreeNodeRefContainer` · datafusion-common 55.1.0

```rust
trait TreeNodeRefContainer<'a, T: 'a>: Sized
```

Source: `src/tree_node.rs:1070`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

[`TreeNodeRefContainer`](../operations/datafusion_common.tree_node.TreeNodeRefContainer.md#op-448a0103d611637e63be3b9b) contains references to elements that a function can be
applied on. The elements of the container are siblings so the continuation rules are
similar to [`TreeNodeRecursion::visit_sibling`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-15bc155415d574d7fcc55b26).

This container is similar to [`TreeNodeContainer`](../operations/datafusion_common.tree_node.TreeNodeContainer.md#op-3ec7d2edd771ac78a03ad2f6), but the lifetime of the reference
elements (`T`) are not derived from the container's lifetime.
A typical usage of this container is in `Expr::apply_children` when we need to
construct a temporary container to be able to call `apply_ref_elements` on a
collection of tree node references. But in that case the container's temporary
lifetime is different to the lifetime of tree nodes that we put into it.
Please find an example use case in `Expr::apply_children` with the `Expr::Case` case.

Most of the cases we don't need to create a temporary container with
`TreeNodeRefContainer`, but we can just call `TreeNodeContainer::apply_elements`.
Please find an example use case in `Expr::apply_children` with the `Expr::GroupingSet`
case.

<a id="op-894be7655e119db715efeac6"></a>
## apply_ref_elements

`function` · `datafusion_common::tree_node::TreeNodeRefContainer::apply_ref_elements` · datafusion-common 55.1.0

```rust
fn apply_ref_elements<F: FnMut(&'a T) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:1074`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Applies `f` to all elements of the container.
This method is usually called from [`TreeNode::apply_children`](../operations/datafusion_common.tree_node.TreeNode.md#op-5865e77f6b800cfeddf0e0fc) implementations as
a node is actually a container of the node's children.
