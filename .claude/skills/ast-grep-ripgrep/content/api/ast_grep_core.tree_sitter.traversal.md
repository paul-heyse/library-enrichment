# `ast_grep_core::tree_sitter::traversal`

Crate `ast-grep-core` · 12 public items · structured records in [`model/ast_grep_core.tree_sitter.traversal.json`](../model/ast_grep_core.tree_sitter.traversal.json)

## Level

`struct` · `ast_grep_core::tree_sitter::traversal::Level`

```rust
struct Level<'tree, L: LanguageExt>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (1)

```rust
fn new(node: &Node<'tree, StrDoc<L>>) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Represents a level-order traversal.
It is implemented with [`VecDeque`] since quadratic backtracking is too time consuming.
Though level-order is not used as frequently as other DFS traversals,
traversing a big AST with level-order should be done with caution since it might increase the memory usage.

---

## Post

`struct` · `ast_grep_core::tree_sitter::traversal::Post`

```rust
struct Post<'tree, L: LanguageExt>
```

**Implements**: `ast_grep_core::tree_sitter::traversal::Traversal`, `core::iter::traits::iterator::Iterator`

**Methods** (1)

```rust
fn new(node: &Node<'tree, StrDoc<L>>) -> Self
```

**via `ast_grep_core::tree_sitter::traversal::Traversal`**

```rust
fn calibrate_for_match(&mut self, depth: Option<usize>)
fn get_current_depth(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Represents a post-order traversal

---

## PostOrder

`struct` · `ast_grep_core::tree_sitter::traversal::PostOrder`

```rust
struct PostOrder
```

**Implements**: `ast_grep_core::tree_sitter::traversal::Algorithm`

**via `ast_grep_core::tree_sitter::traversal::Algorithm`**

```rust
fn traverse<L: LanguageExt>(node: Node<'_, StrDoc<L>>) -> Self::Traversal<'_, L>
```

---

## Pre

`struct` · `ast_grep_core::tree_sitter::traversal::Pre`

```rust
struct Pre<'tree, L: LanguageExt>
```

**Implements**: `ast_grep_core::tree_sitter::traversal::Traversal`, `core::iter::traits::iterator::Iterator`

**Methods** (1)

```rust
fn new(node: &Node<'t, StrDoc<L>>) -> Self
```

**via `ast_grep_core::tree_sitter::traversal::Traversal`**

```rust
fn calibrate_for_match(&mut self, depth: Option<usize>)
fn get_current_depth(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

---

## PreOrder

`struct` · `ast_grep_core::tree_sitter::traversal::PreOrder`

```rust
struct PreOrder
```

**Implements**: `ast_grep_core::tree_sitter::traversal::Algorithm`

**via `ast_grep_core::tree_sitter::traversal::Algorithm`**

```rust
fn traverse<L: LanguageExt>(node: Node<'_, StrDoc<L>>) -> Self::Traversal<'_, L>
```

---

## Prune

`struct` · `ast_grep_core::tree_sitter::traversal::Prune`

```rust
struct Prune<'tree, L: LanguageExt>
```

**Methods** (6)

```rust
fn current_node(&self) -> Option<Node<'tree, StrDoc<L>>>
fn current_subtree(&self) -> PruneSubtree<'tree>
fn descend(&mut self)
fn has_left_subtree(&self, subtree: PruneSubtree<'tree>) -> bool
fn new(node: &Node<'tree, StrDoc<L>>) -> Self
fn skip_subtree(&mut self)
```

Pre-order cursor traversal where the caller decides whether to enter children.

This is useful when matching the current node determines whether its whole
subtree can be skipped. Unlike [`Pre`] plus reentrancy calibration, this
traversal does not step into a child before the caller has made that choice.

---

## PruneSubtree

`struct` · `ast_grep_core::tree_sitter::traversal::PruneSubtree`

```rust
struct PruneSubtree<'tree>
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Opaque marker for a subtree in a [`Prune`] traversal.

Callers can store this when visiting a node and later ask whether traversal
has moved past that node's subtree without depending on cursor depth.

---

## TsPre

`struct` · `ast_grep_core::tree_sitter::traversal::TsPre`

Also reachable as `ast_grep_core::tree_sitter::TsPre`

```rust
struct TsPre<'tree>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (1)

```rust
fn new(node: &ts::Node<'tree>) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Represents a pre-order traversal

---

## Visit

`struct` · `ast_grep_core::tree_sitter::traversal::Visit`

```rust
struct Visit<'t, D, T, M>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

---

## Visitor

`struct` · `ast_grep_core::tree_sitter::traversal::Visitor`

Also reachable as `ast_grep_core::tree_sitter::Visitor`

```rust
struct Visitor<M, A = PreOrder>
```

**Methods** (5)

```rust
fn algorithm<Algo>(self) -> Visitor<M, Algo>
fn named_only(self, named_only: bool) -> Self
fn new(matcher: M) -> Visitor<M>
fn reentrant(self, reentrant: bool) -> Self
fn visit<L: LanguageExt>(self, node: Node<'_, StrDoc<L>>) -> Visit<'_, StrDoc<L>, A::Traversal<'_, L>, M> where M: Matcher
```

---

## Algorithm

`trait` · `ast_grep_core::tree_sitter::traversal::Algorithm`

```rust
trait Algorithm
```

**Implementors** (2)

- `ast_grep_core::tree_sitter::traversal::PostOrder`
- `ast_grep_core::tree_sitter::traversal::PreOrder`

**Methods** (1)

```rust
fn traverse<L: LanguageExt>(node: Node<'_, StrDoc<L>>) -> Self::Traversal<'_, L>
```

---

## Traversal

`trait` · `ast_grep_core::tree_sitter::traversal::Traversal`

```rust
trait Traversal<'t, D: Doc + 't>: Iterator<Item = Node<'t, D>>
```

**Implementors** (2)

- `ast_grep_core::tree_sitter::traversal::Post`
- `ast_grep_core::tree_sitter::traversal::Pre`

**Methods** (2)

```rust
fn calibrate_for_match(&mut self, depth: Option<usize>)
fn get_current_depth(&self) -> usize
```

Traversal can iterate over node by using traversal algorithm.
The `next` method should only handle normal, reentrant iteration.
If reentrancy is not desired, traversal should mutate cursor in `calibrate_for_match`.
Visit will maintain the matched node depth so traversal does not need to use extra field.

---
