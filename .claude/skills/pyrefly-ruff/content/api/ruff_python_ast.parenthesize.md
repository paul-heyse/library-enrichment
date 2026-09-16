# `ruff_python_ast::parenthesize`

Crate `ruff_python_ast` · 1 public items · structured records in [`model/ruff_python_ast.parenthesize.json`](../model/ruff_python_ast.parenthesize.json)

## parenthesized_range

`function` · `ruff_python_ast::parenthesize::parenthesized_range`

```rust
fn parenthesized_range(expr: ExprRef<'_>, parent: AnyNodeRef<'_>, comment_ranges: &ruff_python_trivia::CommentRanges, source: &str) -> Option<ruff_text_size::TextRange>
```

Returns the [`TextRange`] of a given expression including parentheses, if the expression is
parenthesized; or `None`, if the expression is not parenthesized.

Prefer [`crate::token::parenthesized_range`] if you have access to [`crate::token::Tokens`].

---
