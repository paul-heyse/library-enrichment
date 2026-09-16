# `ruff_python_ast::stmt_if`

Crate `ruff_python_ast` · 4 public items · structured records in [`model/ruff_python_ast.stmt_if.json`](../model/ruff_python_ast.stmt_if.json)

## BranchKind

`enum` · `ruff_python_ast::stmt_if::BranchKind`

```rust
enum BranchKind
```

**Variants**: `If`, `Elif`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## elif_else_range

`function` · `ruff_python_ast::stmt_if::elif_else_range`

```rust
fn elif_else_range(clause: &ElifElseClause, contents: &str) -> Option<ruff_text_size::TextRange>
```

Return the `Range` of the first `Elif` or `Else` token in an `If` statement.

---

## if_elif_branches

`function` · `ruff_python_ast::stmt_if::if_elif_branches`

```rust
fn if_elif_branches(stmt_if: &StmtIf) -> impl Iterator<Item = IfElifBranch<'_>>
```

---

## IfElifBranch

`struct` · `ruff_python_ast::stmt_if::IfElifBranch`

```rust
struct IfElifBranch<'a>
```

**Fields**: `kind`, `test`, `body`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Debug

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---
