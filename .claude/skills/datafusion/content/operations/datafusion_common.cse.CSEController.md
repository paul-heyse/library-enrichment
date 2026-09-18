# `datafusion_common::cse::CSEController`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.CSEController.json).

<a id="op-086e29ba8a5925bafea4e457"></a>
## CSEController

`trait` · `datafusion_common::cse::CSEController` · datafusion-common 55.1.0

```rust
trait CSEController
```

Source: `src/cse.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) specific definition of elimination.

<a id="op-e450c06cbcafb5fa811fed80"></a>
## Node

`assoc_type` · `datafusion_common::cse::CSEController::Node` · datafusion-common 55.1.0

```rust
Node
```

Source: `src/cse.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The type of the tree nodes.

<a id="op-882bc762e2d26d9d9c8fc1c1"></a>
## conditional_children

`function` · `datafusion_common::cse::CSEController::conditional_children` · datafusion-common 55.1.0

```rust
fn conditional_children(node: &Self::Node) -> Option<(Vec<&Self::Node>, Vec<&Self::Node>)>
```

Source: `src/cse.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Splits the children to normal and conditionally evaluated ones or returns `None`
if all are always evaluated.

<a id="op-101b3cfc389fb4d38e37ac75"></a>
## generate_alias

`function` · `datafusion_common::cse::CSEController::generate_alias` · datafusion-common 55.1.0

```rust
fn generate_alias(&self) -> String
```

Source: `src/cse.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0de52dcc30c1ae4f1d0016d6"></a>
## is_ignored

`function` · `datafusion_common::cse::CSEController::is_ignored` · datafusion-common 55.1.0

```rust
fn is_ignored(&self, node: &Self::Node) -> bool
```

Source: `src/cse.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5f8cc84876540be0d1dec36"></a>
## is_valid

`function` · `datafusion_common::cse::CSEController::is_valid` · datafusion-common 55.1.0

```rust
fn is_valid(node: &Self::Node) -> bool
```

Source: `src/cse.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69d87303ee5e20ba480cd244"></a>
## rewrite

`function` · `datafusion_common::cse::CSEController::rewrite` · datafusion-common 55.1.0

```rust
fn rewrite(&mut self, node: &Self::Node, alias: &str) -> Self::Node
```

Source: `src/cse.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c1d7e77734ec75587e74613"></a>
## rewrite_f_down

`function` · `datafusion_common::cse::CSEController::rewrite_f_down` · datafusion-common 55.1.0

```rust
fn rewrite_f_down(&mut self, _node: &Self::Node)
```

Source: `src/cse.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e80c3dcaaa286c40530b2111"></a>
## rewrite_f_up

`function` · `datafusion_common::cse::CSEController::rewrite_f_up` · datafusion-common 55.1.0

```rust
fn rewrite_f_up(&mut self, _node: &Self::Node)
```

Source: `src/cse.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
