# `ruff_python_ast::relocate`

Crate `ruff_python_ast` · 1 public items · structured records in [`model/ruff_python_ast.relocate.json`](../model/ruff_python_ast.relocate.json)

## relocate_expr

`function` · `ruff_python_ast::relocate::relocate_expr`

```rust
fn relocate_expr(expr: &mut Expr, range: ruff_text_size::TextRange)
```

Change an expression's location (recursively) to match a desired, fixed
range.

---
