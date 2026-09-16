# `ruff_python_trivia::comments`

Crate `ruff_python_trivia` · 2 public items · structured records in [`model/ruff_python_trivia.comments.json`](../model/ruff_python_trivia.comments.json)

## CommentLinePosition

`enum` · `ruff_python_trivia::comments::CommentLinePosition`

Also reachable as `ruff_python_trivia::CommentLinePosition`

```rust
enum CommentLinePosition
```

**Variants**: `EndOfLine`, `OwnLine`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn for_range(comment_range: TextRange, source_code: &str) -> Self
const fn is_end_of_line(self) -> bool
const fn is_own_line(self) -> bool
```

The position of a comment in the source text.

---

## SuppressionKind

`enum` · `ruff_python_trivia::comments::SuppressionKind`

Also reachable as `ruff_python_trivia::SuppressionKind`

```rust
enum SuppressionKind
```

**Variants**: `Off`, `On`, `Skip`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn from_comment(comment: &str) -> Option<Self>
fn is_suppression_off(slice: &str, position: CommentLinePosition) -> bool
fn is_suppression_on(slice: &str, position: CommentLinePosition) -> bool
```

---
