# `ruff_python_ast::find_node`

Crate `ruff_python_ast` · 2 public items · structured records in [`model/ruff_python_ast.find_node.json`](../model/ruff_python_ast.find_node.json)

## covering_node

`function` · `ruff_python_ast::find_node::covering_node`

```rust
fn covering_node(root: AnyNodeRef<'_>, range: ruff_text_size::TextRange) -> CoveringNode<'_>
```

Returns the node with a minimal range that fully contains `range`.

If `range` is empty and falls within a parser *synthesized* node generated during error recovery,
then the first node with the given range is returned.

## Panics
Panics if `range` is not contained within `root`.

---

## CoveringNode

`struct` · `ruff_python_ast::find_node::CoveringNode`

```rust
struct CoveringNode<'a>
```

**Derives**: Debug

**Methods** (6)

```rust
fn ancestors(&self) -> impl DoubleEndedIterator<Item = AnyNodeRef<'a>> + '_
fn find_first(self, f: impl Fn(AnyNodeRef<'a>) -> bool) -> Result<Self, Self>
fn find_last(self, f: impl Fn(AnyNodeRef<'a>) -> bool) -> Result<Self, Self>
fn from_ancestors(ancestors: Vec<AnyNodeRef<'a>>) -> Self
fn node(&self) -> AnyNodeRef<'a>
fn parent(&self) -> Option<AnyNodeRef<'a>>
```

The node with a minimal range that fully contains the search range.

---
