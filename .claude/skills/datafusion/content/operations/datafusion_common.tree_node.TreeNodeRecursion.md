# `datafusion_common::tree_node::TreeNodeRecursion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TreeNodeRecursion.json).

<a id="op-71e77b162ffdf243709ca1dc"></a>
## TreeNodeRecursion

`enum` · `datafusion_common::tree_node::TreeNodeRecursion` · datafusion-common 55.1.0

```rust
enum TreeNodeRecursion
```

Source: `src/tree_node.rs:517`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Controls how [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) recursions should proceed.

<a id="op-128238c101f7ed2798f6d11c"></a>
## Continue

`variant` · `datafusion_common::tree_node::TreeNodeRecursion::Continue` · datafusion-common 55.1.0

```rust
Continue
```

Source: `src/tree_node.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Continue recursion with the next node.

<a id="op-5524dc7ddeb08b422f96ae27"></a>
## Jump

`variant` · `datafusion_common::tree_node::TreeNodeRecursion::Jump` · datafusion-common 55.1.0

```rust
Jump
```

Source: `src/tree_node.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

In top-down traversals, skip recursing into children but continue with
the next node, which actually means pruning of the subtree.

In bottom-up traversals, bypass calling bottom-up closures till the next
leaf node.

In combined traversals, if it is the `f_down` (pre-order) phase, execution
"jumps" to the next `f_up` (post-order) phase by shortcutting its children.
If it is the `f_up` (post-order) phase, execution "jumps" to the next `f_down`
(pre-order) phase by shortcutting its parent nodes until the first parent node
having unvisited children path.

<a id="op-fbbedd1f0409ac786df535a1"></a>
## Stop

`variant` · `datafusion_common::tree_node::TreeNodeRecursion::Stop` · datafusion-common 55.1.0

```rust
Stop
```

Source: `src/tree_node.rs:533`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Stop recursion.

<a id="op-a7c9e3eab54ac222984a2406"></a>
## clone

`function` · `datafusion_common::tree_node::TreeNodeRecursion::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> TreeNodeRecursion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRecursion", "path": "TreeNodeRecursion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 28], "end": [516, 33], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tree_node.rs:516`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43aaed2245f3ebd55098566e"></a>
## eq

`function` · `datafusion_common::tree_node::TreeNodeRecursion::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &TreeNodeRecursion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRecursion", "path": "TreeNodeRecursion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 17], "end": [516, 26], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tree_node.rs:516`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ca6fef8fc64fbbf46d2df02"></a>
## fmt

`function` · `datafusion_common::tree_node::TreeNodeRecursion::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRecursion", "path": "TreeNodeRecursion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 10], "end": [516, 15], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tree_node.rs:516`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d0c0d7dc2862af05356b5cc"></a>
## visit_children

`function` · `datafusion_common::tree_node::TreeNodeRecursion::visit_children` · datafusion-common 55.1.0

```rust
fn visit_children<F: FnOnce() -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRecursion", "path": "TreeNodeRecursion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [536, 1], "end": [573, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc)
value and the fact that `f` is visiting the current node's children.

<a id="op-439620e2dd8866bec7249563"></a>
## visit_parent

`function` · `datafusion_common::tree_node::TreeNodeRecursion::visit_parent` · datafusion-common 55.1.0

```rust
fn visit_parent<F: FnOnce() -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRecursion", "path": "TreeNodeRecursion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [536, 1], "end": [573, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:564`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc)
value and the fact that `f` is visiting the current node's parent.

<a id="op-15bc155415d574d7fcc55b26"></a>
## visit_sibling

`function` · `datafusion_common::tree_node::TreeNodeRecursion::visit_sibling` · datafusion-common 55.1.0

```rust
fn visit_sibling<F: FnOnce() -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRecursion", "path": "TreeNodeRecursion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [536, 1], "end": [573, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc)
value and the fact that `f` is visiting the current node's sibling.
