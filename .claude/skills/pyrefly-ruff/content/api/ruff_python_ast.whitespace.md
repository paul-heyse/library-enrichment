# `ruff_python_ast::whitespace`

Crate `ruff_python_ast` · 3 public items · structured records in [`model/ruff_python_ast.whitespace.json`](../model/ruff_python_ast.whitespace.json)

## indentation

`function` · `ruff_python_ast::whitespace::indentation`

```rust
fn indentation<'a, T>(source: &'a str, located: &T) -> Option<&'a str> where T: Ranged
```

Extract the leading indentation from a line.

---

## trailing_comment_start_offset

`function` · `ruff_python_ast::whitespace::trailing_comment_start_offset`

```rust
fn trailing_comment_start_offset<T>(located: &T, source: &str) -> Option<ruff_text_size::TextSize> where T: Ranged
```

If a [`Ranged`] has a trailing comment, return the index of the hash.

---

## trailing_lines_end

`function` · `ruff_python_ast::whitespace::trailing_lines_end`

```rust
fn trailing_lines_end(stmt: &Stmt, source: &str) -> ruff_text_size::TextSize
```

Return the end offset at which the empty lines following a statement.

---
