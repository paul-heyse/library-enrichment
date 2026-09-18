# `datafusion_common::cse`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.json).

<a id="op-d0f4b0443c3d3fbce71388f5"></a>
## cse

`module` · `datafusion_common::cse` · datafusion-common 55.1.0

```rust
mod cse
```

Source: `src/cse.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Common Subexpression Elimination logic implemented in [`CSE`](../operations/datafusion_common.cse.CSE.md#op-7313c40d02d9c07f20f71723) can be controlled with
a [`CSEController`](../operations/datafusion_common.cse.CSEController.md#op-086e29ba8a5925bafea4e457), that defines how to eliminate common subtrees from a particular
[`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) tree.
