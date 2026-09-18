# `datafusion_common::cse::HashNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.HashNode.json).

<a id="op-770172abaa178f889bde92b3"></a>
## HashNode

`trait` · `datafusion_common::cse::HashNode` · datafusion-common 55.1.0

```rust
trait HashNode
```

Source: `src/cse.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Hashes the direct content of an [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) without recursing into its children.

This method is useful to incrementally compute hashes, such as in [`CSE`](../operations/datafusion_common.cse.CSE.md#op-7313c40d02d9c07f20f71723) which builds
a deep hash of a node and its descendants during the bottom-up phase of the first
traversal and so avoid computing the hash of the node and then the hash of its
descendants separately.

If a node doesn't have any children then the value returned by `hash_node()` is
similar to '.hash()`, but not necessarily returns the same value.

<a id="op-c83d6ce2bc2650595b3afadd"></a>
## hash_node

`function` · `datafusion_common::cse::HashNode::hash_node` · datafusion-common 55.1.0

```rust
fn hash_node<H: Hasher>(&self, state: &mut H)
```

Source: `src/cse.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
