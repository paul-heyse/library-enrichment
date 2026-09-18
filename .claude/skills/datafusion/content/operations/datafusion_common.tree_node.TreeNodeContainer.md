# `datafusion_common::tree_node::TreeNodeContainer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TreeNodeContainer.json).

<a id="op-3ec7d2edd771ac78a03ad2f6"></a>
## TreeNodeContainer

`trait` · `datafusion_common::tree_node::TreeNodeContainer` · datafusion-common 55.1.0

```rust
trait TreeNodeContainer<'a, T: 'a>: Sized
```

Source: `src/tree_node.rs:781`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

[`TreeNodeContainer`](../operations/datafusion_common.tree_node.TreeNodeContainer.md#op-3ec7d2edd771ac78a03ad2f6) contains elements that a function can be applied on or mapped.
The elements of the container are siblings so the continuation rules are similar to
[`TreeNodeRecursion::visit_sibling`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-15bc155415d574d7fcc55b26) / [`Transformed::transform_sibling`](../operations/datafusion_common.tree_node.Transformed.md#op-ec1488aced780e3217f315bd).

<a id="op-2166688c506d5270f8f3eae3"></a>
## apply_elements

`function` · `datafusion_common::tree_node::TreeNodeContainer::apply_elements` · datafusion-common 55.1.0

```rust
fn apply_elements<F: FnMut(&'a T) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:785`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Applies `f` to all elements of the container.
This method is usually called from [`TreeNode::apply_children`](../operations/datafusion_common.tree_node.TreeNode.md#op-5865e77f6b800cfeddf0e0fc) implementations as
a node is actually a container of the node's children.

<a id="op-c8efeaa5685b065346eb3e7d"></a>
## map_elements

`function` · `datafusion_common::tree_node::TreeNodeContainer::map_elements` · datafusion-common 55.1.0

```rust
fn map_elements<F: FnMut(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<Self>>
```

Source: `src/tree_node.rs:793`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maps all elements of the container with `f`.
This method is usually called from [`TreeNode::map_children`](../operations/datafusion_common.tree_node.TreeNode.md#op-68847d001fe9160150e23725) implementations as
a node is actually a container of the node's children.
