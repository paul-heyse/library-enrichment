# `datafusion_common::cse::FoundCommonNodes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.FoundCommonNodes.json).

<a id="op-382c86505deaddf19f722887"></a>
## FoundCommonNodes

`enum` · `datafusion_common::cse::FoundCommonNodes` · datafusion-common 55.1.0

```rust
enum FoundCommonNodes<N>
```

Source: `src/cse.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The result of potentially rewriting a list of [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s to eliminate common
subtrees.

<a id="op-55d8ece9065f2dbef6d859e4"></a>
## No

`variant` · `datafusion_common::cse::FoundCommonNodes::No` · datafusion-common 55.1.0

```rust
No
```

Source: `src/cse.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No common [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s were found

<a id="op-d2ff7a1742df8a713e8f63c3"></a>
## Yes

`variant` · `datafusion_common::cse::FoundCommonNodes::Yes` · datafusion-common 55.1.0

```rust
Yes
```

Source: `src/cse.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Common [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29)s were found

<a id="op-1fda41d55e467ec55fad9253"></a>
## fmt

`function` · `datafusion_common::cse::FoundCommonNodes::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "N"}}], "constraints": []}}, "id": "datafusion_common::cse::FoundCommonNodes", "path": "FoundCommonNodes"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 10], "end": [209, 15], "filename": "src/cse.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cse.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
