# `datafusion_common::cse`

Crate `datafusion-common` · 6 public items · structured records in [`model/datafusion_common.cse.json`](../model/datafusion_common.cse.json)

## FoundCommonNodes

`enum` · `datafusion_common::cse::FoundCommonNodes`

```rust
enum FoundCommonNodes<N>
```

**Variants**: `No`, `Yes`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/datafusion_common.cse.FoundCommonNodes.md).


The result of potentially rewriting a list of [`TreeNode`]s to eliminate common
subtrees.

---

## CSE

`struct` · `datafusion_common::cse::CSE`

```rust
struct CSE<N, C: CSEController<Node = N>>
```

**Methods** (2)

```rust
fn extract_common_nodes(&mut self, nodes_list: Vec<Vec<N>>) -> Result<FoundCommonNodes<N>>
fn new(controller: C) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cse.CSE.md).


The main entry point of Common Subexpression Elimination.

[`CSE`] requires a [`CSEController`], that defines how common subtrees of a particular
[`TreeNode`] tree can be eliminated. The elimination process can be started with the
[`CSE::extract_common_nodes()`] method.

---

## CSEController

`trait` · `datafusion_common::cse::CSEController`

```rust
trait CSEController
```

**Methods** (7)

```rust
fn conditional_children(node: &Self::Node) -> Option<(Vec<&Self::Node>, Vec<&Self::Node>)>
fn generate_alias(&self) -> String
fn is_ignored(&self, node: &Self::Node) -> bool
fn is_valid(node: &Self::Node) -> bool
fn rewrite(&mut self, node: &Self::Node, alias: &str) -> Self::Node
fn rewrite_f_down(&mut self, _node: &Self::Node)
fn rewrite_f_up(&mut self, _node: &Self::Node)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cse.CSEController.md).


The [`TreeNode`] specific definition of elimination.

---

## HashNode

`trait` · `datafusion_common::cse::HashNode`

```rust
trait HashNode
```

**Implementors** (2)

- `alloc::sync::Arc`
- `datafusion_expr::expr::Expr`

**Methods** (1)

```rust
fn hash_node<H: Hasher>(&self, state: &mut H)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cse.HashNode.md).


Hashes the direct content of an [`TreeNode`] without recursing into its children.

This method is useful to incrementally compute hashes, such as in [`CSE`] which builds
a deep hash of a node and its descendants during the bottom-up phase of the first
traversal and so avoid computing the hash of the node and then the hash of its
descendants separately.

If a node doesn't have any children then the value returned by `hash_node()` is
similar to '.hash()`, but not necessarily returns the same value.

---

## NormalizeEq

`trait` · `datafusion_common::cse::NormalizeEq`

```rust
trait NormalizeEq: Eq + Normalizeable
```

**Implementors** (2)

- `datafusion_expr::expr::Expr`
- `datafusion_expr::logical_plan::plan::Subquery`

**Methods** (1)

```rust
fn normalize_eq(&self, other: &Self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cse.NormalizeEq.md).


The `NormalizeEq` trait extends `Eq` and `Normalizeable` to provide a method for comparing
normalized nodes in optimizations like Common Subexpression Elimination (CSE).

The `normalize_eq` method ensures that two nodes that are semantically equivalent (after normalization)
are considered equal in CSE optimization, even if their original forms differ.

This trait allows for equality comparisons between nodes with equivalent semantics, regardless of their
internal representations.

---

## Normalizeable

`trait` · `datafusion_common::cse::Normalizeable`

```rust
trait Normalizeable
```

**Implementors** (2)

- `datafusion_expr::expr::Expr`
- `datafusion_expr::logical_plan::plan::Subquery`

**Methods** (1)

```rust
fn can_normalize(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cse.Normalizeable.md).


The `Normalizeable` trait defines a method to determine whether a node can be normalized.

Normalization is the process of converting a node into a canonical form that can be used
to compare nodes for equality. This is useful in optimizations like Common Subexpression Elimination (CSE),
where semantically equivalent nodes (e.g., `a + b` and `b + a`) should be treated as equal.

---
