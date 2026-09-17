# `ast_grep_core::node`

Crate `ast-grep-core` · 3 public items · structured records in [`model/ast_grep_core.node.json`](../model/ast_grep_core.node.json)

## Node

`struct` · `ast_grep_core::node::Node`

Also reachable as `ast_grep_core::Node`

```rust
struct Node<'r, D: Doc>
```

**Implements**: `ast_grep_core::replacer::Replacer`, `core::convert::From`

**Derives**: Clone

**Methods** (45)

```rust
fn after(&self) -> source::Edit<<D as Doc>::Source>
fn ancestors(&self) -> impl Iterator<Item = Node<'r, D>> + '_
fn append(&self) -> source::Edit<<D as Doc>::Source>
fn before(&self) -> source::Edit<<D as Doc>::Source>
fn child(&self, nth: usize) -> Option<Self>
fn child_by_field_id(&self, field_id: u16) -> Option<Self>
fn children(&self) -> impl ExactSizeIterator<Item = Node<'r, D>> + '_
fn dfs<'s>(&'s self) -> impl Iterator<Item = Node<'r, D>> + 's
fn empty(&self) -> Option<source::Edit<<D as Doc>::Source>>
fn end_pos(&self) -> Position
fn field(&self, name: &str) -> Option<Self>
fn field_children(&self, name: &str) -> impl Iterator<Item = Node<'r, D>> + '_
fn find<M: Matcher>(&self, pat: M) -> Option<NodeMatch<'r, D>>
fn find_all<'s, M: Matcher + 's>(&'s self, pat: M) -> impl Iterator<Item = NodeMatch<'r, D>> + 's
fn follows<M: Matcher>(&self, m: M) -> bool
fn get_doc(&self) -> &'r D
fn get_inner_node(&self) -> D::Node<'r>
fn has<M: Matcher>(&self, m: M) -> bool
fn inside<M: Matcher>(&self, m: M) -> bool
fn is_error(&self) -> bool
fn is_extra(&self) -> bool
fn is_leaf(&self) -> bool
fn is_missing(&self) -> bool
fn is_named(&self) -> bool
fn is_named_leaf(&self) -> bool
fn kind(&self) -> Cow<'_, str>
fn kind_id(&self) -> u16
fn lang(&self) -> &'r D::Lang
fn matches<M: Matcher>(&self, m: M) -> bool
fn named_children(&self) -> impl Iterator<Item = Node<'r, D>> + '_
fn next(&self) -> Option<Self>
fn next_all(&self) -> impl Iterator<Item = Node<'r, D>> + '_
fn node_id(&self) -> usize
fn parent(&self) -> Option<Self>
fn precedes<M: Matcher>(&self, m: M) -> bool
fn prepend(&self) -> source::Edit<<D as Doc>::Source>
fn prev(&self) -> Option<Node<'r, D>>
fn prev_all(&self) -> impl Iterator<Item = Node<'r, D>> + '_
fn range(&self) -> std::ops::Range<usize>
fn remove(&self) -> source::Edit<<D as Doc>::Source>
fn replace<M: Matcher, R: Replacer<D>>(&self, matcher: M, replacer: R) -> Option<source::Edit<<D as Doc>::Source>>
fn replace_all<M: Matcher, R: Replacer<StrDoc<L>>>(&self, matcher: M, replacer: R) -> Vec<Edit<String>>
fn root(&self) -> &'r Root<D>
fn start_pos(&self) -> Position
fn text(&self) -> Cow<'r, str>
```

**via `ast_grep_core::replacer::Replacer`**

```rust
fn generate_replacement(&self, _nm: &NodeMatch<'_, D>) -> Underlying<D>
```

**via `core::convert::From`**

```rust
fn from(node_match: NodeMatch<'tree, D>) -> Self
```

'r represents root lifetime

---

## Position

`struct` · `ast_grep_core::node::Position`

Also reachable as `ast_grep_core::Position`

```rust
struct Position
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn byte_point(&self) -> (usize, usize)
fn column<D: Doc>(&self, node: &Node<'_, D>) -> usize
fn line(&self) -> usize
fn new(line: usize, byte_column: usize, byte_offset: usize) -> Self
```

Represents a position in the source code.
The line and column are zero-based, character offsets.
It is different from tree-sitter's position which is zero-based `byte` offsets.
Note, accessing `column` is O(n) operation.

---

## Root

`struct` · `ast_grep_core::node::Root`

```rust
struct Root<D: Doc>
```

Represents [`tree_sitter::Tree`] and owns source string
Note: Root is generic against [`Language`](crate::language::Language)

---
