# `ruff_python_trivia::comment_ranges`

Crate `ruff_python_trivia` · 3 public items · structured records in [`model/ruff_python_trivia.comment_ranges.json`](../model/ruff_python_trivia.comment_ranges.json)

## CommentRanges

`struct` · `ruff_python_trivia::comment_ranges::CommentRanges`

Also reachable as `ruff_python_trivia::CommentRanges`

```rust
struct CommentRanges
```

**Implements**: `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn block_comments(&self, source: &str) -> Vec<TextSize>
fn comments_in_range(&self, range: TextRange) -> &[TextRange]
fn has_comments<T>(&self, node: &T, source: &str) -> bool where T: Ranged
fn intersects(&self, target: TextRange) -> bool
fn is_own_line(offset: TextSize, source: &str) -> bool
fn new(ranges: Vec<TextRange>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Stores the ranges of comments sorted by [`TextRange::start`] in increasing order. No two ranges
are overlapping.

---

## ParenthesizedExpressions

`struct` · `ruff_python_trivia::comment_ranges::ParenthesizedExpressions`

Also reachable as `ruff_python_trivia::ParenthesizedExpressions`

```rust
struct ParenthesizedExpressions
```

**Derives**: Clone, Default

**Methods** (2)

```rust
fn contains(&self, range: TextRange) -> bool
fn new(ranges: FxHashSet<TextRange>) -> Self
```

Index of source ranges enclosed by matching parentheses.

---

## TriviaRanges

`struct` · `ruff_python_trivia::comment_ranges::TriviaRanges`

Also reachable as `ruff_python_trivia::TriviaRanges`

```rust
struct TriviaRanges
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn comments(&self) -> &CommentRanges
fn new(comments: CommentRanges, parenthesized_expressions: ParenthesizedExpressions) -> Self
fn parenthesized(&self) -> &ParenthesizedExpressions
```

Token-derived range indexes shared by comment placement and formatting.

---
