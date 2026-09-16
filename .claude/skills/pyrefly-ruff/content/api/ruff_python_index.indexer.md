# `ruff_python_index::indexer`

Crate `ruff_python_index` · 1 public items · structured records in [`model/ruff_python_index.indexer.json`](../model/ruff_python_index.indexer.json)

## Indexer

`struct` · `ruff_python_index::indexer::Indexer`

Also reachable as `ruff_python_index::Indexer`

```rust
struct Indexer
```

**Methods** (9)

```rust
const fn comment_ranges(&self) -> &CommentRanges
fn continuation_line_starts(&self) -> &[TextSize]
fn followed_by_multi_statement_line(&self, stmt: &Stmt, source: &str) -> bool
fn from_tokens(tokens: &Tokens, source: &str) -> Self
fn in_multi_statement_line(&self, stmt: &Stmt, source: &str) -> bool
const fn interpolated_string_ranges(&self) -> &InterpolatedStringRanges
const fn multiline_ranges(&self) -> &MultilineRanges
fn preceded_by_continuations(&self, offset: TextSize, source: &str) -> Option<TextSize>
fn preceded_by_multi_statement_line(&self, stmt: &Stmt, source: &str) -> bool
```

---
