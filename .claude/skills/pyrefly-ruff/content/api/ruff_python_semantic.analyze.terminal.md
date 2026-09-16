# `ruff_python_semantic::analyze::terminal`

Crate `ruff_python_semantic` · 1 public items · structured records in [`model/ruff_python_semantic.analyze.terminal.json`](../model/ruff_python_semantic.analyze.terminal.json)

## Terminal

`enum` · `ruff_python_semantic::analyze::terminal::Terminal`

```rust
enum Terminal
```

**Variants**: `None`, `Implicit`, `Raise`, `RaiseNotImplemented`, `Return`, `RaiseOrReturn`, `ConditionalReturn`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn from_function(function: &ast::StmtFunctionDef, semantic: &SemanticModel<'_>) -> Terminal
fn has_implicit_return(self) -> bool
fn is_always_raise(self) -> bool
```

---
