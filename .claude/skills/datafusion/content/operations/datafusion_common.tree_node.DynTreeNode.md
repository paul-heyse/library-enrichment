# `datafusion_common::tree_node::DynTreeNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.DynTreeNode.json).

<a id="op-1ef8f5b7d0cf03205404a43e"></a>
## DynTreeNode

`trait` · `datafusion_common::tree_node::DynTreeNode` · datafusion-common 55.1.0

```rust
trait DynTreeNode
```

Source: `src/tree_node.rs:1266`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Helper trait for implementing [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) that have children stored as
`Arc`s. If some trait object, such as `dyn T`, implements this trait,
its related `Arc<dyn T>` will automatically implement [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29).

<a id="op-abe9ab5f0180ff81b5a982ff"></a>
## arc_children

`function` · `datafusion_common::tree_node::DynTreeNode::arc_children` · datafusion-common 55.1.0

```rust
fn arc_children(&self) -> Vec<&Arc<Self>>
```

Source: `src/tree_node.rs:1268`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns all children of the specified `TreeNode`.

<a id="op-654ff040a6d4c01c649f9573"></a>
## with_new_arc_children

`function` · `datafusion_common::tree_node::DynTreeNode::with_new_arc_children` · datafusion-common 55.1.0

```rust
fn with_new_arc_children(&self, arc_self: Arc<Self>, new_children: Vec<Arc<Self>>) -> Result<Arc<Self>>
```

Source: `src/tree_node.rs:1271`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Constructs a new node with the specified children.
