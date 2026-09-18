# `datafusion_common::cse::FoundCommonNodes::Yes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.FoundCommonNodes.Yes.json).

<a id="op-954d0904d33e06151ea7312c"></a>
## common_nodes

`struct_field` · `datafusion_common::cse::FoundCommonNodes::Yes::common_nodes` · datafusion-common 55.1.0

```rust
common_nodes: Vec<(N, String)>
```

Source: `src/cse.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

extracted common [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)

<a id="op-f6d79a9b6946bb2cb09d0380"></a>
## new_nodes_list

`struct_field` · `datafusion_common::cse::FoundCommonNodes::Yes::new_nodes_list` · datafusion-common 55.1.0

```rust
new_nodes_list: Vec<Vec<N>>
```

Source: `src/cse.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

new [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s with common subtrees replaced

<a id="op-76f38e9b4ca9e4e9b7bb2ebd"></a>
## original_nodes_list

`struct_field` · `datafusion_common::cse::FoundCommonNodes::Yes::original_nodes_list` · datafusion-common 55.1.0

```rust
original_nodes_list: Vec<Vec<N>>
```

Source: `src/cse.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

original [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s
