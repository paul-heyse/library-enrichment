# `ast_grep_core::source`

Crate `ast-grep-core` · 4 public items · structured records in [`model/ast_grep_core.source.json`](../model/ast_grep_core.source.json)

## Edit

`struct` · `ast_grep_core::source::Edit`

```rust
struct Edit<S: Content>
```

**Fields**: `position`, `deleted_length`, `inserted_text`

**Derives**: Debug

---

## Content

`trait` · `ast_grep_core::source::Content`

Also reachable as `ast_grep_core::replacer::Content`

```rust
trait Content: Sized
```

**Implementors** (1)

- `alloc::string::String`

**Methods** (4)

```rust
fn decode_str(src: &str) -> Cow<'_, [Self::Underlying]>
fn encode_bytes(bytes: &[Self::Underlying]) -> Cow<'_, str>
fn get_char_column(&self, column: usize, offset: usize) -> usize
fn get_range(&self, range: Range<usize>) -> &[Self::Underlying]
```

---

## Doc

`trait` · `ast_grep_core::source::Doc`

Also reachable as `ast_grep_core::Doc`

```rust
trait Doc: Clone + 'static
```

**Implementors** (1)

- `ast_grep_core::tree_sitter::StrDoc`

**Methods** (5)

```rust
fn do_edit(&mut self, edit: &Edit<Self::Source>) -> Result<(), String>
fn get_lang(&self) -> &Self::Lang
fn get_node_text<'a>(&'a self, node: &Self::Node<'a>) -> Cow<'a, str>
fn get_source(&self) -> &Self::Source
fn root_node(&self) -> Self::Node<'_>
```

---

## SgNode

`trait` · `ast_grep_core::source::SgNode`

```rust
trait SgNode<'r>: Clone
```

**Implementors** (1)

- `tree_sitter::Node`

**Methods** (25)

```rust
fn ancestors(&self, _root: Self) -> impl Iterator<Item = Self>
fn child(&self, nth: usize) -> Option<Self>
fn child_by_field_id(&self, field_id: u16) -> Option<Self>
fn children(&self) -> impl ExactSizeIterator<Item = Self>
fn dfs(&self) -> impl Iterator<Item = Self>
fn end_pos(&self) -> Position
fn field(&self, name: &str) -> Option<Self>
fn field_children(&self, field_id: Option<u16>) -> impl Iterator<Item = Self>
fn is_error(&self) -> bool
fn is_extra(&self) -> bool
fn is_leaf(&self) -> bool
fn is_missing(&self) -> bool
fn is_named(&self) -> bool
fn is_named_leaf(&self) -> bool
fn kind(&self) -> Cow<'_, str>
fn kind_id(&self) -> u16
fn named_children(&self) -> impl Iterator<Item = Self>
fn next(&self) -> Option<Self>
fn next_all(&self) -> impl Iterator<Item = Self>
fn node_id(&self) -> usize
fn parent(&self) -> Option<Self>
fn prev(&self) -> Option<Self>
fn prev_all(&self) -> impl Iterator<Item = Self>
fn range(&self) -> std::ops::Range<usize>
fn start_pos(&self) -> Position
```

NOTE: Some method names are the same as tree-sitter's methods.
Fully Qualified Syntax may needed https://stackoverflow.com/a/44445976/2198656

---
