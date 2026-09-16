# `ruff_python_ast::token::parentheses`

Crate `ruff_python_ast` · 2 public items · structured records in [`model/ruff_python_ast.token.parentheses.json`](../model/ruff_python_ast.token.parentheses.json)

## parentheses_iterator

`function` · `ruff_python_ast::token::parentheses::parentheses_iterator`

Also reachable as `ruff_python_ast::token::parentheses_iterator`

```rust
fn parentheses_iterator<'a>(expr: ExprRef<'a>, parent: Option<AnyNodeRef<'_>>, tokens: &'a super::Tokens) -> impl Iterator<Item = ruff_text_size::TextRange> + 'a
```

Returns an iterator over the ranges of the optional parentheses surrounding an expression.

E.g. for `((f()))` with `f()` as expression, the iterator returns the ranges (1, 6) and (0, 7).

Note that without a parent the range can be inaccurate, e.g. `f(a)` we falsely return a set of
parentheses around `a` even if the parentheses actually belong to `f`. That is why you should
generally prefer [`parenthesized_range`].

---

## parenthesized_range

`function` · `ruff_python_ast::token::parentheses::parenthesized_range`

Also reachable as `ruff_python_ast::token::parenthesized_range`

```rust
fn parenthesized_range(expr: ExprRef<'_>, parent: AnyNodeRef<'_>, tokens: &super::Tokens) -> Option<ruff_text_size::TextRange>
```

Returns the [`TextRange`] of a given expression including parentheses, if the expression is
parenthesized; or `None`, if the expression is not parenthesized.

---
