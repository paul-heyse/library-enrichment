# `ruff_python_ast::identifier`

Crate `ruff_python_ast` · 3 public items · structured records in [`model/ruff_python_ast.identifier.json`](../model/ruff_python_ast.identifier.json)

## else_

`function` · `ruff_python_ast::identifier::else_`

```rust
fn else_(stmt: &Stmt, source: &str) -> Option<ruff_text_size::TextRange>
```

Return the [`TextRange`] of the `else` token in a `For` or `While` statement.

---

## except

`function` · `ruff_python_ast::identifier::except`

```rust
fn except(handler: &ExceptHandler, source: &str) -> ruff_text_size::TextRange
```

Return the [`TextRange`] of the `except` token in an [`ExceptHandler`].

---

## Identifier

`trait` · `ruff_python_ast::identifier::Identifier`

```rust
trait Identifier
```

**Implementors** (6)

- `ruff_python_ast::generated::Stmt`
- `ruff_python_ast::generated::StmtClassDef`
- `ruff_python_ast::generated::StmtFunctionDef`
- `ruff_python_ast::nodes::Alias`
- `ruff_python_ast::nodes::Parameter`
- `ruff_python_ast::nodes::ParameterWithDefault`

**Methods** (1)

```rust
fn identifier(&self) -> TextRange
```

---
