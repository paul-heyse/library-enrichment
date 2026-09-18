# `datafusion_common::cse::CSE`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.CSE.json).

<a id="op-7313c40d02d9c07f20f71723"></a>
## CSE

`struct` · `datafusion_common::cse::CSE` · datafusion-common 55.1.0

```rust
struct CSE<N, C: CSEController<Node = N>>
```

Source: `src/cse.rs:514`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The main entry point of Common Subexpression Elimination.

[`CSE`](../operations/datafusion_common.cse.CSE.md#op-7313c40d02d9c07f20f71723) requires a [`CSEController`](../operations/datafusion_common.cse.CSEController.md#op-086e29ba8a5925bafea4e457), that defines how common subtrees of a particular
[`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) tree can be eliminated. The elimination process can be started with the
[`CSE::extract_common_nodes()`](../operations/datafusion_common.cse.CSE.md#op-0705fb493008e433cf3185b8) method.

<a id="op-0705fb493008e433cf3185b8"></a>
## extract_common_nodes

`function` · `datafusion_common::cse::CSE::extract_common_nodes` · datafusion-common 55.1.0

```rust
fn extract_common_nodes(&mut self, nodes_list: Vec<Vec<N>>) -> Result<FoundCommonNodes<N>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "N"}}, {"type": {"generic": "C"}}], "constraints": []}}, "id": "datafusion_common::cse::CSE", "path": "CSE"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "N"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "C"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNode", "path": "TreeNode"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_common::cse::HashNode", "path": "HashNode"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_common::cse::NormalizeEq", "path": "NormalizeEq"}}}], "generic_params": [], "type": {"generic": "N"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"generic": "N"}}}, "name": "Node"}]}}, "id": "datafusion_common::cse::CSEController", "path": "CSEController"}}}], "generic_params": [], "type": {"generic": "C"}}}]}, "is_negative": false, "span": {"begin": [520, 1], "end": [675, 2], "filename": "src/cse.rs"}, "trait": null, "trait_path": null}`

Source: `src/cse.rs:634`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Extracts common [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s and rewrites `nodes_list`.

Returns [`FoundCommonNodes`](../operations/datafusion_common.cse.FoundCommonNodes.md#op-382c86505deaddf19f722887) recording the result of the extraction.

<a id="op-52ff9581e9e215002e85d61c"></a>
## new

`function` · `datafusion_common::cse::CSE::new` · datafusion-common 55.1.0

```rust
fn new(controller: C) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "N"}}, {"type": {"generic": "C"}}], "constraints": []}}, "id": "datafusion_common::cse::CSE", "path": "CSE"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "N"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "C"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNode", "path": "TreeNode"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_common::cse::HashNode", "path": "HashNode"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_common::cse::NormalizeEq", "path": "NormalizeEq"}}}], "generic_params": [], "type": {"generic": "N"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"generic": "N"}}}, "name": "Node"}]}}, "id": "datafusion_common::cse::CSEController", "path": "CSEController"}}}], "generic_params": [], "type": {"generic": "C"}}}]}, "is_negative": false, "span": {"begin": [520, 1], "end": [675, 2], "filename": "src/cse.rs"}, "trait": null, "trait_path": null}`

Source: `src/cse.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
