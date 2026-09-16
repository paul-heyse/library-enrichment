# `ruff_python_ast::docstrings`

Crate `ruff_python_ast` · 3 public items · structured records in [`model/ruff_python_ast.docstrings.json`](../model/ruff_python_ast.docstrings.json)

## clean_space

`function` · `ruff_python_ast::docstrings::clean_space`

```rust
fn clean_space(indentation: &str) -> String
```

Replace any non-whitespace characters from an indentation string within a Python docstring.

---

## leading_space

`function` · `ruff_python_ast::docstrings::leading_space`

```rust
fn leading_space(line: &str) -> &str
```

Extract the leading whitespace from a line of text within a Python docstring.

---

## leading_words

`function` · `ruff_python_ast::docstrings::leading_words`

```rust
fn leading_words(line: &str) -> &str
```

Extract the leading words from a line of text within a Python docstring.

---
