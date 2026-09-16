# `ruff_python_ast::traversal`

Crate `ruff_python_ast` · 2 public items · structured records in [`model/ruff_python_ast.traversal.json`](../model/ruff_python_ast.traversal.json)

## suite

`function` · `ruff_python_ast::traversal::suite`

```rust
fn suite<'a>(stmt: impl Into<AnyNodeRef<'a>>, parent: impl Into<AnyNodeRef<'a>>) -> Option<EnclosingSuite<'a>>
```

Given a [`Stmt`] and its parent, return the [`ast::Suite`] that contains the [`Stmt`].

---

## EnclosingSuite

`struct` · `ruff_python_ast::traversal::EnclosingSuite`

```rust
struct EnclosingSuite<'a>
```

**Implements**: `core::ops::deref::Deref`

**Methods** (4)

```rust
fn new(suite: &'a [Stmt], stmt: AnyNodeRef<'a>) -> Option<Self>
fn next_sibling(&self) -> Option<&'a Stmt>
fn next_siblings(&self) -> &'a [Stmt]
fn previous_sibling(&self) -> Option<&'a Stmt>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

---
