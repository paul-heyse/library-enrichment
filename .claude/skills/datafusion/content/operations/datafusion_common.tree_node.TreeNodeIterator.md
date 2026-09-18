# `datafusion_common::tree_node::TreeNodeIterator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TreeNodeIterator.json).

<a id="op-413e586b149a6c215572b04d"></a>
## TreeNodeIterator

`trait` · `datafusion_common::tree_node::TreeNodeIterator` · datafusion-common 55.1.0

```rust
trait TreeNodeIterator: Iterator
```

Source: `src/tree_node.rs:1151`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Transformation helper to process a sequence of iterable tree nodes that are siblings.

<a id="op-0af09088b971eca664debdeb"></a>
## apply_until_stop

`function` · `datafusion_common::tree_node::TreeNodeIterator::apply_until_stop` · datafusion-common 55.1.0

```rust
fn apply_until_stop<F: FnMut(Self::Item) -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Apples `f` to each item in this iterator

Visits all items in the iterator unless
`f` returns an error or `f` returns `TreeNodeRecursion::Stop`.

# Returns
Error if `f` returns an error or `Ok(TreeNodeRecursion)` from the last invocation
of `f` or `Continue` if the iterator is empty

<a id="op-d405385f916a53e68a77a19d"></a>
## map_until_stop_and_collect

`function` · `datafusion_common::tree_node::TreeNodeIterator::map_until_stop_and_collect` · datafusion-common 55.1.0

```rust
fn map_until_stop_and_collect<F: FnMut(Self::Item) -> Result<Transformed<Self::Item>>>(self, f: F) -> Result<Transformed<Vec<Self::Item>>>
```

Source: `src/tree_node.rs:1177`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Apples `f` to each item in this iterator

Visits all items in the iterator unless
`f` returns an error or `f` returns `TreeNodeRecursion::Stop`.

# Returns
Error if `f` returns an error

Ok(Transformed) such that:
1. `transformed` is true if any return from `f` had transformed true
2. `data` from the last invocation of `f`
3. `tnr` from the last invocation of `f` or `Continue` if the iterator is empty
