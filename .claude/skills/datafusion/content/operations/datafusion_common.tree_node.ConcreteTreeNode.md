# `datafusion_common::tree_node::ConcreteTreeNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.ConcreteTreeNode.json).

<a id="op-01691971710eeae6470b8097"></a>
## ConcreteTreeNode

`trait` · `datafusion_common::tree_node::ConcreteTreeNode` · datafusion-common 55.1.0

```rust
trait ConcreteTreeNode: Sized
```

Source: `src/tree_node.rs:1317`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Instead of implementing [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29), it's recommended to implement a [`ConcreteTreeNode`](../operations/datafusion_common.tree_node.ConcreteTreeNode.md#op-01691971710eeae6470b8097) for
trees that contain nodes with payloads. This approach ensures safe execution of algorithms
involving payloads, by enforcing rules for detaching and reattaching child nodes.

<a id="op-3409f12b6e59ca40fffe7441"></a>
## children

`function` · `datafusion_common::tree_node::ConcreteTreeNode::children` · datafusion-common 55.1.0

```rust
fn children(&self) -> &[Self]
```

Source: `src/tree_node.rs:1319`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Provides read-only access to child nodes.

<a id="op-1b92a929600f0d26c9daa456"></a>
## take_children

`function` · `datafusion_common::tree_node::ConcreteTreeNode::take_children` · datafusion-common 55.1.0

```rust
fn take_children(self) -> (Self, Vec<Self>)
```

Source: `src/tree_node.rs:1322`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Detaches the node from its children, returning the node itself and its detached children.

<a id="op-e66d4d07eee9c5e6932ea827"></a>
## with_new_children

`function` · `datafusion_common::tree_node::ConcreteTreeNode::with_new_children` · datafusion-common 55.1.0

```rust
fn with_new_children(self, children: Vec<Self>) -> Result<Self>
```

Source: `src/tree_node.rs:1325`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Reattaches updated child nodes to the node, returning the updated node.
